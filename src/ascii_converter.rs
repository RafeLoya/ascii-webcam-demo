use std::error::Error;
use crate::ascii_frame::AsciiFrame;
use crate::image_frame::ImageFrame;

/// Intermediary translator to transform an `ImageFrame` into an `AsciiFrame`
pub struct AsciiConverter {
    /// ASCII gradient for `AsciiFrame` "pixels"
    ascii: Vec<char>,
    contrast: f32,
    brightness: f32,
}

impl AsciiConverter {
    /// Default ASCII gradient
    pub const DEFAULT_ASCII: &'static str = " .:coPO?@■";
    
    pub fn new(ascii: Vec<char>, contrast: f32, brightness: f32) -> Self {
        Self {
            ascii,
            contrast,
            brightness,
        }
    }
    
    pub fn convert(&self, i_frame: &ImageFrame, a_frame: &mut AsciiFrame) -> Result<(), Box<dyn Error>> {
        // scaling factors to map the ASCII frame dimension 
        // to image frame dimension
        let scale_x = i_frame.w as f32 / a_frame.w as f32;
        let scale_y = i_frame.h as f32 / a_frame.h as f32;
        
        for y in  0..a_frame.h {
            for x in  0..a_frame.w {
                let i_x = (x as f32 * scale_x) as usize;
                let i_y = (y as f32 * scale_y) as usize;
                
                // retrieve RGB values from scaled pixel destination
                // in image frame
                if let Some(rgb) = i_frame.get_pixel(i_x, i_y) {
                    // modify RGB w/ given brightness & contrast values
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

    /// Alter the color channels of an RGB pixel according to the specified
    /// `contrast` and `brightness` values.
    fn adjust_pixel(&self, (r, g, b): (u8, u8, u8)) -> (u8, u8, u8) {
        // closure to independently modify RGB channels
        let apply = |value: u8| -> u8 {
            // normalize color value (0-255) between 0.0 and 1.0
            let mut v = value as f32 / 255.0;
            v = (v - 0.5) * self.contrast + 0.5;
            v += self.brightness;
            // floor of 0.0 and ceiling of 1.0 (prevent overflow)
            v = v.max(0.0).min(1.0);
            (v * 255.0) as u8
        };

        (apply(r), apply(g), apply(b))
    }
    
    /// calculate the grayscale intensity value (relative luminance)
    /// of a given pixel
    fn calculate_intensity(&self, (r, g, b): (u8, u8, u8)) -> u8 {
        // luminance weights from Rec. ITU-R BT.601-7
        (0.2989 * r as f32 + 0.5870 * g as f32 + 0.1140 * b as f32) as u8
    }
}