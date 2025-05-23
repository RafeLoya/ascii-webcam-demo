mod ffmpeg;
mod camera;
mod ascii_renderer;
mod ascii_frame;
mod image_frame;
mod ascii_converter;
mod edge_detector;
mod video_config;

use crate::ascii_converter::AsciiConverter;
use crate::ascii_frame::AsciiFrame;
use crate::ascii_renderer::AsciiRenderer;
use crate::camera::Camera;
use crate::image_frame::ImageFrame;

use crate::video_config::VideoConfig;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = VideoConfig::new(
        640,
        480,
        120,
        40,
        127.50,
        1.5,
        0.0
    );
    
    let mut camera = Camera::new(config.camera_width, config.camera_height)?;
    
    let mut image_frame = ImageFrame::new(config.camera_width, config.camera_height, 3)?;
    let mut ascii_frame = AsciiFrame::new(config.ascii_width, config.ascii_height, ' ')?;
    
    let converter = AsciiConverter::new(
        AsciiConverter::DEFAULT_ASCII_INTENSITY.chars().collect(),
        AsciiConverter::DEFAULT_ASCII_HORIZONTAL.chars().collect(),
        AsciiConverter::DEFAULT_ASCII_VERTICAL.chars().collect(),
        AsciiConverter::DEFAULT_ASCII_FORWARD.chars().collect(),
        AsciiConverter::DEFAULT_ASCII_BACK.chars().collect(),
        config.camera_width,
        config.camera_height,
        config.edge_threshold,
        config.contrast,
        config.brightness
    )?;
    
    let mut renderer = AsciiRenderer::new()?;
    
    loop {
        if let Err(e) = camera.capture_frame(&mut image_frame) {
            eprintln!("failed while capturing frame: {}", e);
            break;
        }
        
        if let Err(e) = converter.convert(&image_frame, &mut ascii_frame) {
            eprintln!("failed while converting frame: {}", e);
            break;
        }
        
        if let Err(e) = renderer.render(&ascii_frame) {
            eprintln!("failed while rendering frame: {}", e);
            break;
        }

        thread::sleep(Duration::from_millis(10));
    }
    
    Ok(())
}