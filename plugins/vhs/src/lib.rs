mod ntsc_settings;
mod utils;

use image::{DynamicImage, RgbImage};
use ntsc_settings::NtscAllParams;
use ntscrs::ntsc::NtscEffect;
use ntscrs::yiq_fielding::YiqView;
use td_rs_top::*;
use utils::td_top_to_yiq_owned;

/// Struct representing our TOP's state
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
        let mut downloaded = vid_input.download_texture(DownloadOptions::default());
        let tex_desc = downloaded.texture_desc();
        //effect goes here
        let effect: NtscEffect = self.params.clone().into();

        let result_img = {
            let mut immg = {
                if let Some(mut yiq_own) = td_top_to_yiq_owned(
                    downloaded,
                    effect.use_field.to_yiq_field(self.execute_count),
                ) {
                    let mut view = YiqView::from(&mut yiq_own);

                    effect.apply_effect_to_yiq(&mut view, self.execute_count);
                    // RgbImage::from(&view)
                    yiq_own
                } else {
                    self.set_error("Unsupported pixel format");
                    return;
                }
            };
            self.set_error("");
            let immg = RgbImage::from(&YiqView::from(&mut immg));
            immg
        };

        //effect ends here

        // let out_size =  immg.
        let immg = DynamicImage::ImageRgb8(result_img).into_rgba8();

        let (width, height) = (immg.width() as usize, immg.height() as usize);

        let raw_data = immg.into_raw();
        let mut buf = self
            .context
            .create_output_buffer(raw_data.len(), TopBufferFlags::None);

        buf.data_mut::<u8>().copy_from_slice(&raw_data);

        let info = UploadInfo {
            buffer_offset: 0,
            texture_desc: TextureDesc {
                width,
                height,
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
