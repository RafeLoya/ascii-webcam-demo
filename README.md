# pinhole-demo

This is a small demo / experiment for a larger Rust project that involves
streaming live video feeds from inside a terminal. After struggling with various
crates and wrappers for multimedia frameworks, I resorted to using the FFmpeg
CLI directly, which seemed to work well.

## Requirements

The program directly uses the FFmpeg CLI, which can be downloaded 
[here](https://ffmpeg.org/download.html). On macOS / Linux, this can be simplified 
by using Homebrew: `brew install ffmpeg`.
