#![feature(f16)]

mod ntsc_settings;
mod settingconverter;

use image::{DynamicImage, RgbImage};
use ntsc_settings::NtscAllParams;
use ntscrs::ntsc::{NtscEffect};
use ntscrs::yiq_fielding::{Bgrx8, Rgbx32f, Rgbx8, YiqField, YiqOwned, YiqView};

use std::mem::size_of;
use td_rs_top::*;

/// Struct representing our SOP's state
pub struct NTSCTop {
    params: NtscAllParams,
    execute_count: usize,
    context: TopContext,
    init: bool,
}

impl TopNew for NTSCTop {
    fn new(_info: NodeInfo, context: TopContext) -> Self {
        Self {
            params: Default::default(),
            execute_count: 0,
            context,
            init: false,
        }
    }
}

impl OpInfo for NTSCTop {
    const OPERATOR_LABEL: &'static str = "CPU Mem Sample";
    const OPERATOR_TYPE: &'static str = "Cpumemsample";
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

// fn top_dl_to_image(top_dl: &mut TopDownloadResult) -> Option<RgbImage> {

//     // match tex_desc.tex_dim {
//     //     TexDim::EInvalid => todo!(),
//     //     TexDim::E2D => todo!(),
//     //     TexDim::E2DArray => todo!(),
//     //     TexDim::E3D => todo!(),
//     //     TexDim::ECube => todo!(),
//     // }

//     // let ( bpc, n_chan , order_rgb ) = match format {
//     //     PixelFormat::Invalid => todo!(),
//     //     PixelFormat::BGRA8Fixed => (size_of::<u8>(),4, false),
//     //     PixelFormat::RGBA8Fixed => (size_of::<u8>(),4, true),
//     //     PixelFormat::RGBA16Fixed => (size_of::<u16>(),4, true),
//     //     PixelFormat::RGBA16Float => (size_of::<f16>(),4, true),
//     //     PixelFormat::RGBA32Float => (size_of::<f32>(),4, true),,

//     //     PixelFormat::Mono8Fixed => (size_of::<u8>(),1, true),
//     //     PixelFormat::Mono16Fixed => (size_of::<u16>(),1, true),
//     //     PixelFormat::Mono16Float => (size_of::<f16>(),1, true),
//     //     PixelFormat::Mono32Float => (size_of::<f32>(),1, true),

//     //     PixelFormat::RG8Fixed => (size_of::<u8>(),2, true),
//     //     PixelFormat::RG16Fixed => (size_of::<f16>(),2, true),
//     //     PixelFormat::RG16Float => (size_of::<f16>(),2, true),
//     //     PixelFormat::RG32Float => (size_of::<f32>(),2, true),
//     //     PixelFormat::A8Fixed => (size_of::<u8>(),1, true),
//     //     PixelFormat::A16Fixed => todo!(),
//     //     PixelFormat::A16Float => todo!(),
//     //     PixelFormat::A32Float => todo!(),
//     //     PixelFormat::MonoA8Fixed => todo!(),
//     //     PixelFormat::MonoA16Fixed => todo!(),
//     //     PixelFormat::MonoA16Float => todo!(),
//     //     PixelFormat::MonoA32Float => todo!(),
//     //     PixelFormat::SBGRA8Fixed => todo!(),
//     //     PixelFormat::SRGBA8Fixed => todo!(),
//     //     PixelFormat::RGB10A2Fixed => todo!(),
//     //     PixelFormat::RGB11Float => todo!(),
//     // };

//     let chan_size = size_of::<f32>() as u32;
//     let n_chan = 4;
//     let expected_size = width as usize * height as usize * n_chan as usize * chan_size as usize;

//     assert_eq!(
//         expected_size, buf_len_bytes,
//         "Testing if expected size == bug len"
//     );

//     let decoded_img = Rgba32FImage::from_raw(width, height, top_dl.data().to_vec()).unwrap();
//     Some(DynamicImage::ImageRgba32F(decoded_img).into_rgb8())
// }

fn td_top_to_yiq_owned(mut top_dl: TopDownloadResult, field: YiqField) -> Option<YiqOwned> {
    let tex_desc = top_dl.texture_desc();

    let width = tex_desc.width;
    let height = tex_desc.height;

    let num_rows = field.num_image_rows(height);

    let num_pixels = width * num_rows;

    let mut data = vec![0f32; num_pixels * 3];
    let (y, iq) = data.split_at_mut(num_pixels);
    let (i, q) = iq.split_at_mut(num_pixels);

    let view = YiqView {
        y,
        i,
        q,
        dimensions: (width, height),
        field,
    };

    let res = match tex_desc.pixel_format {
        PixelFormat::BGRA8Fixed => Some(YiqOwned::from_strided_buffer::<Bgrx8>(
            top_dl.data(),
            width * 4 * size_of::<u8>(),
            width,
            height,
            field,
        )),
        PixelFormat::RGBA8Fixed => Some(YiqOwned::from_strided_buffer::<Rgbx8>(
            top_dl.data(),
            width * 4 * size_of::<u8>(),
            width,
            height,
            field,
        )),
        PixelFormat::RGBA32Float => Some(YiqOwned::from_strided_buffer::<Rgbx32f>(
            top_dl.data(),
            width * 4 * size_of::<f32>(),
            width,
            height,
            field,
        )),
        _ => None, // PixelFormat::RGBA16Fixed => todo!(),
                   // PixelFormat::RGBA16Float => todo!(),
                   // PixelFormat::Mono8Fixed => todo!(),
                   // PixelFormat::Mono16Fixed => todo!(),
                   // PixelFormat::Mono16Float => todo!(),
                   // PixelFormat::Mono32Float => todo!(),
                   // PixelFormat::RG8Fixed => todo!(),
                   // PixelFormat::RG16Fixed => todo!(),
                   // PixelFormat::RG16Float => todo!(),
                   // PixelFormat::RG32Float => todo!(),
                   // PixelFormat::A8Fixed => todo!(),
                   // PixelFormat::A16Fixed => todo!(),
                   // PixelFormat::A16Float => todo!(),
                   // PixelFormat::A32Float => todo!(),
                   // PixelFormat::MonoA8Fixed => todo!(),
                   // PixelFormat::MonoA16Fixed => todo!(),
                   // PixelFormat::MonoA16Float => todo!(),
                   // PixelFormat::MonoA32Float => todo!(),
                   // PixelFormat::SBGRA8Fixed => todo!(),
                   // PixelFormat::SRGBA8Fixed => todo!(),
                   // PixelFormat::RGB10A2Fixed => todo!(),
                   // PixelFormat::RGB11Float => todo!(),
    };

    res
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

        let vid_input: &TopInput = match input.input(0) {
            Some(i) => i,
            None => {
                self.set_error("Not enough inputs");
                return;
            }
        };
        self.set_warning("");
        let mut downloaded = vid_input.download_texture(DownloadOptions::default());
        let tex_desc = downloaded.texture_desc();
        //effect goes here
        let effect: NtscEffect = self.params.clone().into();

        let immg = {
            if let Some(mut yiq_own) = td_top_to_yiq_owned(
                downloaded,
                effect.use_field.to_yiq_field(self.execute_count),
            ) {
                let mut view = YiqView::from(&mut yiq_own);

                effect.apply_effect_to_yiq(&mut view, self.execute_count);
                RgbImage::from(&view)
            } else {
                self.set_error("Unsupported pixel format");
                return;
            }
        };
        self.set_error("");

        // match downloaded.texture_desc().pixel_format {
        //     PixelFormat::RGBA32Float => {
        //         let img = top_dl_to_image(&mut downloaded).unwrap();
        //         let yyy = yyzz.apply_effect(&img, 0);
        //         ()
        //     }
        //     _ => (),
        // };

        //effect ends here

        // let out_size =  immg.
        let immg = DynamicImage::ImageRgb8(immg).into_rgba8();

        let (width, height) = (immg.width() as usize, immg.height() as usize);

        let raw_data = immg.into_raw();
        let mut buf = self
            .context
            .create_output_buffer(raw_data.len(), TopBufferFlags::None);

        buf.data_mut::<u8>().copy_from_slice(&raw_data);

        let info = UploadInfo {
            buffer_offset: 0,
            texture_desc: TextureDesc {
                width: width,
                height: height,
                pixel_format: PixelFormat::RGBA8Fixed,
                ..tex_desc
            },
            first_pixel: FirstPixel::TopLeft,
            color_buffer_index: 0,
        };
        output.upload_buffer(&mut buf, &info);
    }
}

top_plugin!(NTSCTop);
