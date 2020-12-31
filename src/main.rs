use mozjpeg_sys::*;
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_ulong};

#[no_mangle]
pub fn convert(input: *const u8, input_size: u32, quality: u32) -> *const c_char {
    unsafe {
        let (data, width, height) = decode(input, input_size);
        let (output, output_size) = encode(&data, width, height, quality);

        CString::new(format!("{:?}|{:?}", output, output_size))
            .unwrap()
            .into_raw()
    }
}

#[no_mangle]
unsafe fn decode(input: *const u8, input_size: u32) -> (Vec<u8>, u32, u32) {
    let mut err: jpeg_error_mgr = mem::zeroed();
    let mut cinfo: jpeg_decompress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_decompress(&mut cinfo);

    // https://stackoverflow.com/questions/51047146/how-to-read-a-file-with-javascript-to-webassembly-using-rust
    jpeg_mem_src(&mut cinfo, input, input_size as c_ulong);
    jpeg_read_header(&mut cinfo, true as boolean);

    let width = cinfo.image_width;
    let height = cinfo.image_height;
    println!("Image size {}x{}", width, height);

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

    println!("Decoded into {} raw pixel bytes", buffer.len());

    jpeg_finish_decompress(&mut cinfo);
    jpeg_destroy_decompress(&mut cinfo);

    (buffer, width, height)
}

unsafe fn encode(inbuf: &[u8], width: u32, height: u32, quality: u32) -> (*const u8, u32) {
    println!("Start encoding");
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
