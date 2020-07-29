use crate::data::{Color, vunit, reflect, refract};
use crate::engine::{HitRecord, Ray};
use crate::materials::Material;
use rand::distributions::Open01;
use rand::{thread_rng, Rng};
use std::sync::Arc;


pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
    pub fn share(self) -> Arc<dyn Material> {
        Arc::new(self)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = match record.front_face {
            true => 1.0 / self.ref_idx,
            false => self.ref_idx,
        };

        let unit_direction = vunit(&r_in.direction());
        let mut cos_theta = (-1.0 * unit_direction).dot(&record.normal);
        if cos_theta > 1.0 {
            cos_theta = 1.0
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        let rand_sample: f64 = thread_rng().sample(Open01);

        if etai_over_etat * sin_theta > 1.0 || (rand_sample < reflect_prob) {
            let reflected = reflect(&unit_direction, &record.normal);
            *scattered = Ray::new(record.p, reflected);
            return true;
        }
        let refracted = refract(&unit_direction, &record.normal, etai_over_etat);
        *scattered = Ray::new(record.p, refracted);
        return true;
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
