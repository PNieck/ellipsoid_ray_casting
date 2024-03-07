extern crate nalgebra as na;

use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;
use na::Point2;

use super::color::Color;


const PIXEL_LEN: usize = 4;


pub struct Canvas {
    pixels: Pixels,

    width: u32,
    height: u32
}


pub struct Pixel<'a> {
    bytes: &'a [u8; PIXEL_LEN],
}


pub struct PixelsIter<'a> {
    data: &'a [u8],
    cur_pos: usize,
    len: usize
}


impl Canvas {
    pub fn new(window: &Window) -> Canvas {
        let window_size = window.inner_size();

        let mut result = Canvas {
            pixels: {
                let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
                Pixels::new(window_size.width, window_size.height, surface_texture)
                    .expect("Error while creating canvas")
            },
            width: window_size.width,
            height: window_size.height
        };

        result.pixels.frame_mut().fill(u8::MAX);

        result
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height).expect("Error while resizing canvas");
        self.pixels.resize_buffer(width, height).expect("Error while resizing canvas");

        let old_pixels_cnt = (self.width * self.height) as usize;

        if old_pixels_cnt < (width * height) as usize {
            self.pixels.frame_mut()[old_pixels_cnt..].fill(u8::MAX);
        }

        self.width = width;
        self.height = height;
    }

    pub fn pixels(&mut self) -> &Pixels {
        &self.pixels
    }

    pub fn set_pixel(&mut self, color: Color, row: u32, column: u32) {
        assert!(row < self.height, "Row is outside of range");
        assert!(column < self.width, "Column is outside of range");

        let pixel_index: usize = (row * self.width + column) as usize * PIXEL_LEN;

        let frame = self.pixels.frame_mut();
        frame[pixel_index] = color.red();
        frame[pixel_index + 1] = color.green();
        frame[pixel_index + 2] = color.blue();
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn render(&self) {
        self.pixels.render().expect("Error while rendering image");
    }

    pub fn draw_rectangle(&mut self, up_left: Point2<u32>, down_right: Point2<u32>, color: Color) {
        let max_x = u32::min(down_right.x, self.width - 1);
        let max_y = u32::min(down_right.y, self.height - 1);

        for x in up_left.x..=max_x {
            for y in up_left.y..=max_y {
                self.set_pixel(color, y, x);
            }
        }
    }
}


impl<'a> Pixel<'a> {

}


impl<'a> PixelsIter<'a> {
    fn new(canvas: &mut Canvas) -> PixelsIter {
        PixelsIter {
            data: canvas.pixels.frame_mut(),
            cur_pos: 0,
            len: (canvas.width * canvas.height) as usize
        }
    }
}

// impl<'a> Iterator for PixelsIter<'a> {
//     type Item = Pixel<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.cur_pos > self.len {
//             return None;
//         }

//         let new_pos = self.cur_pos + PIXEL_LEN;
//         let result = Pixel{bytes: &self.data[self.cur_pos..=new_pos].try_into().unwrap()};
        
//         self.cur_pos = new_pos;

//         return Some(result);
//     }
// }