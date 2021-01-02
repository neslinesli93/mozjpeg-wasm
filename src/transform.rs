use mozjpeg_sys::*;
use std::mem;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    code: JXFORM_CODE,
}

impl Transform {
    pub fn no_transform(self) -> bool {
        self.code == JXFORM_CODE_JXFORM_NONE
    }
}

// https://sirv.com/help/articles/rotate-photos-to-be-upright/
// http://sylvana.net/jpegcrop/exif_orientation.html
impl From<u32> for Transform {
    fn from(flag: u32) -> Self {
        let code = match flag {
            2 => JXFORM_CODE_JXFORM_FLIP_H,
            3 => JXFORM_CODE_JXFORM_ROT_180,
            4 => JXFORM_CODE_JXFORM_FLIP_V,
            5 => JXFORM_CODE_JXFORM_TRANSPOSE,
            6 => JXFORM_CODE_JXFORM_ROT_90,
            7 => JXFORM_CODE_JXFORM_TRANSVERSE,
            8 => JXFORM_CODE_JXFORM_ROT_270,
            _ => JXFORM_CODE_JXFORM_NONE,
        };

        Transform { code }
    }
}

pub unsafe fn transform(data: *const u8, size: u32, transform_opt: Transform) -> (*const u8, u32) {
    println!("Start transform");

    let mut transformoption: jpeg_transform_info = mem::zeroed();
    transformoption.transform = transform_opt.code;
    transformoption.perfect = false as boolean;
    transformoption.trim = false as boolean;
    transformoption.force_grayscale = false as boolean;
    transformoption.crop = false as boolean;
    transformoption.slow_hflip = false as boolean;

    // Reader
    let mut srcerr: jpeg_error_mgr = mem::zeroed();
    let mut srcinfo: jpeg_decompress_struct = mem::zeroed();
    srcinfo.common.err = jpeg_std_error(&mut srcerr);
    jpeg_create_decompress(&mut srcinfo);
    println!("Create decompress");

    let mut dsterr = mem::zeroed();
    let mut dstinfo: jpeg_compress_struct = mem::zeroed();
    dstinfo.common.err = jpeg_std_error(&mut dsterr);
    jpeg_create_compress(&mut dstinfo);
    println!("Create compress");

    jpeg_mem_src(&mut srcinfo, data, size as c_ulong);
    jpeg_read_header(&mut srcinfo, true as boolean);
    println!("Read header");

    jtransform_request_workspace(&mut srcinfo, &mut transformoption);
    println!("Request workspace");

    let src_coef_arrays = jpeg_read_coefficients(&mut srcinfo);
    println!("Read coeff");

    jpeg_copy_critical_parameters(&srcinfo, &mut dstinfo);
    println!("Copy crit params");

    let dst_coef_arrays = jtransform_adjust_parameters(
        &mut srcinfo,
        &mut dstinfo,
        src_coef_arrays,
        &mut transformoption,
    );
    println!("Adjust params");

    // Writer
    let mut output_size: c_ulong = 0;
    let mut outbuf: *mut u8 = std::ptr::null_mut();
    jpeg_mem_dest(&mut dstinfo, &mut outbuf, &mut output_size);
    println!("Mem dest");

    jpeg_write_coefficients(&mut dstinfo, dst_coef_arrays);
    println!("Write coeffs");

    jtransform_execute_transform(
        &mut srcinfo,
        &mut dstinfo,
        src_coef_arrays,
        &mut transformoption,
    );
    println!("Execute transform");

    jpeg_finish_compress(&mut dstinfo);
    jpeg_destroy_compress(&mut dstinfo);

    jpeg_finish_decompress(&mut srcinfo);
    jpeg_destroy_decompress(&mut srcinfo);

    (outbuf, output_size as u32)
}
