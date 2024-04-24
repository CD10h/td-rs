use td_rs_derive::{Param, Params};
use td_rs_top::{MenuParam, Param};

use ntscrs::ntsc::{
    ChromaDemodulationFilter, ChromaLowpass, ChromaNoiseSettings, FilterType,
    HeadSwitchingSettings, LumaLowpass, NtscEffect, PhaseShift, RingingSettings,
    TrackingNoiseSettings, UseField, VHSEdgeWaveSettings, VHSSettings, VHSSharpenSettings,
    VHSTapeSpeed,
};
use td_rs_top::{OperatorParams, ParamInputs, ParamOptions, ParameterManager, StringParameter};

#[derive(Param, Default, Clone, Debug)]
enum NtscUseField {
    #[default]
    Alternating,
    UpperOnly,
    LowerOnly,
    InterleavedUpper,
    InterleavedLower,
    Both,
}

impl From<UseField> for NtscUseField {
    fn from(value: UseField) -> Self {
        match value {
            UseField::Alternating => Self::Alternating,
            UseField::Upper => Self::UpperOnly,
            UseField::Lower => Self::LowerOnly,
            UseField::Both => Self::Both,
            UseField::InterleavedUpper => Self::InterleavedUpper,
            UseField::InterleavedLower => Self::InterleavedLower,
        }
    }
}

impl From<NtscUseField> for UseField {
    fn from(value: NtscUseField) -> Self {
        match value {
            NtscUseField::Alternating => Self::Alternating,
            NtscUseField::UpperOnly => Self::Upper,
            NtscUseField::LowerOnly => Self::Lower,
            NtscUseField::InterleavedUpper => Self::InterleavedUpper,
            NtscUseField::InterleavedLower => Self::InterleavedLower,
            NtscUseField::Both => Self::Both,
        }
    }
}

#[derive(Param, Default, Clone, Debug)]
enum NtscLowPassType {
    #[default]
    ConstantK,
    Butterworth,
}

#[derive(Param, Default, Clone, Debug)]
enum NtscInputLumaFilter {
    #[default]
    Notch,
    Box,
    None,
}

#[derive(Param, Default, Clone, Debug)]
enum NtscChromaLowPass {
    #[default]
    Full,
    Light,
    None,
}

impl From<NtscChromaLowPass> for ChromaLowpass {
    fn from(value: NtscChromaLowPass) -> Self {
        match value {
            NtscChromaLowPass::Full => Self::Full,
            NtscChromaLowPass::Light => Self::Light,
            NtscChromaLowPass::None => Self::None,
        }
    }
}
impl From<ChromaLowpass> for NtscChromaLowPass {
    fn from(value: ChromaLowpass) -> Self {
        match value {
            ChromaLowpass::None => Self::None,
            ChromaLowpass::Light => Self::Light,
            ChromaLowpass::Full => Self::Full,
        }
    }
}

#[derive(Param, Default, Clone, Debug)]
enum NtscScanlinePhaseShift {
    #[default]
    Degrees0,
    Degrees90,
    Degrees180,
    Degrees270,
}

#[derive(Param, Default, Clone, Debug)]
enum NtsChromaDemod {
    #[default]
    Box,
    Notch,
    OneLineComb,
    TwoLineComb,
}

impl From<ChromaDemodulationFilter> for NtsChromaDemod {
    fn from(value: ChromaDemodulationFilter) -> Self {
        match value {
            ChromaDemodulationFilter::Box => Self::Box,
            ChromaDemodulationFilter::Notch => Self::Notch,
            ChromaDemodulationFilter::OneLineComb => Self::OneLineComb,
            ChromaDemodulationFilter::TwoLineComb => Self::TwoLineComb,
        }
    }
}

impl From<NtsChromaDemod> for ChromaDemodulationFilter {
    fn from(value: NtsChromaDemod) -> Self {
        match value {
            NtsChromaDemod::Box => Self::Box,
            NtsChromaDemod::Notch => Self::Notch,
            NtsChromaDemod::OneLineComb => Self::OneLineComb,
            NtsChromaDemod::TwoLineComb => Self::TwoLineComb,
        }
    }
}

#[derive(Param, Default, Clone, Debug)]
enum NtscVhsTapeSpeed {
    #[default]
    StandardPlay,
    LongPlay,
    ExtendedPlay,
    None,
}

impl From<Option<VHSTapeSpeed>> for NtscVhsTapeSpeed {
    fn from(value: Option<VHSTapeSpeed>) -> Self {
        match value {
            Some(ts) => match ts {
                VHSTapeSpeed::SP => Self::StandardPlay,
                VHSTapeSpeed::LP => Self::LongPlay,
                VHSTapeSpeed::EP => Self::ExtendedPlay,
            },
            None => Self::None,
        }
    }
}

