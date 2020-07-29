use crate::engine::Ray;
use nalgebra::Vector3;
use crate::data::{vrandom_in_unit_disk, vunit};

pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lens_radius: f64,
    u: Vector3<f64>,
    v: Vector3<f64>,
    // v: Vector,
    // w: Vector,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f64>,
        look_at: Vector3<f64>,
        vup: Vector3<f64>,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewpoint_height = 2.0 * h;
        let viewpoint_width = aspect_ratio * viewpoint_height;

        let w = vunit(&(look_from - look_at));
        let u = vunit(&vup.cross(&w));
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewpoint_width * u;
        let vertical = focus_dist * viewpoint_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            // w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vector3<f64> = self.lens_radius * vrandom_in_unit_disk();
        let offset = self.u * rd[0] + self.v * rd[1];

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
