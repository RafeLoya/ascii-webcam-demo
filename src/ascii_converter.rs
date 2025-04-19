use std::error::Error;
use crate::ascii_frame::AsciiFrame;
use crate::image_frame::ImageFrame;

pub struct AsciiConverter {
    ascii: Vec<char>,
    contrast: f32,
    brightness: f32,
}

impl AsciiConverter {
    pub const DEFAULT_ASCII: &'static str = " .:coPO?@■";
    
    pub fn new(ascii: Vec<char>, contrast: f32, brightness: f32) -> Self {
        Self {
            ascii,
            contrast,
            brightness,
        }
    }
    
    pub fn convert(&self, i_frame: &ImageFrame, a_frame: &mut AsciiFrame) -> Result<(), Box<dyn Error>> {
        let scale_x = i_frame.w as f32 / a_frame.w as f32;
        let scale_y = i_frame.h as f32 / a_frame.h as f32;
        
        for y in  0..a_frame.h {
            for x in  0..a_frame.w {
                let i_x = (x as f32 * scale_x) as usize;
                let i_y = (y as f32 * scale_y) as usize;
                
                if let Some(rgb) = i_frame.get_pixel(i_x, i_y) {
                    let rgb_adj = self.adjust_pixel(rgb);
                    let intensity = self.calculate_intensity(rgb_adj);
                    
                    let char_i = (intensity as f32 / 255.0 * self.ascii.len() as f32) as usize;
                    // bounds check (e.g. floating point rounding error)
                    let char_i = char_i.min(self.ascii.len() - 1);
                    
                    a_frame.set_char(x, y, self.ascii[char_i]);
                }
            }
        }
        
        Ok(())
    }

    fn adjust_pixel(&self, (r, g, b): (u8, u8, u8)) -> (u8, u8, u8) {
        let apply = |value: u8| -> u8 {
            let mut v = value as f32 / 255.0;
            v = (v - 0.5) * self.contrast + 0.5;
            v += self.brightness;
            v = v.max(0.0).min(1.0);
            (v * 255.0) as u8
        };

        (apply(r), apply(g), apply(b))
    }
    
    fn calculate_intensity(&self, (r, g, b): (u8, u8, u8)) -> u8 {
        (0.2989 * r as f32 + 0.5870 * g as f32 + 0.1140 * b as f32) as u8
    }
}