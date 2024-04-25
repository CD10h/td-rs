use std::mem::size_of;

use ntscrs::yiq_fielding::{Bgrx8, Rgbx16, Rgbx32f, Rgbx8, YiqField, YiqOwned};
use td_rs_top::{PixelFormat, TopDownloadResult};

pub(crate) fn td_top_to_yiq_owned(
    mut top_dl: TopDownloadResult,
    field: YiqField,
) -> Option<YiqOwned> {
    let tex_desc = top_dl.texture_desc();

    let width = tex_desc.width;
    let height = tex_desc.height;

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
        PixelFormat::RGBA16Fixed => Some(YiqOwned::from_strided_buffer::<Rgbx16>(
            top_dl.data(),
            width * 4 * size_of::<u16>(),
            width,
            height,
            field,
        )),
        _ => None,
    };

    res
}
