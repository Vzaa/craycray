use std::ops::{Add, Mul};
use std::iter::Sum;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Color(pub f64, pub f64, pub f64);

pub const RED: Color = Color(1.0, 0.0, 0.0);
pub const GREEN: Color = Color(0.0, 1.0, 0.0);
pub const BLUE: Color = Color(0.0, 0.0, 1.0);
pub const WHITE: Color = Color(1.0, 1.0, 1.0);
pub const BLACK: Color = Color(0.0, 0.0, 0.0);

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let mut r_sum = self.0 + other.0;
        let mut g_sum = self.1 + other.1;
        let mut b_sum = self.2 + other.2;

        if r_sum > 1.0 {
            r_sum = 1.0;
        }

        if g_sum > 1.0 {
            g_sum = 1.0;
        }

        if b_sum > 1.0 {
            b_sum = 1.0;
        }

        Color(r_sum, g_sum, b_sum)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Color {
    pub fn intensity(&self) -> f64 {
        (self.0 + self.1 + self.2) / 3.0
    }

    pub fn col_to_u8(self) -> (u8, u8, u8) {
        ((255.0 * self.0) as u8, (255.0 * self.1) as u8, (255.0 * self.2) as u8)
    }
}

impl Sum for Color {
    fn sum<I>(iter: I) -> Self
        where I: Iterator<Item = Self>
    {
        iter.fold(Color(0.0, 0.0, 0.0), |s, c| s + c)
    }
}
