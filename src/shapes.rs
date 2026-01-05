pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Circle {
    radius: f64,
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidWidth,
    InvalidHeight,
    InvalidRadius,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Result<Self, Error> {
        if width <= 0.0 {
            return Err(Error::InvalidWidth);
        }
        if height <= 0.0 {
            return Err(Error::InvalidHeight)
        }
        Ok (Self { width, height })
    }

    pub fn set_width(&mut self, width: f64) -> Result<(), Error> {
        if width <= 0.0 {
            return Err(Error::InvalidWidth);
        }
        self.width = width;
        Ok(())
    }

    pub fn set_height(&mut self, height: f64) -> Result<(), Error> {
        if height <= 0.0 {
            return Err(Error::InvalidHeight);
        }
        self.height = height;
        Ok(())
    }

    fn get_width(&self) -> f64 {
        self.width
    }

    fn get_height(&self) -> f64 {
        self.height
    }
}

impl Circle {
    pub fn new(radius: f64) -> Result<Self, Error> {
        if radius <= 0.0 {
            return Err(Error::InvalidRadius);
        }
        Ok(Self { radius })
    }

    pub fn set_radius(&mut self, radius: f64) -> Result<(), Error> {
        if radius <= 0.0 {
            return Err(Error::InvalidRadius);
        }
        self.radius = radius;
        Ok(())
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width * self.height)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    
    }
}