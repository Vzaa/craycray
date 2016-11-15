#[derive(Copy, Clone)]
pub struct Color(pub f64, pub f64, pub f64);

pub const RED: Color = Color(1.0, 0.0, 0.0);
pub const GREEN: Color = Color(0.0, 1.0, 0.0);
pub const BLUE: Color = Color(0.0, 0.0, 1.0);
pub const WHITE: Color = Color(1.0, 1.0, 1.0);
pub const BLACK: Color = Color(0.0, 0.0, 0.0);

impl Color {
    pub fn add(a: &Color, b: &Color) -> Color {
        let mut r_sum = a.0 + b.0;
        let mut g_sum = a.1 + b.1;
        let mut b_sum = a.2 + b.2;

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

    pub fn intensity(&self) -> f64 {
        (self.0 + self.1 + self.2) / 3.0
    }

    pub fn scale(a: &Color, b: f64) -> Color {
        Color(a.0 * b, a.1 * b, a.2 * b)
    }

    pub fn col_to_u8(self) -> (u8, u8, u8) {
        ((255.0 * self.0) as u8, (255.0 * self.1) as u8, (255.0 * self.2) as u8)
    }
}
