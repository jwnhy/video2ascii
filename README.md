
# video2ascii

Yet another video to ASCII tool (in Rust)

## Demo

![Demo](./demo.apng)

## Requirements

- `opencv`

## Installation

```bash
cargo install video2ascii
```

## How to use

It comes with a self-explain help file.

```bash
$ ./video2ascii --help
video2ascii 0.1.0
Simple program to encode video into ascii animation

USAGE:
    video2ascii [OPTIONS]

OPTIONS:
    -c, --colored            Colorized or not
    -h, --height <HEIGHT>    Height of output animation
        --help               Print help information
    -i, --input <INPUT>      Video input, either a path "~/test.avi" or a camera id "0/1/..."
                             [default: 0]
    -s, --scale <SCALE>      Brightness scale represented with a ASCII string [default: " .:=+*#%@"]
    -V, --version            Print version information
    -w, --width <WIDTH>      Width of output animation
```

### Options

`-i --input` specifies the source of input video.

It can be either a path to a video file `~/test.avi` or a camera installed in your computer.
In Linux, the camera is listed in `/dev/videoX`, where `X` is the number to be input here.

`-c --colored` specifies whether to output colored ASCII animation.

`-s --scale` specifies the *brightness* of the ASCII anime, which should looks like `.:=+*#%@`.

`-w --width` specifies the height of the ASCII anime, only one of width or height is needed.

As `video2ascii` respect the original ratio of the video.
