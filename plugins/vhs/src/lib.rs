mod ntsc_settings;
mod utils;

use std::sync::mpsc::{Receiver, SyncSender, TryRecvError, TrySendError};
use std::sync::Arc;
use std::thread::{JoinHandle, Thread};
use std::{mem, panic};

use image::flat::View;
use image::{DynamicImage, RgbImage};
use ntsc_settings::NtscAllParams;
use ntscrs::ntsc::NtscEffect;
use ntscrs::yiq_fielding::{YiqOwned, YiqView};
use td_rs_top::*;
use utils::td_top_to_yiq_owned;

/// Struct representing our TOP's state
// #[derive( Default, Clone, Debug)]
pub struct NTSCTop {
    params: NtscAllParams,
    execute_count: usize,
    context: TopContext,
    init: bool,
    ntsc_producer: NtscProducingLoop,
}

struct TopUpload {
    info: UploadInfo,
    buf: TopBuffer,
}

impl TopUpload {
    fn get_pixel_size_bytes(pix_format: &PixelFormat) -> Option<u8> {
        let size = match pix_format {
            PixelFormat::BGRA8Fixed => 4,
            PixelFormat::RGBA8Fixed => 4,
            PixelFormat::RGBA16Fixed => 8,
            PixelFormat::RGBA16Float => 8,
            PixelFormat::RGBA32Float => 16,

            PixelFormat::RG8Fixed => 2,
            PixelFormat::RG16Fixed => 4,
            PixelFormat::RG16Float => 4,
            PixelFormat::RG32Float => 8,

            PixelFormat::A8Fixed => 1,
            PixelFormat::A16Fixed => 2,
            PixelFormat::A16Float => 2,
            PixelFormat::A32Float => 4,

            PixelFormat::RGB10A2Fixed => 4,
            PixelFormat::RGB11Float => 4,
            _ => return None,
        };

        Some(size)
    }

    fn new_internal(info: UploadInfo, context: &mut TopContext) -> Self {
        let pix_size =
            Self::get_pixel_size_bytes(&info.texture_desc.pixel_format).unwrap() as usize;

        let (width, height) = (info.texture_desc.width, info.texture_desc.height);
        let data_len = width * height * pix_size;

        let buf = context.create_output_buffer(data_len, TopBufferFlags::None);
        Self { info, buf }
    }

    fn new_from_texdesc(texture_desc: TextureDesc, context: &mut TopContext) -> Self {
        let info = UploadInfo {
            buffer_offset: 0,
            texture_desc,
            first_pixel: FirstPixel::TopLeft,
            color_buffer_index: 0,
        };

        Self::new_internal(info, context)
    }

    fn new_simple(
        width: usize,
        height: usize,
        pixel_format: PixelFormat,
        context: &mut TopContext,
    ) -> Self {
        Self::new_from_texdesc(
            TextureDesc {
                width,
                height,
                pixel_format,
                ..Default::default()
            },
            context,
        )
    }

    fn upload(&mut self, output: &mut TopOutput) {
        output.upload_buffer(&mut self.buf, &self.info);
    }

    fn write_from_slice(&mut self, slice: &[u8]) {
        assert!(slice.len() == self.buf.size(), "Copied slice must be equal");
        self.buf.data_mut::<u8>().copy_from_slice(slice);
    }
}

impl TopNew for NTSCTop {
    fn new(_info: NodeInfo, context: TopContext) -> Self {
        Self {
            params: Default::default(),
            execute_count: 0,
            context,
            init: false,
            ntsc_producer: NtscProducingLoop::new(),
        }
    }
}

impl OpInfo for NTSCTop {
    const OPERATOR_LABEL: &'static str = "Composite video fx";
    const OPERATOR_TYPE: &'static str = "Vhseffect";
    // const OPERATOR_ICON: &'static str = "CPM";
    const MAX_INPUTS: usize = 1;
    const MIN_INPUTS: usize = 1;
}

impl TopInfo for NTSCTop {
    const EXECUTE_MODE: ExecuteMode = ExecuteMode::Cpu;
}

impl Op for NTSCTop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn pulse_pressed(&mut self, name: &str) {
        if name == "Reset" {}
    }
}

