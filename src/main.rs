use opencv::{
    Result,
    videoio,
    imgproc,
    highgui,
    prelude::*,
    core::*,
};

use rgb::RGB8;
use ansi_rgb::*;

use std::thread::sleep;
use std::time::Duration;

fn clear_screen() {
    print!("\x1b[2J");
    print!("\x1b[H");
}

fn pixel_to_ascii(pixel: &VecN<u8, 3>) -> char {
    let brightness = brightness(pixel);
    //let scale = r#"@%#*+=-:.   "#;
    let scale = r#"   .:=+*#%@"#;
    let max_idx = scale.len();
    let idx = (brightness / 256.0 * max_idx as f32).floor() as usize;
    scale.chars().nth(idx).unwrap()
}

fn pixel_to_rgb(pixel: &VecN<u8, 3>) -> RGB8 {
    let (b, g, r) = (pixel[0], pixel[1], pixel[2]);
    RGB8::new(r, g, b)
}

fn brightness(pixel: &VecN<u8, 3>) -> f32 {
    let (b, g, r) = (pixel[0], pixel[1], pixel[2]);
    0.3 * r as f32 + 0.59 * g as f32 + 0.11 * b as f32
}

fn print_ascii(img: Mat) {
    let mut buf = String::with_capacity((img.rows() * (img.cols() + 1)) as usize);
    for i in 0..img.rows() {
        for j in 0..img.cols() {
            let pixel = img.at_2d::<VecN<u8, 3>>(i, j).unwrap();
            let ch = pixel_to_ascii(pixel);
            let rgb = pixel_to_rgb(pixel);
            let ch = ch.fg(rgb).to_string();
            buf.push_str(&ch);
        }
        buf.push('\n');
    }
    print!("{}", buf);
}

fn main() -> Result<()> {
    //let mut video = videoio::VideoCapture::from_file("/home/john/Documents/video2ascii/test.avi", videoio::CAP_FFMPEG)?;
    let mut video = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?; 
    loop {
        clear_screen();
        let mut old_frame = Mat::default();
        let mut new_frame = Mat::default();
        video.read(&mut old_frame)?;
        if old_frame.size()?.width <= 0 {
            continue;
        }
        imgproc::resize(&old_frame, &mut new_frame, Size::new(96, 72), 0.0, 0.0, imgproc::INTER_CUBIC)?;
        print_ascii(new_frame);
        sleep(Duration::from_micros(41666));
    }
}
