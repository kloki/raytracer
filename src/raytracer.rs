use crate::body::Body;
use crate::point::Point;
use crate::window::Window;
use indicatif::ProgressBar;
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

pub struct Tracer {
    screen: Window,
    width: usize,
    height: usize,
    origin: Point,
    horizontal: Point,
    vertical: Point,
    lower_left_corner: Point,
    bodies: Vec<Box<dyn Body>>,
}

impl Tracer {
    pub fn new(
        width: usize,
        height: usize,
        vp_width: f64,
        vp_height: f64,
        focal_length: f64,
        bodies: Vec<Box<dyn Body>>,
    ) -> Self {
        let screen = Window::new(width, height);
        let origin = Point::new(0., 0., 0.);
        let horizontal = Point::new(vp_width, 0., 0.);
        let vertical = Point::new(0., vp_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Point::new(0., 0., focal_length);
        Tracer {
            screen,
            width,
            height,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            bodies,
        }
    }
    pub fn ray_color(&self, ray: Ray) -> Point {
        for body in &self.bodies {
            if let Some(angle) = body.hit(&ray) {
                return body.color(&ray, angle);
            }
        }
        //background
        let unit_d = ray.direction;
        let t = 0.5 * (unit_d.y + 1.);
        return (1. - t) * Point::new(1., 1., 1.) + t * Point::new(0.5, 0.7, 1.0);
    }

    pub fn render(&mut self) {
        let bar = ProgressBar::new((self.width * self.height).try_into().unwrap());
        for j in 0..self.height {
            for i in 0..self.width {
                bar.inc(1);
                let u = i as f64 / (self.width - 1) as f64;
                let v = j as f64 / (self.height - 1) as f64;
                let direction =
                    self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
                let ray = Ray::new(self.origin, direction);
                let ray_color = self.ray_color(ray);

                self.screen.pixels[j][i].set_color(ray_color);
            }
        }

        bar.finish()
    }

    pub fn image(&self) -> String {
        self.screen.to_ppm()
    }
}
