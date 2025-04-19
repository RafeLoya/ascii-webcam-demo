use std::error::Error;

const DEFAULT_W: usize = 120;
const DEFAULT_H: usize = 40;

pub struct AsciiFrame {
    pub w: usize,
    pub h: usize,
    chars: Vec<char>,
}

impl AsciiFrame {
    pub fn new(w: usize, h: usize, default_char: char) -> Result<Self, Box<dyn Error>> {
        if w == 0 || h == 0 {
            return Err("Dimensions must be greater than zero".into());
        }

        Ok(Self {
            w,
            h,
            chars: vec![default_char; w * h],
        })
    }

    pub fn set_char(&mut self, x: usize, y: usize, c: char) -> bool {
        if x >= self.w || y >= self.h {
            return false;
        }

        let idx = y * self.w + x;
        if idx < self.chars.len() {
            self.chars[idx] = c;
            true
        } else {
            false
        }
    }

    pub fn get_char(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.w || y >= self.h {
            return None;
        }

        let idx = y * self.w + x;
        self.chars.get(idx).copied()
    }

    pub fn chars(&self) -> &[char] {
        &self.chars
    }
}