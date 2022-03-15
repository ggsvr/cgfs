use image::{RgbImage, Rgb};
use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub image: RgbImage,
}

impl Canvas {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            image: RgbImage::new(x, y),
        }
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }
    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn max_x(&self) -> i32 {
        self.image.width() as i32 / 2
    }
    pub fn min_x(&self) -> i32 {
        -(self.image.width() as i32 / 2)
    }

    pub fn max_y(&self) -> i32 {
        self.image.height() as i32 / 2
    }
    pub fn min_y(&self) -> i32 {
        -(self.image.height() as i32 / 2)
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let new_x = self.image.width() as i32 / 2 + x;
        let new_y = self.image.height() as i32 / 2 - y - 1;

        //let new_x: u32 = match new_x.try_into() {
        //    Ok(x) => x,
        //    Err(_) => return,
        //};

        //let new_y: u32 = match new_y.try_into() {
        //    Ok(y) => y,
        //    Err(_) => return,
        //};

        let new_x: u32 = new_x.try_into().unwrap();
        let new_y: u32 = new_y.try_into().unwrap();

        self.image.put_pixel(new_x, new_y, Rgb([color.r, color.g, color.b]));
    }
}
