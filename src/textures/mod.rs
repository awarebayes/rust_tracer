pub mod texture;
pub mod misc;
pub mod image;

pub use crate::textures::texture::Texture;
pub use crate::textures::misc::{SolidColor, CheckerTexture, NoiseTexture};
// pub use crate::textures::perlin::Perlin;