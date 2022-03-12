use opencv::{
    Result,
    videoio,
    imgproc,
    prelude::*,
    core::*,
};
use std::thread::sleep;
use std::time::Duration;

fn clear_screen() {
    print!("\x1b[2J");
    print!("\x1b[H");
}

fn brightness_to_ascii(brightness: f32) -> char {
    let scale = r#"@%#*+=-:.  "#;
    let max_idx = scale.len();
    let idx = (brightness / 256.0 * max_idx as f32).floor() as usize;
    scale.chars().nth(idx).unwrap()
}

fn brightness(pixel: &VecN<u8, 3>) -> f32 {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    0.3 * r as f32 + 0.59 * g as f32 + 0.11 * b as f32
}

fn print_ascii(img: Mat) {
    let mut buf = String::with_capacity((img.rows() * (img.cols() + 1)) as usize);
    for i in 0..img.rows() {
        for j in 0..img.cols() {
            buf.push(brightness_to_ascii(brightness(img.at_2d::<VecN<u8, 3>>(i, j).unwrap())));
        }
        buf.push('\n');
    }
    print!("{}", buf);
}

fn main() -> Result<()> {
    let mut video = videoio::VideoCapture::from_file("/home/john/Documents/video2ascii/test.avi", videoio::CAP_FFMPEG)?;
    loop {
        clear_screen();
        let mut old_frame = Mat::default();
        let mut new_frame = Mat::default();
        video.read(&mut old_frame)?;
        imgproc::resize(&old_frame, &mut new_frame, Size::new(96, 72), 0.0, 0.0, imgproc::INTER_CUBIC)?;
        print_ascii(new_frame);
        sleep(Duration::from_micros(41666));
    }
}
