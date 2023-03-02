use crate::body::World;
use crate::point::Point;
use crate::window::Window;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

#[allow(dead_code)]
pub enum Diffusion {
    Random,
    Lambertian,
    Alternative,
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
}

impl Camera {
    pub fn new(vp_width: f64, vp_height: f64, focal_length: f64) -> Self {
        let origin = Point::new(0., 0., 0.);
        let horizontal = Point::new(vp_width, 0., 0.);
        let vertical = Point::new(0., vp_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Point::new(0., 0., focal_length);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn new_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}

pub struct Tracer {
    screen: Window,
    width: usize,
    height: usize,
    camera: Camera,
    world: World,
    samples_per_pixel: usize,
    max_depth: usize,
    diffusion: Diffusion,
}

impl Tracer {
    pub fn new(
        width: usize,
        height: usize,
        focal_length: f64,
        world: World,
        samples_per_pixel: usize,
        max_depth: usize,
        diffusion: Diffusion,
    ) -> Self {
        let screen = Window::new(width, height);
        let camera = Camera::new(2. * (width as f64 / height as f64), 2., focal_length);
        Tracer {
            screen,
            width,
            height,
            camera,
            world,
            samples_per_pixel,
            max_depth,
            diffusion,
        }
    }
    pub fn ray_color(&self, ray: Ray, depth: usize) -> Point {
        if depth <= 0 {
            return Point::default();
        }

        if let Some(record) = self.world.hit(&ray, 0.001, f64::INFINITY) {
            let target = match self.diffusion {
                Diffusion::Random => record.p + record.normal + Point::random_in_unit_sphere(),
                Diffusion::Lambertian => record.p + record.normal + Point::random_unit_vector(),
                Diffusion::Alternative => {
                    record.p + record.normal + Point::random_in_hemisphere(record.normal)
                }
            };
            return 0.5 * self.ray_color(Ray::new(record.p, target - record.p), depth - 1);
        }
        //background
        let unit_d = ray.direction;
        let t = 0.5 * (unit_d.y + 1.);
        return (1. - t) * Point::new(1., 1., 1.) + t * Point::new(0.5, 0.7, 1.0);
    }

    pub fn render(&mut self) {
        let mut rng = rand::thread_rng();

        let bar = ProgressBar::new((self.width * self.height).try_into().unwrap());
        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise:.cyan}] {spinner}{bar:40.cyan/blue} {msg}",
            )
            .unwrap()
            .progress_chars("▰▰▱")
            .tick_strings(&vec!["🙈 🙉 🙊 ", "🙉 🙊 🙈 ", "🙊 🙈 🙉 "]),
        );
        for j in 0..self.height {
            for i in 0..self.width {
                bar.inc(1);
                let mut color = Point::default();
                for _ in 0..self.samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / (self.width - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (self.height - 1) as f64;
                    let ray = self.camera.new_ray(u, v);
                    color = color + self.ray_color(ray, self.max_depth);
                }

                self.screen.pixels[j][i].set_color(color, self.samples_per_pixel);
            }
        }

        bar.finish()
    }

    pub fn image(&self) -> String {
        self.screen.to_ppm()
    }
}
