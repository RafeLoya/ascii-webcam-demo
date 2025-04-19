use std::error::Error;

/// Initial frame received from webcam feed
pub struct ImageFrame{
    pub w: usize,
    pub h: usize,
    /// usually 3 (RGB)
    pub bytes_per_pixel: usize,
    /// frame data
    buffer: Vec<u8>,
}

impl ImageFrame{
    pub fn new(w: usize, h: usize, bytes_per_pixel: usize) -> Result<Self, Box<dyn Error>> {
        if w == 0 || h == 0 || bytes_per_pixel == 0 {
            return Err("width, height, and bytes per pixel must be greater than zero".into());
        }
        
        Ok(Self {
            w,
            h,
            bytes_per_pixel,
            buffer: vec![0; w * h * bytes_per_pixel],
        })
    }
    
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
    
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
    
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<(u8, u8, u8)> {
        if x >= self.w || y >= self.h {
            return None;
        }
        
        let i = (y * self.w + x) * self.bytes_per_pixel;
        if i + 2 >=  self.buffer.len() {
            return None;
        }
        
        Some((
            self.buffer[i], 
            self.buffer[i + 1],
            self.buffer[i + 2],
        ))
    }
}