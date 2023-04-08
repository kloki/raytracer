use crate::bodies::{Body, BodyProps, Cube, Sphere, BVH};
use crate::point::Point;
use crate::raytracer::{Camera, Tracer};
#[allow(dead_code)]
pub fn three_balls() -> Tracer {
    let floor = Sphere::new(
        Point::new(0., -100.5, -1.),
        100.0,
        BodyProps::matte(Point::new(0.8, 0.8, 0.)),
    );

    let ball_1 = Sphere::new(
        Point::new(0., 0., -1.),
        0.5,
        BodyProps::matte(Point::new(0.1, 0.2, 0.5)),
    );
    let ball_2 = Sphere::new(Point::new(-1., 0., -1.), 0.5, BodyProps::glass(1.5));

    let ball_3 = Sphere::new(
        Point::new(1., 0., -1.),
        0.5,
        BodyProps::metal(Point::new(0.8, 0.6, 0.2), 0.),
    );
    let aspect_ratio = 16. / 9.;
    let look_from = Point::new(1., 1., 5.);
    let look_at = Point::new(0., 0., -1.);
    let camera = Camera::new(
        look_from,
        look_at,
        Point::new(0., 1., 0.),
        20.,
        aspect_ratio,
        0.1,
        5.,
    );
    // let world = BVH::new(vec![&floor, &ball_1, &ball_2, &ball_3]);
    let world = BVH::new(vec![Box::new(floor), Box::new(ball_1)]);
    let mut tracer = Tracer::new(400, (400. / aspect_ratio) as usize, camera, 100, 50);
    tracer.render(&world);
    tracer
}
// #[allow(dead_code)]
// pub fn square() -> Tracer {
//     let mut body_list: Vec<Box<dyn Body>> = vec![];
//     body_list.push(Box::new(Sphere::new(
//         Point::new(0., -100.5, -1.),
//         100.0,
//         BodyProps::matte(Point::new(0.2, 0.2, 0.2)),
//     )));
//     body_list.push(Box::new(Cube::new(
//         Point::new(-1.5, 0., 0.),
//         Point::new(-0.5, 1., 1.),
//         BodyProps::matte(Point::new(0.8, 0.2, 0.2)),
//     )));
//     body_list.push(Box::new(Cube::new(
//         Point::new(0., 0., 0.),
//         Point::new(1., 1., 1.),
//         BodyProps::metal(Point::new(0.7, 0.7, 0.7), 0.0),
//     )));
//     body_list.push(Box::new(Cube::new(
//         Point::new(1.5, 0., 0.),
//         Point::new(2.5, 1., 1.),
//         BodyProps::matte(Point::new(0.5, 0.7, 0.2)),
//     )));
//     let aspect_ratio = 16. / 9.;
//     let look_from = Point::new(4., 3., 5.);
//     let look_at = Point::new(0., 0., 0.);
//     let camera = Camera::new(
//         look_from,
//         look_at,
//         Point::new(0., 1., 0.),
//         20.,
//         aspect_ratio,
//         0.1,
//         5.,
//     );

//     let world = World::new(body_list);
//     Tracer::new(400, (400. / aspect_ratio) as usize, camera, world, 100, 50)
// }

// #[allow(dead_code)]
// pub fn book_cover() -> Tracer {
//     let mut body_list: Vec<Box<dyn Body>> = vec![];
//     body_list.push(Box::new(Sphere::new(
//         Point::new(0., -1000., -1.),
//         1000.0,
//         BodyProps::matte(Point::new(0.5, 0.5, 0.5)),
//     )));

//     let mut rng = rand::thread_rng();

//     for a in -11..11 {
//         for b in -11..11 {
//             let location = Point::new(
//                 a as f64 + 0.9 * rng.gen::<f64>(),
//                 0.2,
//                 b as f64 + 0.9 * rng.gen::<f64>(),
//             );
//             match rng.gen::<f64>() {
//                 x if x < 0.8 => {
//                     body_list.push(Box::new(Sphere::new(
//                         location,
//                         0.2,
//                         BodyProps::matte(Point::random() * Point::random()),
//                     )));
//                 }
//                 x if x < 0.95 => {
//                     body_list.push(Box::new(Sphere::new(
//                         location,
//                         0.2,
//                         BodyProps::metal(Point::random(), rng.gen_range(0.5..1.)),
//                     )));
//                 }
//                 _ => {
//                     body_list.push(Box::new(Sphere::new(location, 0.2, BodyProps::glass(1.5))));
//                 }
//             }
//         }
//     }

//     body_list.push(Box::new(Sphere::new(
//         Point::new(0., 1., 0.),
//         1.,
//         BodyProps::glass(1.5),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(-4., 1., 0.),
//         1.,
//         BodyProps::matte(Point::new(0.8, 0.8, 0.)),
//     )));

