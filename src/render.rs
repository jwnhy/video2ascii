use ansi_rgb::*;
use rgb::RGB8;
use opencv::prelude::*;
use opencv::core::*;
use opencv::imgproc;
use opencv::Result;

#[inline]
fn brightness(pixel: &RGB8) -> f32 {
    0.3 * pixel.r as f32 + 0.59 * pixel.g as f32 + 0.11 * pixel.b as f32
}

fn pixel_to_ascii(pixel: &RGB8, scale: &str) -> char {
    let brightness = brightness(pixel);
    let max_idx = scale.len();
    let idx = (brightness / 256.0 * max_idx as f32).floor() as usize;
    scale.chars().nth(idx).unwrap()
}

pub fn resize(img: &Mat, w: Option<u32>, h: Option<u32>) -> Result<Mat> {
    let img_size = img.size()?;
    let ratio = match (w, h) {
        (Some(w), _)  =>  (w as f64) / (img_size.width as f64),
        (None, Some(h)) => (h as f64) / (img_size.height as f64),
        (None, None) => 0.1
    };

    let mut resized = Mat::default();
    imgproc::resize(&img, &mut resized, Size_::new(0, 0), ratio, ratio, imgproc::INTER_CUBIC)?;
    Ok(resized)
}

pub fn bgr2rgb(img: Mat) -> Result<Mat> {
    let mut rgb = Mat::default();
    imgproc::cvt_color(&img, &mut rgb, imgproc::COLOR_BGR2RGB, 0)?;
    Ok(rgb)
}


pub fn render_ascii(img: &Mat, colored: bool, scale: &str) -> String {
    let mut buf = String::with_capacity((img.rows() * (img.cols() + 1)) as usize);
    for i in 0..img.rows() {
        for j in 0..img.cols() {
            let pixel = img.at_2d::<RGB8>(i, j).unwrap();
            let ch = pixel_to_ascii(pixel, scale);
            if colored {
                buf.push_str(&ch.fg(*pixel).to_string());
            } else {
                buf.push(ch);
            }
        }
        buf.push('\n');
    }
    buf
}

