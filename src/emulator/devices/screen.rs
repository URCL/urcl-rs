#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone)]
pub struct Screen {
    pixels: Vec<u32>, // RGBA
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0x00_00_00_ff; width*height];
        Self { pixels, width, height, x: 0, y: 0 }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn out_x(&mut self, value: u64) {
        self.x = value as usize;
    }
    pub fn out_y(&mut self, value: u64) {
        self.y = value as usize;
    }
    pub fn out_color(&mut self, value: u64) {
        if self.x >= self.width || self.y >= self.height {return;}
        self.pixels[self.x + self.y * self.width] = ((value as u32) << 8u32).to_be() | 0xff_00_00_00;
    }
    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }
}