impl From<NtscVhsTapeSpeed> for Option<VHSTapeSpeed> {
    fn from(value: NtscVhsTapeSpeed) -> Self {
        match value {
            NtscVhsTapeSpeed::StandardPlay => Some(VHSTapeSpeed::SP),
            NtscVhsTapeSpeed::LongPlay => Some(VHSTapeSpeed::LP),
            NtscVhsTapeSpeed::ExtendedPlay => Some(VHSTapeSpeed::EP),
            NtscVhsTapeSpeed::None => None,
        }
    }
}

impl From<NtscEffect> for NtscAllParams {
    fn from(value: NtscEffect) -> Self {
        let (
            head_switching_enable,
            head_switching_height,
            head_switching_offset,
            head_switching_horizontal_shift,
        ) = if let Some(hs) = value.head_switching {
            (true, hs.height, hs.offset, hs.horiz_shift)
        } else {
            (
                false,
                Default::default(),
                Default::default(),
                Default::default(),
            )
        };

        let (
            tracking_noise_enable,
            tracking_noise_height,
            tracking_noise_wave_intensity,
            tracking_noise_snow_intensity,
            tracking_noise_snow_anisotropy,
            tracking_noise_noise_intensity,
        ) = if let Some(tn) = value.tracking_noise {
            (
                true,
                tn.height,
                tn.wave_intensity,
                tn.snow_anisotropy,
                tn.snow_anisotropy,
                tn.noise_intensity,
            )
        } else {
            (
                false,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            )
        };

        let (ringing_enable, ringing_frequency, ringing_power, ringing_scale) =
            if let Some(ring) = value.ringing {
                (true, ring.frequency, ring.power, ring.intensity)
            } else {
                (
                    false,
                    Default::default(),
                    Default::default(),
                    Default::default(),
                )
            };

        let (
            chroma_noise_enable,
            chroma_noise_intensity,
            chroma_noise_frequency,
            chroma_noise_detail,
        ) = if let Some(cn) = value.chroma_noise {
            (true, cn.intensity, cn.frequency, cn.detail)
        } else {
            (
                false,
                Default::default(),
                Default::default(),
                Default::default(),
            )
        };

        let (
            vhs_enable,
            vhs_tape_speed,
            vhs_chroma_loss,
            vhs_sharpen_enable,
            vhs_sharpen_intensity,
            vhs_sharpen_frequency,
            vhs_edge_wav_enable,
            vhs_edge_wave_intensity,
            vhs_edge_wave_speed,
            vhs_edge_wave_frequency,
            vhs_edge_wave_detail,
        ) = if let Some(vhs) = value.vhs_settings {
            let chroma_loss = vhs.chroma_loss;
            let tape_speed = vhs.tape_speed.into();

            let (sharpen_en, sharpen_intense, shapren_freq) = if let Some(sharpen) = vhs.sharpen {
                (true, sharpen.intensity, sharpen.frequency)
            } else {
                (false, Default::default(), Default::default())
            };

            let (ew_en, ew_int, ew_speed, ew_freq, ew_detail) =
                if let Some(edgewave) = vhs.edge_wave {
                    (
                        true,
                        edgewave.intensity,
                        edgewave.speed,
                        edgewave.frequency,
                        edgewave.detail,
                    )
                } else {
                    (
                        false,
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    )
                };

            (
                true,
                tape_speed,
                chroma_loss,
                sharpen_en,
                sharpen_intense,
                shapren_freq,
                ew_en,
                ew_int,
                ew_speed,
                ew_freq,
                ew_detail,
            )
        } else {
            (
                false,
                Default::default(),
                Default::default(),
                false,
                Default::default(),
                Default::default(),
                false,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            )
        };

        Self {
            random_seed: value.random_seed,
            bandwidth_scale: value.bandwidth_scale,
            use_field: value.use_field.into(),
            low_pass_type: match value.filter_type {
                FilterType::ConstantK => NtscLowPassType::ConstantK,
                FilterType::Butterworth => NtscLowPassType::Butterworth,
            },
            input_luma_filter: match value.input_luma_filter {
                LumaLowpass::None => NtscInputLumaFilter::None,
                LumaLowpass::Box => NtscInputLumaFilter::Box,
                LumaLowpass::Notch => NtscInputLumaFilter::Notch,
            },
            input_chroma_lowpass: value.chroma_lowpass_in.into(),
            composite_preemphasis: value.composite_preemphasis,
            composite_noise: value.composite_noise_intensity,
            snow: value.snow_intensity,
            snow_anisotropy: value.snow_anisotropy,
            scanline_phase_shift: match value.video_scanline_phase_shift {
                PhaseShift::Degrees0 => NtscScanlinePhaseShift::Degrees0,
                PhaseShift::Degrees90 => NtscScanlinePhaseShift::Degrees90,
                PhaseShift::Degrees180 => NtscScanlinePhaseShift::Degrees180,
                PhaseShift::Degrees270 => NtscScanlinePhaseShift::Degrees270,
            },
            scanline_phase_shift_offset: value.video_scanline_phase_shift_offset,
            chroma_demod_filter: value.chroma_demodulation.into(),
            luma_smear: value.luma_smear,
            head_switching_enable: head_switching_enable,
            head_switching_height: head_switching_height,
            head_switching_offset: head_switching_offset,
            head_switching_horizontal_shift,
            tracking_noise_enable: tracking_noise_enable,
            tracking_noise_height: tracking_noise_height,
            tracking_noise_wave_intensity: tracking_noise_wave_intensity,
            tracking_noise_snow_intensity: tracking_noise_snow_intensity,
            tracking_noise_snow_anisotropy: tracking_noise_snow_anisotropy,
            tracking_noise_noise_intensity: tracking_noise_noise_intensity,
            ringing_enable: ringing_enable,
            ringing_frequency: ringing_frequency,
            ringing_power: ringing_power,
            ringing_scale: ringing_scale,
            chroma_noise_enable: chroma_noise_enable,
            chroma_noise_intensity: chroma_noise_intensity,
            chroma_noise_frequency: chroma_noise_frequency,
            chroma_noise_detail: chroma_noise_detail,
            chroma_phase_error: value.chroma_phase_error,
            chroma_phase_noise: value.chroma_phase_noise_intensity,
            chroma_delay_horizontal: value.chroma_delay.0,
            chroma_delay_vertical: value.chroma_delay.1,
            vhs_enable: vhs_enable,
            vhs_tape_speed: vhs_tape_speed,
            vhs_chroma_loss: vhs_chroma_loss,
            vhs_sharpen_enable: vhs_sharpen_enable,
            vhs_sharpen_intensity: vhs_sharpen_intensity,
            vhs_sharpen_frequency: vhs_sharpen_frequency,
            vhs_edge_wav_enable: vhs_edge_wav_enable,
            vhs_edge_wave_intensity: vhs_edge_wave_intensity,
            vhs_edge_wave_speed: vhs_edge_wave_speed,
            vhs_edge_wave_frequency: vhs_edge_wave_frequency,
            vhs_edge_wave_detail: vhs_edge_wave_detail,
            vertically_blend_chroma: value.chroma_vert_blend,
            chroma_low_pass_out: value.chroma_lowpass_out.into(),
        }
    }
}

