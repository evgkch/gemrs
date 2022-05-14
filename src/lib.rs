//! # Geometry
//!
//! A library to work with geometry

use std::ops::{Add, Sub, Neg, Mul, Div};
use std::cmp::{PartialEq};

/// 2d Vector
#[derive(Debug, Copy, Clone)]
pub struct Vector2<T>(T, T);

/// 2d Point
#[derive(Debug, Copy, Clone)]
pub struct Point2<T>(T, T);

/// (==): &Vector × &Vector -> bool
/// (!=): &Vector × &Vector -> bool
impl<T: PartialEq> PartialEq for Vector2<T> {    
    fn eq(&self, v: &Vector2<T>) -> bool {
        self.0 == v.0 && self.1 == v.1
    }
    fn ne(&self, v: &Vector2<T>) -> bool {
        self.0 != v.0 && self.1 != v.1
    }
}

/// (-): Vector -> Vector
impl<T: Neg<Output=T>> Neg for Vector2<T> {
    type Output = Vector2<T>;
    
    fn neg(self) -> Self::Output {
        Vector2(-self.0, -self.1)
    }
}

/// (+): Vector × Vector -> Vector
impl<T: Add<Output=T>> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, v: Vector2<T>) -> Self::Output {
        Vector2(self.0 + v.0, self.1 + v.1)
    }
}

/// (-): Vector × Vector -> Vector
impl<T: Sub<Output=T>> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, v: Vector2<T>) -> Self::Output {
        Vector2(self.0 - v.0, self.1 - v.1)
    }
}

/// (*): Vector × K -> Vector
/// where K is a ring
impl<T: Mul<Output=T> + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, k: T) -> Self::Output {
        Vector2(self.0 * k, self.1 * k)
    }
}

/// (/): Vector × K -> Vector
/// where K is a ring
impl<T: Div<Output=T> + Copy> Div<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, k: T) -> Self::Output {
        Vector2(self.0 / k, self.1 / k)
    }
}

/// (==): &Point × &Point -> bool
/// (!=): &Point × &Point -> bool
impl<T: PartialEq> PartialEq for Point2<T> {    
    fn eq(&self, p: &Point2<T>) -> bool {
        self.0 == p.0 && self.1 == p.1
    }
    fn ne(&self, p: &Point2<T>) -> bool {
        self.0 != p.0 && self.1 != p.1
    }
}

/// (+): Point × Vector -> Point
impl<T: Add<Output=T>> Add<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;
    
    fn add(self, v: Vector2<T>) -> Self::Output {
        Point2(self.0 + v.0, self.1 + v.1)
    }
}

// (-): Point × Vector -> Point
impl<T: Sub<Output=T>> Sub<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;
    
    fn sub(self, v: Vector2<T>) -> Self::Output {
        Point2(self.0 - v.0, self.1 - v.1)
    }
}

// (-): Point × Point -> Vector
impl<T: Sub<Output=T>> Sub for Point2<T> {
    type Output = Point2<T>;
    
    fn sub(self, p: Point2<T>) -> Self::Output {
        Point2(self.0 - p.0, self.1 - p.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eq_vec2() {
        let v = Vector2(1, 0);
        let w = Vector2(1, 0);
        assert_eq!(&v == &w, true);
        assert_ne!(&v != &w, false);
    }

    #[test]
    fn ne_vec2() {
        let v = Vector2(1, 0);
        let w = Vector2(0, 1);
        assert_eq!(&v == &w, true);
        assert_ne!(&v != &w, false);
    }

    #[test]
    fn neg_vec2() {
        assert_eq!(-Vector2(1, 1), Vector2(-1, -1));
    }

    #[test]
    fn add_sub_vec2() {
        assert_eq!(Vector2(1, 1) + Vector2(2, 3), Vector2(3, 4));
        assert_eq!(Vector2(1, 1) - Vector2(2, 3), Vector2(-1, -2));
    }

    #[test]
    fn mul_div_vec2() {
        assert_eq!(Vector2(1, 1) * 2, Vector2(2, 2));
        assert_eq!(Vector2(1.0, 1.0) / 2.0, Vector2(0.5, 0.5));
    }

    #[test]
    fn eq_point2() {
        let a = Point2(1, 0);
        let b = Point2(1, 0);
        assert_eq!(&a == &b, true);
        assert_ne!(&a != &b, false);
    }

    #[test]
    fn ne_point2() {
        let a = Point2(1, 0);
        let b = Point2(0, 1);
        assert_eq!(&a == &b, true);
        assert_ne!(&a != &b, false);
    }

    #[test]
    fn add_sub_point2_vec2() {
        assert_eq!(Point2(1, 1) + Vector2(2, 3), Point2(3, 4));
        assert_eq!(Point2(1, 1) - Vector2(2, 3), Point2(-1, -2));
    }

    #[test]
    fn sub_point2() {
        assert_eq!(Point2(1, 1) - Point2(2, 2), Point2(-1, -1));
    }
}