impl Top for NTSCTop {
    fn general_info(&self, _input: &OperatorInputs<TopInput>) -> TopGeneralInfo {
        TopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: true,
            input_size_index: 0,
        }
    }

    fn execute(&mut self, mut output: TopOutput, input: &OperatorInputs<TopInput>) {
        self.execute_count += 1;

        let restart_producer = match self.ntsc_producer.check_thread_status() {
            ThreadStatus::Alive => false,
            ThreadStatus::Dead => true,
        };

        if restart_producer {
            let old_producer = mem::replace(&mut self.ntsc_producer, NtscProducingLoop::new());
            let _ = old_producer.join_thread();
            self.init = false;
        }

        if self.init == false {
            self.params = Default::default();
            self.init = true;
        }

        let vid_input: &TopInput = match input.input(0) {
            Some(i) => i,
            None => {
                self.set_error("Not enough inputs");
                return;
            }
        };
        self.set_warning("");
        self.set_error("");
        let mut downloaded = vid_input.download_texture(DownloadOptions::default());
        let tex_desc = downloaded.texture_desc();

        let result: DynamicImage = if let Some(result) = self.ntsc_producer.apply_effect(
            downloaded,
            self.params.clone().into(),
            self.execute_count,
        ) {
            result
        } else {
            self.set_error("effect failed");
            return;
        };

        let immg = result.into_rgba8();

        let (width, height) = (immg.width() as usize, immg.height() as usize);

        let raw_data = immg.into_raw();

        let mut upload = TopUpload::new_from_texdesc(
            TextureDesc {
                width,
                height,
                pixel_format: PixelFormat::RGBA8Fixed,
                ..tex_desc
            },
            &mut self.context,
        );

        upload.write_from_slice(&raw_data);
        upload.upload(&mut output);
    }
}

struct EffectInfo(YiqOwned, NtscEffect, usize);

struct NtscProducingLoop {
    produce_loop: JoinHandle<()>,
    rx: Receiver<YiqOwned>,
    trigger_tx: SyncSender<EffectInfo>,
}

#[derive(Debug, Clone, Copy)]
enum ThreadStatus {
    Alive,
    Dead,
}

#[derive(Debug, Clone, Copy)]
enum ThreadExitStatus {
    Success,
    Failure,
}

impl NtscProducingLoop {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(3);
        let (trigger_tx, trigger_rx) = std::sync::mpsc::sync_channel(1);

        let produce_loop = Self::produce_loop(tx, trigger_rx);

        NtscProducingLoop {
            rx,
            trigger_tx,
            produce_loop,
        }
    }

    fn produce_loop(tx: SyncSender<YiqOwned>, trigger_rx: Receiver<EffectInfo>) -> JoinHandle<()> {
        let produce_loop = std::thread::spawn(move || {
            loop {
                // Wait for a frame to be requested
                let (mut img_yiq, effect, frame_no) = {
                    let info = trigger_rx.recv().unwrap();
                    (info.0, info.1, info.2)
                };

                let img_yiq = {
                    let mut view = YiqView::from(&mut img_yiq);
                    effect.apply_effect_to_yiq(&mut view, frame_no);
                    img_yiq
                };
                tx.send(img_yiq).unwrap();
            }
        });

        produce_loop
    }

    fn join_thread(self) -> ThreadExitStatus {
        let error_msg = match self.produce_loop.join() {
            Ok(_) => None,
            Err(err) => {
                let msg = if let Some(msg) = err.downcast_ref::<&'static str>() {
                    msg.to_string()
                } else if let Some(msg) = err.downcast_ref::<String>() {
                    msg.clone()
                } else {
                    format!("?{:?}", err)
                };
                Some(msg)
            }
        };

        if let Some(_) = error_msg {
            ThreadExitStatus::Failure
        } else {
            ThreadExitStatus::Success
        }
    }

    fn check_thread_status(&self) -> ThreadStatus {
        if self.produce_loop.is_finished() {
            ThreadStatus::Dead
        } else {
            ThreadStatus::Alive
        }
    }

    fn apply_effect(
        &self,
        downloaded: TopDownloadResult,
        effect: NtscEffect,
        frame_no: usize,
    ) -> Option<DynamicImage> {
        //effect goes here
        // let effect: NtscEffect = self.params.clone().into();

        let maybe_yiq = td_top_to_yiq_owned(downloaded, effect.use_field.to_yiq_field(frame_no));
        let img_yiq = if let Some(yiq_own) = maybe_yiq {
            yiq_own
        } else {
            // self.set_error("Unsupported pixel format");
            return None;
        };

        let mut applied_effect = if let Some(image) = self.get_image(img_yiq, effect, frame_no) {
            image
        } else {
            return None;
        };

        let view = YiqView::from(&mut applied_effect);

        let result_img = RgbImage::from(&view);

        Some(DynamicImage::ImageRgb8(result_img))
    }

    fn get_image(
        &self,
        img_yiq: YiqOwned,
        effect: NtscEffect,
        frame_no: usize,
    ) -> Option<YiqOwned> {
        let to_send = EffectInfo(img_yiq, effect, frame_no);

        match self.trigger_tx.try_send(to_send) {
            Ok(_) => {}
            Err(err) => {
                match err {
                    TrySendError::Full(_) => {
                        // would block, so just return
                    }
                    TrySendError::Disconnected(_) => {
                        panic!("Producer thread disconnected!")
                    }
                }
            }
        };

        match self.rx.try_recv() {
            Ok(img) => {
                return Some(img);
            }
            Err(err) => match err {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => {
                    panic!("Producer thread disconnected!")
                }
            },
        };

        None
    }
}
top_plugin!(NTSCTop);