//     body_list.push(Box::new(Sphere::new(
//         Point::new(4., 1., 0.),
//         1.,
//         BodyProps::metal(Point::new(0.7, 0.6, 0.5), 0.),
//     )));
//     let aspect_ratio = 3. / 2.;
//     let image_width: usize = 1200;
//     let image_height = (1200. / aspect_ratio) as usize;
//     let look_from = Point::new(13., 2., 3.);
//     let look_at = Point::new(0., 0., 0.);
//     let camera = Camera::new(
//         look_from,
//         look_at,
//         Point::new(0., 1., 0.),
//         20.,
//         aspect_ratio,
//         0.1,
//         10.,
//     );

//     let world = World::new(body_list);
//     Tracer::new(image_width, image_height, camera, world, 500, 50)
// }
// #[allow(dead_code)]
// pub fn phone_wall_debug_paper() -> Tracer {
//     let mut body_list: Vec<Box<dyn Body>> = vec![];

//     body_list.push(Box::new(Sphere::new(
//         Point::new(0., -1000.5, -1.),
//         1000.0,
//         BodyProps::matte(Point::new(0.2, 0.2, 0.2)),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(0., 0., -1.),
//         0.5,
//         BodyProps::matte(Point::new(0.1, 0.2, 0.5)),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(-1., 0., -1.),
//         0.5,
//         BodyProps::glass(-1.5),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(-1., 0., -2.),
//         0.5,
//         BodyProps::matte(Point::new(0.1, 0.2, 0.5)),
//     )));

//     body_list.push(Box::new(Sphere::new(
//         Point::new(1., 0., -1.),
//         0.5,
//         BodyProps::metal(Point::new(0.8, 0.6, 0.2), 0.),
//     )));
//     let image_width: usize = 720;
//     let image_height: usize = 1480;
//     let aspect_ratio = image_width as f64 / image_height as f64;
//     let look_from = Point::new(3., 10., 20.);
//     let look_at = Point::new(0., 0., 0.);
//     let camera = Camera::new(
//         look_from,
//         look_at,
//         Point::new(0., 1., 0.),
//         20.,
//         aspect_ratio,
//         0.1,
//         20.,
//     );

//     let world = World::new(body_list);
//     Tracer::new(image_width, image_height, camera, world, 100, 50)
// }

// #[allow(dead_code)]
// pub fn phone_wallpaper() -> Tracer {
//     let mut body_list: Vec<Box<dyn Body>> = vec![];

//     body_list.push(Box::new(Sphere::new(
//         Point::new(0., -1000., 0.),
//         1000.0,
//         BodyProps::metal(Point::new(0.7, 0.6, 0.5), 0.0),
//     )));

//     body_list.push(Box::new(Sphere::new(
//         Point::new(0., 1.8, 0.),
//         1.8,
//         BodyProps::metal(Point::new(0.1, 0.2, 0.5), 0.3),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(1.5, 0.8, 5.),
//         0.8,
//         BodyProps::matte(Point::new(0.7, 0.2, 0.5)),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(-2., 1.5, 4.),
//         1.5,
//         BodyProps::metal(Point::new(0.8, 0.6, 0.2), 0.),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(-2., 0.4, 5.8),
//         0.4,
//         BodyProps::glass(1.5),
//     )));

//     body_list.push(Box::new(Cube::new_as_sphere(
//         Point::new(-2.3, 0.15, 6.3),
//         0.15,
//         BodyProps::matte(Point::new(0.0, 0.1, 0.1)),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(2., 0.3, 5.9),
//         0.3,
//         BodyProps::metal(Point::new(0.8, 0.8, 0.8), 0.),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(1., 1.5, 3.),
//         1.5,
//         BodyProps::glass(1.5),
//     )));
//     body_list.push(Box::new(Cube::new_as_sphere(
//         Point::new(0.6, 0.4, 6.),
//         0.4,
//         BodyProps::matte(Point::new(0.3, 0.6, 0.4)),
//     )));

//     body_list.push(Box::new(Sphere::new(
//         Point::new(-1.1, 0.5, -12.1),
//         0.6,
//         BodyProps::matte(Point::new(0.9, 0.2, 0.2)),
//     )));
//     body_list.push(Box::new(Sphere::new(
//         Point::new(-1.6, 0.4, -11.),
//         0.5,
//         BodyProps::metal(Point::new(0.5, 0.8, 0.5), 0.2),
//     )));
//     body_list.push(Box::new(Cube::new_as_sphere(
//         Point::new(-0.8, 0.2, -11.0),
//         0.3,
//         BodyProps::matte(Point::new(0.3, 0.2, 0.7)),
//     )));

//     body_list.push(Box::new(Sphere::new(
//         Point::new(-2., 0.3, 5.8),
//         0.4,
//         BodyProps::glass(1.5),
//     )));

//     let image_width: usize = 720;
//     let image_height: usize = 1480;
//     let aspect_ratio = image_width as f64 / image_height as f64;
//     let look_from = Point::new(-4., 20., 35.);
//     let look_at = Point::new(0., 0., 0.);
//     let camera = Camera::new(
//         look_from,
//         look_at,
//         Point::new(0., 1., 0.),
//         20.,
//         aspect_ratio,
//         0.1,
//         30.,
//     );

//     let world = World::new(body_list);
//     Tracer::new(image_width, image_height, camera, world, 500, 50)
// }
