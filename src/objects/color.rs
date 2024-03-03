use std::ops;

const MAX_VALUE: u8 = u8::MAX;
const MAX_VALUE_F: f32 = MAX_VALUE as f32;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32
}


impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: (r as f32) / MAX_VALUE_F,
            green: (g as f32) / MAX_VALUE_F,
            blue: (b as f32) / MAX_VALUE_F
        }
    }

    pub fn from_rgb_f(r: f64, g: f64, b: f64) -> Color {
        Color {
            red: Color::trim_component(r as f32),
            green: Color::trim_component(g as f32),
            blue: Color::trim_component(b as f32)
        }
    }

    pub fn red(self) -> u8 {
        (self.red * MAX_VALUE_F) as u8
    }

    pub fn green(self) -> u8 {
        (self.green * MAX_VALUE_F) as u8
    }

    pub fn blue(self) -> u8 {
        (self.blue * MAX_VALUE_F) as u8
    }

    fn trim_component(val: f32) -> f32 {
        if val > 1.0 {
            return 1.0;
        }
        
        if val < 0.0 {
            return  0.0;
        }

        return  val;
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            red: Color::trim_component(self.red * rhs),
            green: Color::trim_component(self.green * rhs),
            blue: Color::trim_component(self.blue * rhs),
        }
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        self * rhs as f32
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self as f32
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: Color::trim_component(self.red + rhs.red),
            green: Color::trim_component(self.green + rhs.green),
            blue: Color::trim_component(self.blue + rhs.blue)
        }
    }
}