impl From<NtscAllParams> for NtscEffect {
    fn from(value: NtscAllParams) -> Self {
        let mut fx = NtscEffect::default();

        fx.random_seed = value.random_seed;
        fx.use_field = value.use_field.into();
        fx.filter_type = match value.low_pass_type {
            NtscLowPassType::ConstantK => FilterType::ConstantK,
            NtscLowPassType::Butterworth => FilterType::Butterworth,
        };
        fx.input_luma_filter = match value.input_luma_filter {
            NtscInputLumaFilter::Notch => LumaLowpass::Notch,
            NtscInputLumaFilter::Box => LumaLowpass::Box,
            NtscInputLumaFilter::None => LumaLowpass::None,
        };
        fx.chroma_lowpass_in = value.input_chroma_lowpass.into();
        fx.chroma_demodulation = value.chroma_demod_filter.into();
        fx.luma_smear = value.luma_smear;
        fx.composite_preemphasis = value.composite_preemphasis;
        fx.video_scanline_phase_shift = match value.scanline_phase_shift {
            NtscScanlinePhaseShift::Degrees0 => PhaseShift::Degrees0,
            NtscScanlinePhaseShift::Degrees90 => PhaseShift::Degrees90,
            NtscScanlinePhaseShift::Degrees180 => PhaseShift::Degrees180,
            NtscScanlinePhaseShift::Degrees270 => PhaseShift::Degrees270,
        };
        fx.video_scanline_phase_shift_offset = value.scanline_phase_shift_offset;
        fx.head_switching = if value.head_switching_enable {
            Some(HeadSwitchingSettings {
                height: value.head_switching_height,
                offset: value.head_switching_offset,
                horiz_shift: value.head_switching_horizontal_shift,
            })
        } else {
            None
        };
        fx.tracking_noise = if value.tracking_noise_enable {
            Some(TrackingNoiseSettings {
                height: value.tracking_noise_height,
                wave_intensity: value.tracking_noise_wave_intensity,
                snow_intensity: value.tracking_noise_snow_intensity,
                snow_anisotropy: value.tracking_noise_snow_anisotropy,
                noise_intensity: value.tracking_noise_noise_intensity,
            })
        } else {
            None
        };
        fx.composite_noise_intensity = value.composite_noise;
        fx.ringing = if value.ringing_enable {
            Some(RingingSettings {
                frequency: value.ringing_frequency,
                power: value.ringing_power,
                intensity: value.ringing_scale,
            })
        } else {
            None
        };
        fx.chroma_noise = if value.chroma_noise_enable {
            Some(ChromaNoiseSettings {
                frequency: value.chroma_noise_frequency,
                intensity: value.chroma_noise_intensity,
                detail: value.chroma_noise_detail,
            })
        } else {
            None
        };
        fx.snow_intensity = value.snow;
        fx.snow_anisotropy = value.snow_anisotropy;
        fx.chroma_phase_noise_intensity = value.chroma_phase_noise;
        fx.chroma_phase_error = value.chroma_phase_error;
        fx.chroma_delay = (value.chroma_delay_horizontal, value.chroma_delay_vertical);
        fx.vhs_settings = if value.vhs_enable {
            Some(VHSSettings {
                tape_speed: value.vhs_tape_speed.into(),
                chroma_loss: value.vhs_chroma_loss,
                sharpen: if value.vhs_sharpen_enable {
                    Some(VHSSharpenSettings {
                        intensity: value.vhs_sharpen_intensity,
                        frequency: value.vhs_sharpen_frequency,
                    })
                } else {
                    None
                },
                edge_wave: if value.vhs_edge_wav_enable {
                    Some(VHSEdgeWaveSettings {
                        intensity: value.vhs_edge_wave_intensity,
                        speed: value.vhs_edge_wave_speed,
                        frequency: value.vhs_edge_wave_frequency,
                        detail: value.vhs_edge_wave_detail,
                    })
                } else {
                    None
                },
            })
        } else {
            None
        };
        fx.chroma_vert_blend = value.vertically_blend_chroma;
        fx.chroma_lowpass_out = value.chroma_low_pass_out.into();
        fx.bandwidth_scale = value.bandwidth_scale;
        fx
    }
}

