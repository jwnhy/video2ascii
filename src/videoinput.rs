
use opencv::videoio::VideoCapture;
use opencv::videoio::CAP_ANY;
use opencv::Result;

pub fn from_file(path: &str) -> Result<VideoCapture> {
    VideoCapture::from_file(path, CAP_ANY)
}

pub fn from_cam(src: Option<i32>) -> Result<VideoCapture> {
    match src {
        None => VideoCapture::new(0, CAP_ANY),
        Some(src) => VideoCapture::new(src, CAP_ANY)
    }
}
