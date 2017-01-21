extern crate time;
extern crate kiss3d;
extern crate nalgebra as na;

use na::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use time::precise_time_ns;

fn main() {
    let mut window = Window::new("Kiss3d: lines");
    window.set_light(Light::StickToCamera);
    let mut total_time = 0u64;
    let mut count = 0usize;
    while window.render() {
        let start = precise_time_ns();
        let a = Point3::<f32>::new(-0.1, -0.1, 0.0);
        let b = Point3::<f32>::new(0.0, 0.1, 0.0);
        let c = Point3::<f32>::new(0.1, -0.1, -0.0);
        window.draw_line(&a, &b, &Point3::new(1.0, 0.0, 0.0));
        window.draw_line(&b, &c, &Point3::new(0.0, 1.0, 0.0));
        window.draw_line(&c, &a, &Point3::new(0.0, 0.0, 1.0));
        let elapsed = precise_time_ns() - start;
        total_time = total_time + elapsed;
        count = count + 1;
        if count == 10 {
            println!("elapsed time: {} ms", (total_time as f64)/1000f64);
            total_time = 0;
            count = 0
        }
    }
}