#[derive(Params, Clone, Debug)]
pub(crate) struct NtscAllParams {
    random_seed: i32,
    bandwidth_scale: f32,
    use_field: NtscUseField,
    low_pass_type: NtscLowPassType,
    input_luma_filter: NtscInputLumaFilter,
    input_chroma_lowpass: NtscChromaLowPass,

    composite_preemphasis: f32,
    composite_noise: f32,
    snow: f32,
    snow_anisotropy: f32,

    scanline_phase_shift: NtscScanlinePhaseShift,
    scanline_phase_shift_offset: i32,

    chroma_demod_filter: NtsChromaDemod,
    luma_smear: f32,

    //GROUP: Head switching
    head_switching_enable: bool,
    head_switching_height: u32,
    head_switching_offset: u32,
    head_switching_horizontal_shift: f32,

    //GROUP: Tracking noise
    tracking_noise_enable: bool,
    tracking_noise_height: u32,
    tracking_noise_wave_intensity: f32,
    tracking_noise_snow_intensity: f32,
    tracking_noise_snow_anisotropy: f32,
    tracking_noise_noise_intensity: f32,

    //GROUP: Ringing
    ringing_enable: bool,
    ringing_frequency: f32,
    ringing_power: f32,
    ringing_scale: f32,

    //GROUP: chroma noise
    chroma_noise_enable: bool,
    chroma_noise_intensity: f32,
    chroma_noise_frequency: f32,
    chroma_noise_detail: u32,

    chroma_phase_error: f32,
    chroma_phase_noise: f32,
    chroma_delay_horizontal: f32,
    chroma_delay_vertical: i32,

    ///GROUP: VHS emulation
    vhs_enable: bool,
    vhs_tape_speed: NtscVhsTapeSpeed,
    vhs_chroma_loss: f32,
    //SUBGROUP: Sharpen
    vhs_sharpen_enable: bool,
    vhs_sharpen_intensity: f32,
    vhs_sharpen_frequency: f32,
    //SUBGROUP: Edge wave
    vhs_edge_wav_enable: bool,
    vhs_edge_wave_intensity: f32,
    vhs_edge_wave_speed: f32,
    vhs_edge_wave_frequency: f32,
    vhs_edge_wave_detail: i32,

    vertically_blend_chroma: bool,

    chroma_low_pass_out: NtscChromaLowPass,
}

impl Default for NtscAllParams {
    fn default() -> Self {
        NtscEffect::default().into()
    }
}
