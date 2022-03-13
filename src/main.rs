mod render;
mod termctl;
mod videoinput;

use clap::Parser;
use opencv::prelude::*;
use opencv::Result;
use std::sync::mpsc;
use std::thread;
use std::time::*;

/// Encode video into ascii animation
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Video input, either a path "~/test.avi" or a camera id "0/1/..."
    #[clap(short, long, default_value = "0")]
    input: String,
    /// Colorized or not
    #[clap(short, long)]
    colored: bool,
    /// Brightness scale represented with a ASCII string
    #[clap(short, long, default_value = r#" .:=+*#%@"#)]
    scale: String,
    /// Width of output animation
    #[clap(short, long)]
    width: Option<u32>,
    /// Height of output animation
    #[clap(short, long)]
    height: Option<u32>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut video = match args.input.parse::<i32>() {
        Ok(cam) => videoinput::from_cam(Some(cam)),
        Err(_) => videoinput::from_file(&args.input),
    }?;
    let (tx, rx) = mpsc::sync_channel(100);
    thread::spawn(move || {
        let mut img = Mat::default();
        while video.read(&mut img).unwrap_or(false) {
            if let Ok(img) = render::resize(&img, args.width, args.height) {
                if let Ok(img) = render::bgr2rgb(img) {
                    let buf = render::render_ascii(&img, args.colored, &args.scale);
                    tx.send(buf).unwrap();
                } else { break; }
            } else { break; }
        }
    });
    while let Ok(buf) = rx.recv() {
        let timer = Instant::now();
        if buf == "" {
            break;
        } else {
            print!("{}", buf);
            termctl::clear_screen();
        }
        thread::sleep(Duration::from_micros(41666) - timer.elapsed())
    }
    Ok(())
}
