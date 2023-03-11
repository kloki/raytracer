use std::f64::consts::PI;

use crate::body::World;
use crate::color::Color;
use crate::point::Point;
use crate::window::Window;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Point {
        self.origin + distance * self.direction
    }
}

pub struct Camera {
    origin: Point,
    horizontal: Point,
    vertical: Point,
    lower_left_corner: Point,
    u: Point,
    v: Point,
    lens_radius: f64,
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        vup: Point,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let vp_height = 2. * h;
        let vp_width = aspect_ratio * vp_height;

        let w = (look_from - look_at).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * vp_width * u;
        let vertical = focus_dist * vp_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_dist * w;
        let lens_radius = aperture / 2.;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }
    pub fn new_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Point::random_unit_vector();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offset,
        }
    }
}

pub struct Tracer {
    pub screen: Window,
    width: usize,
    height: usize,
    camera: Camera,
    world: World,
    samples_per_pixel: usize,
    max_depth: usize,
}

impl Tracer {
    pub fn new(
        width: usize,
        height: usize,
        camera: Camera,
        world: World,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let screen = Window::new(width, height);
        Tracer {
            screen,
            width,
            height,
            camera,
            world,
            samples_per_pixel,
            max_depth,
        }
    }
    pub fn ray_color(&self, ray: Ray, depth: usize) -> Point {
        if depth <= 0 {
            return Point::default();
        }

        if let Some(record) = self.world.hit(&ray, 0.001, f64::INFINITY) {
            match record.body_props.scatter(&ray, &record) {
                None => return Point::default(),
                Some((attenuation, scattered)) => {
                    return attenuation * self.ray_color(scattered, depth - 1)
                }
            }
        }
        //background
        let unit_d = ray.direction;
        let t = 0.5 * (unit_d.y + 1.);
        return (1. - t) * Point::new(1., 1., 1.) + t * Point::new(0.5, 0.7, 1.0);
    }

    pub fn progress_bar(&self) -> ProgressBar {
        let bar = ProgressBar::new((self.width * self.height).try_into().unwrap());
        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise:.cyan}] {spinner}{bar:40.cyan/blue} {msg}",
            )
            .unwrap()
            .progress_chars("▰▰▱")
            .tick_strings(&vec!["🙈 🙉 🙊 ", "🙉 🙊 🙈 ", "🙊 🙈 🙉 "]),
        );
        bar
    }
    pub fn render(&mut self) {
        let bar = self.progress_bar();

        let mut new_pixels: Vec<Vec<Color>> = vec![];

        for j in 0..self.height {
            new_pixels.push(
                (0..self.width)
                    .into_par_iter()
                    .map(|i| {
                        let mut rng = rand::thread_rng();
                        bar.inc(1);
                        let mut color = Point::default();
                        for _ in 0..self.samples_per_pixel {
                            let u = (i as f64 + rng.gen::<f64>()) / (self.width - 1) as f64;
                            let v = (j as f64 + rng.gen::<f64>()) / (self.height - 1) as f64;
                            let ray = self.camera.new_ray(u, v);
                            color = color + self.ray_color(ray, self.max_depth);
                        }

                        let mut rgb = Color::default();
                        rgb.set_color(color, self.samples_per_pixel);
                        rgb
                    })
                    .collect::<Vec<Color>>(),
            )
        }

        self.screen.pixels = new_pixels;

        bar.finish()
    }

    pub fn image(&self) -> String {
        self.screen.to_ppm()
    }
}
