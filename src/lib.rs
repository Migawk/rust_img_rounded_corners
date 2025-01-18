use std::ffi::{c_char, CStr};

use image::{ImageBuffer, ImageReader, Rgba, RgbaImage};

#[no_mangle]
pub extern "C" fn create_rounded_corners_image(
    inp: *const c_char,
    out: *const c_char,
    radius: u32,
) -> bool {
    let inp_str = unsafe {
        assert!(!inp.is_null()); // Ensure the pointer is not null
        CStr::from_ptr(inp)
    }
    .to_string_lossy();

    let out_str = unsafe {
        assert!(!out.is_null()); // Ensure the pointer is not null
        CStr::from_ptr(out)
    }
    .to_string_lossy();

    let orig = ImageReader::open(inp_str.as_ref())
        .expect("Not found")
        .decode()
        .expect("Err during the processing")
        .to_rgb8();

    let (w, h) = orig.dimensions();

    let mut img: RgbaImage = ImageBuffer::new(w, h);
    let right_x = w - radius;
    let bottom_y = h - radius;

    for x in 0..w {
        for y in 0..h {
            let px = orig.get_pixel(x, y);
            img.put_pixel(x, y, Rgba([px[0], px[1], px[2], 255]));
        }
    }
    for x in 0..radius {
        for y in 0..radius {
            let dx = (x as i32 - radius as i32).abs();
            let dy = (y as i32 - radius as i32).abs();

            let is_within = dx * dx + dy * dy <= (radius * radius) as i32;

            if !is_within {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }
    for x in right_x..w {
        for y in 0..radius {
            let dx = (w as i32 - x as i32 - radius as i32).abs();
            let dy = (y as i32 - radius as i32).abs();

            let is_within = (dx * dx + dy * dy) <= (radius * radius) as i32;

            if !is_within {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }
    for x in 0..radius {
        for y in bottom_y..h {
            let dx = (x as i32 - radius as i32).abs();
            let dy = (h as i32 - y as i32 - radius as i32).abs();

            let is_within = (dx * dx + dy * dy) <= (radius * radius) as i32;

            if !is_within {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }
    for x in right_x..w {
        for y in bottom_y..h {
            let dx = (w as i32 - x as i32 - radius as i32).abs();
            let dy = (h as i32 - y as i32 - radius as i32).abs();

            let is_within = (dx * dx + dy * dy) <= (radius * radius) as i32;

            if !is_within {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    let res = img.save(out_str.as_ref());

    if res.is_ok() {
        true
    } else {
        false
    }
}
