mod ffmpeg;
mod camera;
mod ascii_renderer;
mod ascii_frame;
mod image_frame;
mod ascii_converter;
mod edge_detector;

use crate::camera::Camera;
use crate::ascii_converter::AsciiConverter;
use crate::ascii_frame::AsciiFrame;
use crate::ascii_renderer::AsciiRenderer;
use crate::image_frame::ImageFrame;

use std::{thread};
use std::time::Duration;
use crate::edge_detector::EdgeDetector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera_w = 640;
    let camera_h = 480;
    
    let ascii_w = 120;
    let ascii_h = 40;
    
    let mut camera = Camera::new(camera_w, camera_h)?;
    
    let mut image_frame = ImageFrame::new(camera_w, camera_h, 3)?;
    let mut ascii_frame = AsciiFrame::new(ascii_w, ascii_h, ' ')?;
    
    let edge_detector = EdgeDetector::new(camera_w, camera_h, 20.0);
    let _edge_thread = edge_detector.start(camera_w, camera_h)?;
    
    let converter = AsciiConverter::default();
    
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