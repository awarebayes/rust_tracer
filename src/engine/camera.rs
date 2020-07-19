use crate::data::export::Vector;
use crate::engine::export::Ray;

pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    lens_radius: f64,
    u: Vector,
    // v: Vector,
    // w: Vector,
}

impl Camera {
    pub fn new(
        look_from: Vector,
        look_at: Vector,
        vup: Vector,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewpoint_height = 2.0 * h;
        let viewpoint_width = aspect_ratio * viewpoint_height;

        let w = Vector::unit_vector(&(look_from - look_at));
        let u = Vector::unit_vector(&Vector::cross(&vup, &w));
        let v = Vector::cross(&w, &u);

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
            // v,
            // w,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vector::random_in_unit_disk();
        let offset = self.u * rd.x() + v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}

// unsafe impl Sync for Camera {}
