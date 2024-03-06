#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_rgb(self) -> (RGB, u8) {
        (
            RGB {
                r: self.r,
                g: self.g,
                b: self.b,
            },
            self.a,
        )
    }

    pub fn scale_alpha(mut self, scalar: f32) -> Self {
        self.a = (self.a as f32 * scalar).clamp(0.0, 255.0) as u8;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    /// Adds an RGBA value onto a RGB value returning the result.
    /// This simply performs a linear interpolation between the two.
    pub fn add_rgba(self, other: RGBA) -> Self {
        let (other, alpha) = other.to_rgb();
        self.lerp(&other, alpha as f32 / 255.0)
    }

    /// Performs a linear interpolation between two RGB values returning the result.
    pub fn lerp(&self, other: &Self, a: f32) -> Self {
        RGB {
            r: ((1.0 - a) * self.r as f32 + a * other.r as f32) as u8,
            g: ((1.0 - a) * self.g as f32 + a * other.g as f32) as u8,
            b: ((1.0 - a) * self.b as f32 + a * other.b as f32) as u8,
        }
    }
}

//== constants =====

pub const TRANSPARANT: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

pub const BLACK: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub const WHITE: RGBA = RGBA {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

pub const RED: RGBA = RGBA {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

pub const GREEN: RGBA = RGBA {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

pub const BLUE: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};
