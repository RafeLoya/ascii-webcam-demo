use std::error::Error;
use crate::ffmpeg;

use std::io::{BufReader, Read};
use std::process::Child;
use crate::image_frame::ImageFrame;

const DEFAULT_BYTES_PER_PIXEL: usize = 3;

pub struct Camera {
    w: usize,
    h: usize,
    ffmpeg_proc: Child,
    frame_reader: BufReader<std::process::ChildStdout>,
    frame_buffer: Vec<u8>
}

impl Camera {
    pub fn new(w: usize, h: usize) -> Result<Self, Box<dyn Error>> {
        if w == 0 || h == 0 {
            return Err("dimensions must be greater than zero".into());
        }
        
        let mut ffmpeg_proc = ffmpeg::setup_default()?;

        let stdout = match ffmpeg_proc.stdout.take() {
            Some(stdout) => stdout,
            None => return Err("failed to get ffmpeg stdout".into()),
        };
        
        let bytes_per_pixel = DEFAULT_BYTES_PER_PIXEL;
        let buffer_size = w * h * bytes_per_pixel;
        
        Ok(Camera {
            w,
            h,
            ffmpeg_proc,
            frame_reader: BufReader::with_capacity(buffer_size, stdout),
            frame_buffer: vec![0u8; buffer_size]
        })
    }
    
    pub fn capture_frame(&mut self, frame: &mut ImageFrame) -> Result<(), Box<dyn Error>> {
        if frame.w != self.w || frame.h != self.h {
            return Err(format!(
                "frame dimensions ({}x{}) do not match camera dimensions ({}x{})",
                frame.w, frame.h, self.w, self.h
            ).into());
        }
        
        if let Err(e) = self.frame_reader.read_exact(&mut self.frame_buffer) {
            return Err(format!("failed to read camera frame: {}", e).into());
        }
        
        //let target_buffer = frame.buffer_mut();
        if self.frame_buffer.len() != frame.buffer().len() {
            return Err(format!(
                "buffer size not consistent between camera ({}) and frame ({})",
                self.frame_buffer.len(), frame.buffer().len()
            ).into());
        }
        
        frame.buffer_mut().copy_from_slice(&self.frame_buffer);
        
        Ok(())
    }
    
    pub fn dimensions(&self) -> (usize, usize) {
        (self.w, self.h)
    }
}

impl Drop for Camera {
    fn drop(&mut self) {
        // kill ffmpeg when Camera is dropped
        if let Err(e) = self.ffmpeg_proc.kill() {
            eprintln!("failed to kill ffmpeg: {}", e);
        }
    }
}