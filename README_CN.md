# video2ascii

![Issues](https://img.shields.io/github/issues/jwnhy/video2ascii)
![Forks](https://img.shields.io/github/forks/jwnhy/video2ascii)
![Stars](https://img.shields.io/github/stars/jwnhy/video2ascii)
![License](https://img.shields.io/github/license/jwnhy/video2ascii)

## 需要的依赖

- `opencv` （尝试静态编译但失败了）

## 安装方式

```bash
cargo install video2ascii
```

> 可能还需要将 `cargo` 的路径加入你的 `$PATH` 环境变量

## Demo

![Demo](https://github.com/jwnhy/video2ascii/raw/main/demo.apng)

## 如何使用

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

只做了下面几个功能，加彩色，改使用的ASCII字符，修改长宽（比例不变）。
后面估计还会加上音频，更多显示的方式。

### 配置选项

`-i --input` 指定你使用的视频来源

可以是一个文件路径 `~/test.avi`，可以是一个摄像头，如果是摄像头的话输入摄像头编号。
在 Linux 上是 `/dev/videoX`， `X` 就是可以输入的编号。

`-c --colored` 是否使用彩色输出

`-s --scale` 指定 ASCII *亮度*，例如 `.:=+*#%@`.

`-w --width`  指定宽度，宽度和高度设置一个就行，默认是将原视频压缩到 `1/10`
