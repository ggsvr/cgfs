#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[inline(always)]
pub const fn color(r: u8, g: u8, b: u8) -> Color {
    Color::new(r, g, b)
}

impl Color {
    pub const CHANNELS: u8 = 3;
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r, g, b
        }
    }
}

impl From<[u8; 3]> for Color {
    fn from(a: [u8; 3]) -> Self {
        Color::new(a[0], a[1], a[2])
    }
}
impl From<(u8, u8, u8)> for Color {
    fn from(a: (u8, u8, u8)) -> Self {
        Color::new(a.0, a.1, a.2)
    }
}

impl std::ops::Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color::new(
            self.r.saturating_add(rhs.r),
            self.g.saturating_add(rhs.g),
            self.b.saturating_add(rhs.b),
        )
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(
            self.r.saturating_sub(rhs.r),
            self.g.saturating_sub(rhs.g),
            self.b.saturating_sub(rhs.b),
        )
    }
}

impl std::ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Color) {
        *self = *self - rhs;
    }
}

macro_rules! op_int {
    ($t:ty) => {
        impl std::ops::Mul<$t> for Color {
            type Output = Color;
            fn mul(self, rhs: $t) -> Self::Output {
                Color::new(
                    ((self.r as $t).saturating_mul(rhs)).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    ((self.g as $t).saturating_mul(rhs)).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    ((self.b as $t).saturating_mul(rhs)).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                )
            }
        }

        impl std::ops::MulAssign<$t> for Color {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        impl std::ops::Div<$t> for Color {
            type Output = Color;
            fn div(self, rhs: $t) -> Self::Output {
                Color::new(
                    ((self.r as $t).saturating_div(rhs)).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    ((self.g as $t).saturating_div(rhs)).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    ((self.b as $t).saturating_div(rhs)).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                )
            }
        }

        impl std::ops::DivAssign<$t> for Color {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }
    };

    ($x:ty, $($y:ty),+) => {
        op_int!($x);
        op_int!($($y),+);
    }
}

macro_rules! op_float {
    ($t:ty) => {
        impl std::ops::Mul<$t> for Color {
            type Output = Color;
            fn mul(self, rhs: $t) -> Self::Output {
                Color::new(
                    (self.r as $t * rhs).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.g as $t * rhs).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.b as $t * rhs).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                )
            }
        }

        impl std::ops::MulAssign<$t> for Color {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }

        impl std::ops::Div<$t> for Color {
            type Output = Color;
            fn div(self, rhs: $t) -> Self::Output {
                Color::new(
                    (self.r as $t / rhs).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.g as $t / rhs).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                    (self.b as $t / rhs).clamp(u8::MIN as $t, u8::MAX as $t) as u8,
                )
            }
        }

        impl std::ops::DivAssign<$t> for Color {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }
    };

    ($x:ty, $($y:ty),+) => {
        op_float!($x);
        op_float!($($y),+);
    }
}

op_int!(u8, u16, u32, u64, u128, i16, i32, i64, i128);
op_float!(f32, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_math() {
        assert_eq!(color(255, 0, 0) + color(0, 255, 127), color(255, 255, 127));
        assert_eq!(color(255, 0, 0) + color(255, 255, 0), color(255, 255, 0));
        assert_eq!(color(255, 50, 10) * 3, color(255, 150, 30));
        assert_eq!(color(250, 50, 10) / 5, color(50, 10, 2));
        assert_eq!(color(100, 50, 10) * 1.5, color(150, 75, 15));
        assert_eq!(color(100, 50, 10) / 1.5, color(66, 33, 6));
    }
}
