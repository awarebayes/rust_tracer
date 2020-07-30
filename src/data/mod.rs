pub mod color;
pub mod utils;
pub mod vector;
pub use crate::data::color::Color;
// pub use crate::data::vector::Vector;
pub use crate::data::utils::{
    rand_float, rand_float01, rand_int 
};

pub use crate::data::vector::{
    vrandom, vrandom_in_unit_disk,
    vrandom_in_unit_sphere, vrandom_range, vunit, vlen,
    reflect, refract
};