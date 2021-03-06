mod materials;
mod objects;
mod raytracer;
mod vector;

extern crate image;

use crate::materials::*;
use crate::objects::*;
use crate::vector::Vector;

const WIDTH: usize = 640;

fn main() {
    let aspect_ratio = AspectRatio::new(16, 9);
    let height = aspect_ratio.calc_height(WIDTH);
    let world = get_world();
    let viewplane = raytracer::run(world, WIDTH, aspect_ratio);
    create_image(&viewplane, WIDTH, height);
}

fn get_world() -> HitableList {
    let mut hitables: Vec<Box<dyn Hitable>> = Vec::new();
    hitables.push(Box::new(Sphere::new(
        Vector::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Diffuse::new(Rgb::new(0.8, 0.3, 0.3))),
    )));
    hitables.push(Box::new(Sphere::new(
        Vector::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Rgb::new(0.8, 0.6, 0.2))),
    )));
    hitables.push(Box::new(Sphere::new(
        Vector::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Rgb::new(0.8, 0.8, 0.8))),
    )));
    hitables.push(Box::new(Sphere::new(
        Vector::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Diffuse::new(Rgb::new(0.8, 0.8, 0.0))),
    )));
    HitableList::new(hitables)
}

pub struct AspectRatio {
    pub w: usize,
    pub h: usize,
}

impl AspectRatio {
    pub fn new(w: usize, h: usize) -> AspectRatio {
        AspectRatio { w, h }
    }

    pub fn calc_height(&self, width: usize) -> usize {
        (width / self.w) * self.h
    }

    pub fn resize(&self, val: f32) -> f32 {
        (self.w as f32 / self.h as f32) * val
    }
}

type Rgb = Vector;

fn create_image(matrix: &Vec<Vec<Rgb>>, width: usize, height: usize) {
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let rgb = matrix[y as usize][x as usize] * 255.0;
        *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
    }
    imgbuf.save("render.png").unwrap();
}
