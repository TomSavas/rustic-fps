use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec2f {
    x: f32,
    y: f32
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f{ x, y }
    }

    pub fn x(&self) -> &f32 {
        &self.x
    }

    pub fn y(&self) -> &f32 {
        &self.y
    }

    pub fn sqr_len(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.len();
        Vec2f { x: self.x / len, y: self.y / len }
    }

    /// Positive angle denotes counter-clockwise rotation
    pub fn rotate(self, angle_deg: f32) -> Self {
        let angle_rad = angle_deg.to_radians();
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        
        Vec2f { x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos 
        }
    }   
}

impl PartialEq for Vec2f {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl ops::Neg for Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Self::Output {
        Vec2f { x: -self.x, y: -self.y }
    }
}

impl ops::Neg for &Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Self::Output {
        Vec2f { x: -self.x, y: -self.y }
    }
}

impl ops::Add for Vec2f {
    type Output = Vec2f;

    fn add(self, other: Self) -> Self::Output {
        Vec2f { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Add<&Vec2f> for Vec2f {
    type Output = Vec2f;

    fn add(self, other: &Vec2f) -> Self::Output {
        Vec2f { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Add for &Vec2f {
    type Output = Vec2f;

    fn add(self, other: Self) -> Self::Output {
        Vec2f { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Add<Vec2f> for &Vec2f {
    type Output = Vec2f;

    fn add(self, other: Vec2f) -> Self::Output {
        Vec2f { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, other: Self) -> Self::Output {
        Vec2f { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::Sub<&Vec2f> for Vec2f {
    type Output = Vec2f;

    fn sub(self, other: &Vec2f) -> Self::Output {
        Vec2f { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::Sub for &Vec2f {
    type Output = Vec2f;

    fn sub(self, other: Self) -> Self::Output {
        Vec2f { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::Sub<Vec2f> for &Vec2f {
    type Output = Vec2f;

    fn sub(self, other: Vec2f) -> Self::Output {
        Vec2f { x: self.x - other.x, y: self.y - other.y }
    }
}

impl ops::Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, other: f32) -> Self::Output {
        Vec2f { x: self.x * other, y: self.y * other }
    }
}

impl ops::Mul<f32> for &Vec2f {
    type Output = Vec2f;

    fn mul(self, other: f32) -> Self::Output {
        Vec2f { x: self.x * other, y: self.y * other }
    }
}

impl ops::Div<f32> for Vec2f {
    type Output = Vec2f;

    fn div(self, other: f32) -> Self::Output {
        Vec2f { x: self.x / other, y: self.y / other }
    }
}

impl ops::Div<f32> for &Vec2f {
    type Output = Vec2f;

    fn div(self, other: f32) -> Self::Output {
        Vec2f { x: self.x / other, y: self.y / other }
    }
}
