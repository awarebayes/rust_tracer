pub mod metal;
pub mod lambertian;
pub mod material;
pub mod dielectric;

pub use crate::materials::metal::Metal;
pub use crate::materials::lambertian::Lambertian;
pub use crate::materials::dielectric::Dielectric;
pub use crate::materials::material::Material;