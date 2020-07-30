use nalgebra::Vector3;

// crate imports
use crate::data::Color;
use crate::data::{rand_float, rand_float01, vlen, vrandom_range};
use crate::engine::{Hittable, HittableList, Sphere, XYRect};
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::textures::{CheckerTexture, ImageTexture, NoiseTexture, Texture};

pub fn random_world() -> HittableList {
    let mut world = HittableList::new();

    let checker = CheckerTexture::new(Color::new(0.5, 0.5, 0.5), Color::new(0.1, 0.1, 0.1)).share();
    let ground_matrial = Lambertian::from_texture(checker).share();
    world.add(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, ground_matrial).share());

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_material = rand_float01();
            let center = Vector3::new(a + 0.9 * rand_float01(), 0.2, b + 0.9 * rand_float01());

            if vlen(&(center - Vector3::new(4.0, 0.2, 0.0))) <= 0.9 {
                continue;
            }
            if choose_material < 0.8 {
                let albedo = Color::random() * Color::random();
                let sphere_material = Lambertian::from_color(albedo).share();
                world.add(Sphere::new(center, 0.2, sphere_material).share())
            } else if choose_material < 0.95 {
                let albedo = Color::from_vector(&vrandom_range(0.5, 1.0));
                let fuzz = rand_float(0.0, 0.5);
                let sphere_material = Metal::new(albedo, fuzz).share();
                world.add(Sphere::new(center, 0.2, sphere_material).share())
            } else {
                let sphere_material = Dielectric::new(1.5).share();
                world.add(Sphere::new(center, 0.2, sphere_material).share());
            }
        }
    }

    let material1 = Dielectric::new(1.5).share();
    world.add(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, material1).share());

    let material2 = Lambertian::from_color(Color::new(0.4, 0.2, 0.1)).share();
    world.add(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2).share());

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0).share();
    world.add(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, material3).share());

    return world;
}

pub fn two_spheres_checker() -> HittableList {
    let mut objects = HittableList::new();

    let checker = CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)).share();
    let lamb = Lambertian::from_texture(checker).share();
    objects.add(Sphere::new(Vector3::new(0.0, -10.0, 0.0), 10.0, lamb.clone()).share());
    objects.add(Sphere::new(Vector3::new(0.0, 10.0, 0.0), 10.0, lamb.clone()).share());
    return objects;
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = NoiseTexture::new(2.0).share();
    let lamb = Lambertian::from_texture(pertext.clone()).share();
    objects.add(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, lamb.clone()).share());
    objects.add(Sphere::new(Vector3::new(0.0, 2.0, 0.0), 2.0, lamb.clone()).share());

    return objects;
}

pub fn earth() -> HittableList {
    let earth_texture = ImageTexture::new(
        "/home/mikew/Documents/Programming/rust/rust_tracer/res/earthmap.jpg".to_string(),
    )
    .share();
    let earth_surface = Lambertian::from_texture(earth_texture).share();
    let globe = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, earth_surface).share();
    let mut world = HittableList::new();
    world.add(globe.clone());
    return world;
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = NoiseTexture::new(2.0).share();
    let lamb = Lambertian::from_texture(pertext.clone()).share();
    let diff_light = DiffuseLight::from_color(Color::new(0.5, 1.0, 100.0)).share();
    objects.add(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, lamb.clone()).share());
    objects.add(Sphere::new(Vector3::new(0.0, 2.0, 0.0), 2.0, diff_light.clone()).share());

    objects.add(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, diff_light.clone()).share());
    return objects;
}
