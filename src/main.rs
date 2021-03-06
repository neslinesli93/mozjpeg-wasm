pub mod transform;

use mozjpeg_sys::*;
use std::mem;
use std::os::raw::c_ulong;

use crate::transform::{transform, Transform};

#[repr(C)]
pub struct JpegData {
    pointer: u32,
    size: u32,
}

#[no_mangle]
pub fn new_convert(
    input: *const u8,
    input_size: u32,
    quality: u32,
    orientation: u32,
) -> *mut JpegData {
    unsafe {
        let (data, width, height) = decode(input, input_size);
        let (output, output_size) = encode(&data, width, height, quality);

        let transform_opt: Transform = orientation.into();
        let (result, result_size) = if transform_opt.no_transform() {
            (output, output_size)
        } else {
            transform(output, output_size, transform_opt)
        };

        let boxed = Box::new(JpegData {
            pointer: result as u32,
            size: result_size,
        });
        Box::into_raw(boxed)
    }
}

#[no_mangle]
unsafe fn decode(input: *const u8, input_size: u32) -> (Vec<u8>, u32, u32) {
    let mut err: jpeg_error_mgr = mem::zeroed();
    let mut cinfo: jpeg_decompress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_decompress(&mut cinfo);

    jpeg_mem_src(&mut cinfo, input, input_size as c_ulong);
    jpeg_read_header(&mut cinfo, true as boolean);

    let width = cinfo.image_width;
    let height = cinfo.image_height;

    cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
    jpeg_start_decompress(&mut cinfo);
    let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
    let buffer_size = row_stride * cinfo.image_height as usize;
    let mut buffer = vec![0u8; buffer_size];

    while cinfo.output_scanline < cinfo.output_height {
        let offset = cinfo.output_scanline as usize * row_stride;
        let mut jsamparray = [buffer[offset..].as_mut_ptr()];
        jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1);
    }

    jpeg_finish_decompress(&mut cinfo);
    jpeg_destroy_decompress(&mut cinfo);

    (buffer, width, height)
}

unsafe fn encode(inbuf: &[u8], width: u32, height: u32, quality: u32) -> (*const u8, u32) {
    let mut err = mem::zeroed();
    let mut cinfo: jpeg_compress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_compress(&mut cinfo);

    let mut output_size: c_ulong = 0;
    let mut outbuf: *mut u8 = std::ptr::null_mut();
    jpeg_mem_dest(&mut cinfo, &mut outbuf, &mut output_size);

    cinfo.image_width = width;
    cinfo.image_height = height;
    cinfo.in_color_space = J_COLOR_SPACE::JCS_RGB;
    cinfo.input_components = 3;
    jpeg_set_defaults(&mut cinfo);

    let row_stride = cinfo.image_width as usize * cinfo.input_components as usize;
    cinfo.dct_method = J_DCT_METHOD::JDCT_ISLOW;
    jpeg_set_quality(&mut cinfo, quality as i32, true as boolean);

    jpeg_start_compress(&mut cinfo, true as boolean);

    while cinfo.next_scanline < cinfo.image_height {
        let offset = cinfo.next_scanline as usize * row_stride;
        let jsamparray = [inbuf[offset..].as_ptr()];
        jpeg_write_scanlines(&mut cinfo, jsamparray.as_ptr(), 1);
    }

    jpeg_finish_compress(&mut cinfo);
    jpeg_destroy_compress(&mut cinfo);

    (outbuf, output_size as u32)
}

fn main() {
    // Deliberately blank.
}
