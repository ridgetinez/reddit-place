use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PixelEvent {
    pub x: usize,
    pub y: usize,
    pub pixel: Pixel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r: r, g: g, b: b, a: a }
	}

	pub fn as_bytes(&self) -> Vec<u8> {
		vec![self.r, self.g, self.b, self.a]
	}
}
