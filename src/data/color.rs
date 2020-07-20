use nalgebra::Vector3;
use image::Rgba;
use std::cmp::min;
use std::iter::Sum;
use std::ops;
use crate::data::vrandom;

const RGB_MULT: f64 = 255.99;
const RGB_MAX: u64 = 255;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn from_vector(v: &Vector3<f64>) -> Color {
        Color::new(v[0], v[1], v[2])
    }

    pub fn from_rgb(r: u32, g: u32, b: u32) -> Color {
        Color {
            r: (r as f64) / 256.0,
            g: (g as f64) / 256.0,
            b: (b as f64) / 256.0,
        }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn len(&self) -> f64 {
        (&self).len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.r * self.r + self.g * self.g + self.b * self.b
    }

    pub fn unit_vector(&self) -> Color {
        let k = 1.0 / self.len();
        Color {
            r: self.r * k,
            g: self.g * k,
            b: self.b * k,
        }
    }

    pub fn dot(lhs: &Color, rhs: &Color) -> f64 {
        lhs.r * rhs.r + lhs.g * rhs.g + lhs.b * rhs.b
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        assert!(0.0 <= self.r, "Found negative r {}", self.r);
        assert!(0.0 <= self.g, "Found negative g {}", self.g);
        assert!(0.0 <= self.b, "Found negative b {}", self.b);

        let r = (RGB_MULT * self.r) as u64;
        let g = (RGB_MULT * self.g) as u64;
        let b = (RGB_MULT * self.b) as u64;

        // Lights can be brighter than (1.0, 1.0, 1.0) so we must cap to max value for RGB
        let r = min(r, RGB_MAX) as u8;
        let g = min(g, RGB_MAX) as u8;
        let b = min(b, RGB_MAX) as u8;

        Rgba([r, g, b, 255])
    }

    pub fn gamma_2(self) -> Color {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub fn normalize_samples(&self, n_samples: u64) -> Color {
        let scale = 1.0 / n_samples as f64;
        Color {
            r: (self.r() * scale).sqrt(),
            g: (self.g() * scale).sqrt(),
            b: (self.b() * scale).sqrt(),
        }
    }

    pub fn check_not_nan(&self) -> bool {
        if self.r().is_nan() {
            return true;
        }
        if self.g().is_nan() {
            return true;
        }
        if self.b().is_nan() {
            return true;
        }

        return false;
    }

    pub fn random() -> Color {
        Color::from_vector(&vrandom())
    }
}

impl From<&Rgba<u8>> for Color {
    fn from(rgb: &Rgba<u8>) -> Self {
        Color {
            r: f64::from(rgb[0]) / RGB_MULT,
            g: f64::from(rgb[1]) / RGB_MULT,
            b: f64::from(rgb[2]) / RGB_MULT,
        }
    }
}

impl Sum<Color> for Color {
    fn sum<I: Iterator<Item = Color>>(iter: I) -> Color {
        let mut sum = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        for color in iter {
            sum = sum + color;
        }
        sum
    }
}

fn add_colors(lhs: &Color, rhs: &Color) -> Color {
    Color {
        r: lhs.r + rhs.r,
        g: lhs.g + rhs.g,
        b: lhs.b + rhs.b,
    }
}

fn mul_colors(lhs: &Color, rhs: &Color) -> Color {
    Color {
        r: lhs.r * rhs.r,
        g: lhs.g * rhs.g,
        b: lhs.b * rhs.b,
    }
}

fn add_color_and_scalar(color: &Color, scalar: f64) -> Color {
    Color {
        r: color.r + scalar,
        g: color.g + scalar,
        b: color.b + scalar,
    }
}

fn mul_color_and_scalar(color: &Color, scalar: f64) -> Color {
    Color {
        r: color.r * scalar,
        g: color.g * scalar,
        b: color.b * scalar,
    }
}

fn div_color_and_scalar(color: &Color, scalar: f64) -> Color {
    Color {
        r: color.r / scalar,
        g: color.g / scalar,
        b: color.b / scalar,
    }
}

impl ops::Add<&Color> for &Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Color {
        add_colors(self, rhs)
    }
}

impl ops::Add<Color> for &Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        add_colors(self, &rhs)
    }
}

impl ops::Add<&Color> for Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Color {
        add_colors(&self, rhs)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        add_colors(&self, &rhs)
    }
}

impl ops::Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        mul_colors(self, rhs)
    }
}

impl ops::Mul<Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        mul_colors(self, &rhs)
    }
}

impl ops::Mul<&Color> for Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        mul_colors(&self, rhs)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        mul_colors(&self, &rhs)
    }
}

impl ops::Add<f64> for &Color {
    type Output = Color;

    fn add(self, rhs: f64) -> Color {
        add_color_and_scalar(self, rhs)
    }
}

impl ops::Add<&Color> for f64 {
    type Output = Color;

    fn add(self, rhs: &Color) -> Color {
        add_color_and_scalar(rhs, self)
    }
}

impl ops::Add<f64> for Color {
    type Output = Color;

    fn add(self, rhs: f64) -> Color {
        add_color_and_scalar(&self, rhs)
    }
}

impl ops::Add<Color> for f64 {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        add_color_and_scalar(&rhs, self)
    }
}

impl ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        mul_color_and_scalar(self, rhs)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        mul_color_and_scalar(&self, rhs)
    }
}

impl ops::Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        mul_color_and_scalar(rhs, self)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        mul_color_and_scalar(&rhs, self)
    }
}

impl ops::Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        div_color_and_scalar(self, rhs)
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        div_color_and_scalar(&self, rhs)
    }
}
