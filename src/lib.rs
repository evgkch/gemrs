//! # Geometry
//!
//! A library to work with geometry

use std::cmp::{ PartialEq };
use std::ops::{ Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };

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
        self.0 != v.0 || self.1 != v.1
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

/// (+=): Vector × Vector -> Vector
impl<T: Add<Output=T> + Copy> AddAssign for Vector2<T> {
    fn add_assign(&mut self, v: Vector2<T>) {
        *self = Self(self.0 + v.0, self.1 + v.1)
    }
}

/// (-): Vector × Vector -> Vector
impl<T: Sub<Output=T>> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, v: Vector2<T>) -> Self::Output {
        Vector2(self.0 - v.0, self.1 - v.1)
    }
}

/// (-=): Vector × Vector -> Vector
impl<T: Sub<Output=T> + Copy> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, v: Vector2<T>) {
        *self = Self(self.0 - v.0, self.1 - v.1)
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

/// (*=): Vector × K -> Vector
impl<T: Mul<Output=T> + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, k: T) {
        *self = Self(self.0 * k, self.1 * k)
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

/// (/=): Vector × K -> Vector
/// where K is a ring
impl<T: Div<Output=T> + Copy> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, k: T) {
        *self = Self(self.0 / k, self.1 / k)
    }
}

impl<T: Add<Output=T> + Mul<Output=T>> Vector2<T> {
    /// dot: Vector × Vector -> K
    /// where K is a ring
    pub fn dot(self, v: Vector2<T>) -> T {
        self.0 * v.0 + self.1 * v.1
    }
}

/// (==): &Point × &Point -> bool
/// (!=): &Point × &Point -> bool
impl<T: PartialEq> PartialEq for Point2<T> {
    fn eq(&self, p: &Point2<T>) -> bool {
        self.0 == p.0 && self.1 == p.1
    }
    fn ne(&self, p: &Point2<T>) -> bool {
        self.0 != p.0 || self.1 != p.1
    }
}

/// (+): Point × Vector -> Point
impl<T: Add<Output=T>> Add<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn add(self, v: Vector2<T>) -> Self::Output {
        Point2(self.0 + v.0, self.1 + v.1)
    }
}

/// (+=): Point × Vector -> Point
impl<T: Add<Output=T> + Copy> AddAssign<Vector2<T>> for Point2<T> {
    fn add_assign(&mut self, v: Vector2<T>) {
        *self = Self(self.0 + v.0, self.1 + v.1)
    }
}

/// (-): Point × Vector -> Point
impl<T: Sub<Output=T>> Sub<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn sub(self, v: Vector2<T>) -> Self::Output {
        Point2(self.0 - v.0, self.1 - v.1)
    }
}

/// (-=): Point × Vector -> Point
impl<T: Sub<Output=T> + Copy> SubAssign<Vector2<T>> for Point2<T> {
    fn sub_assign(&mut self, v: Vector2<T>) {
        *self = Self(self.0 - v.0, self.1 - v.1)
    }
}

/// (-): Point × Point -> Vector
impl<T: Sub<Output=T>> Sub for Point2<T> {
    type Output = Point2<T>;

    fn sub(self, p: Point2<T>) -> Self::Output {
        Point2(self.0 - p.0, self.1 - p.1)
    }
}

/// 3d Vector
#[derive(Debug, Copy, Clone)]
pub struct Vector3<T>(T, T, T);

/// 3d Point
#[derive(Debug, Copy, Clone)]
pub struct Point3<T>(T, T, T);

/// (==): &Vector × &Vector -> bool
/// (!=): &Vector × &Vector -> bool
impl<T: PartialEq> PartialEq for Vector3<T> {
    fn eq(&self, v: &Vector3<T>) -> bool {
        self.0 == v.0 && self.1 == v.1 && self.2 == v.2
    }
    fn ne(&self, v: &Vector3<T>) -> bool {
        self.0 != v.0 || self.1 != v.1 || self.2 != v.2
    }
}

/// (-): Vector -> Vector
impl<T: Neg<Output=T>> Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        Vector3(-self.0, -self.1, -self.2)
    }
}

/// (+): Vector × Vector -> Vector
impl<T: Add<Output=T>> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, v: Vector3<T>) -> Self::Output {
        Vector3(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}

/// (+=): Vector × Vector -> Vector
impl<T: Add<Output=T> + Copy> AddAssign for Vector3<T> {
    fn add_assign(&mut self, v: Vector3<T>) {
        *self = Self(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}

/// (-): Vector × Vector -> Vector
impl<T: Sub<Output=T>> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, v: Vector3<T>) -> Self::Output {
        Vector3(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
}

/// (-=): Vector × Vector -> Vector
impl<T: Sub<Output=T> + Copy> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, v: Vector3<T>) {
        *self = Self(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
}

/// (*): Vector × K -> Vector
/// where K is a ring
impl<T: Mul<Output=T> + Copy> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, k: T) -> Self::Output {
        Vector3(self.0 * k, self.1 * k, self.2 * k)
    }
}

/// (*=): Vector × K -> Vector
impl<T: Mul<Output=T> + Copy> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, k: T) {
        *self = Self(self.0 * k, self.1 * k, self.2 * k)
    }
}

/// (/): Vector × K -> Vector
/// where K is a ring
impl<T: Div<Output=T> + Copy> Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, k: T) -> Self::Output {
        Vector3(self.0 / k, self.1 / k, self.2 / k)
    }
}

/// (/=): Vector × K -> Vector
/// where K is a ring
impl<T: Div<Output=T> + Copy> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, k: T) {
        *self = Self(self.0 / k, self.1 / k, self.2 / k)
    }
}

impl<T: Add<Output=T> + Mul<Output=T>> Vector3<T> {
    /// dot: Vector × Vector -> K
    /// where K is a ring
    pub fn dot(self, v: Vector3<T>) -> T {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }
}

/// (==): &Point × &Point -> bool
/// (!=): &Point × &Point -> bool
impl<T: PartialEq> PartialEq for Point3<T> {
    fn eq(&self, p: &Point3<T>) -> bool {
        self.0 == p.0 && self.1 == p.1 && self.2 == p.2
    }
    fn ne(&self, p: &Point3<T>) -> bool {
        self.0 != p.0 || self.1 != p.1 || self.2 != p.2
    }
}

/// (+): Point × Vector -> Point
impl<T: Add<Output=T>> Add<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, v: Vector3<T>) -> Self::Output {
        Point3(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}

/// (+=): Point × Vector -> Point
impl<T: Add<Output=T> + Copy> AddAssign<Vector3<T>> for Point3<T> {
    fn add_assign(&mut self, v: Vector3<T>) {
        *self = Self(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}

/// (-): Point × Vector -> Point
impl<T: Sub<Output=T>> Sub<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn sub(self, v: Vector3<T>) -> Self::Output {
        Point3(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
}

/// (-=): Point × Vector -> Point
impl<T: Sub<Output=T> + Copy> SubAssign<Vector3<T>> for Point3<T> {
    fn sub_assign(&mut self, v: Vector3<T>) {
        *self = Self(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
}

/// (-): Point × Point -> Vector
impl<T: Sub<Output=T>> Sub for Point3<T> {
    type Output = Point3<T>;

    fn sub(self, p: Point3<T>) -> Self::Output {
        Point3(self.0 - p.0, self.1 - p.1, self.2 - p.2)
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
        assert_eq!(&v != &w, false);
    }

    #[test]
    fn ne_vec2() {
        let v = Vector2(1, 0);
        let w = Vector2(0, 1);
        assert_eq!(&v == &w, false);
        assert_eq!(&v != &w, true);
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
    fn add_sub_assign_vec2() {
        let mut v = Vector2(1, 1);
        v += Vector2(2, 3);
        assert_eq!(v, Vector2(3, 4));
        v -= Vector2(2, 3) * 2;
        assert_eq!(v, Vector2(-1, -2));
    }

    #[test]
    fn mul_div_vec2() {
        assert_eq!(Vector2(1, 1) * 2, Vector2(2, 2));
        assert_eq!(Vector2(1.0, 1.0) / 2.0, Vector2(0.5, 0.5));
    }

    #[test]
    fn mul_div_assign_vec2() {
        let mut v = Vector2(1.0, 1.0);
        v *= 2.0;
        assert_eq!(v, Vector2(2.0, 2.0));
        v /= 4.0;
        assert_eq!(v, Vector2(0.5, 0.5));
    }

    #[test]
    fn eq_point2() {
        let a = Point2(1, 0);
        let b = Point2(1, 0);
        assert_eq!(&a == &b, true);
        assert_eq!(&a != &b, false);
    }

    #[test]
    fn ne_point2() {
        let a = Point2(1, 0);
        let b = Point2(0, 1);
        assert_eq!(&a == &b, false);
        assert_eq!(&a != &b, true);
    }

    #[test]
    fn add_sub_point2_vec2() {
        assert_eq!(Point2(1, 1) + Vector2(2, 3), Point2(3, 4));
        assert_eq!(Point2(1, 1) - Vector2(2, 3), Point2(-1, -2));
    }

    #[test]
    fn add_sub_assign_point2_vec2() {
        let mut p = Point2(1, 1);
        p += Vector2(2, 3);
        assert_eq!(p, Point2(3, 4));
        p -= Vector2(2, 3) * 2;
        assert_eq!(p, Point2(-1, -2));
    }

    #[test]
    fn sub_point2() {
        assert_eq!(Point2(1, 1) - Point2(2, 2), Point2(-1, -1));
    }

    #[test]
    fn eq_vec3() {
        let v = Vector3(1, 0, -1);
        let w = Vector3(1, 0, -1);
        assert_eq!(&v == &w, true);
        assert_eq!(&v != &w, false);
    }

    #[test]
    fn ne_vec3() {
        let v = Vector3(1, 0, -1);
        let w = Vector3(0, 1, -1);
        assert_eq!(&v == &w, false);
        assert_eq!(&v != &w, true);
    }

    #[test]
    fn neg_vec3() {
        assert_eq!(-Vector3(1, 1, 1), Vector3(-1, -1, -1));
    }

    #[test]
    fn add_sub_vec3() {
        assert_eq!(Vector3(1, 1, 1) + Vector3(2, 3, 4), Vector3(3, 4, 5));
        assert_eq!(Vector3(1, 1, 1) - Vector3(2, 3, 4), Vector3(-1, -2, -3));
    }

    #[test]
    fn add_sub_assign_vec3() {
        let mut v = Vector3(1, 1, 1);
        v += Vector3(2, 3, 4);
        assert_eq!(v, Vector3(3, 4, 5));
        v -= Vector3(2, 3, 4) * 2;
        assert_eq!(v, Vector3(-1, -2, -3));
    }

    #[test]
    fn mul_div_vec3() {
        assert_eq!(Vector3(1, 1, 1) * 2, Vector3(2, 2, 2));
        assert_eq!(Vector3(1.0, 1.0, 1.0) / 2.0, Vector3(0.5, 0.5, 0.5));
    }

    #[test]
    fn mul_div_assign_vec3() {
        let mut v = Vector3(1.0, 1.0, 1.0);
        v *= 2.0;
        assert_eq!(v, Vector3(2.0, 2.0, 2.0));
        v /= 4.0;
        assert_eq!(v, Vector3(0.5, 0.5, 0.5));
    }

    #[test]
    fn eq_point3() {
        let a = Point3(1, 0, -1);
        let b = Point3(1, 0, -1);
        assert_eq!(&a == &b, true);
        assert_eq!(&a != &b, false);
    }

    #[test]
    fn ne_point3() {
        let a = Point3(1, 0, -1);
        let b = Point3(0, 1, -1);
        assert_eq!(&a == &b, false);
        assert_eq!(&a != &b, true);
    }

    #[test]
    fn add_sub_point3_vec3() {
        assert_eq!(Point3(1, 1, 1) + Vector3(2, 3, 4), Point3(3, 4, 5));
        assert_eq!(Point3(1, 1, 1) - Vector3(2, 3, 4), Point3(-1, -2, -3));
    }

    #[test]
    fn add_sub_assign_point3_vec3() {
        let mut p = Point3(1, 1, 1);
        p += Vector3(2, 3, 4);
        assert_eq!(p, Point3(3, 4, 5));
        p -= Vector3(2, 3, 4) * 2;
        assert_eq!(p, Point3(-1, -2, -3));
    }

    #[test]
    fn sub_point3() {
        assert_eq!(Point3(1, 1, 1) - Point3(2, 2, 2), Point3(-1, -1, -1));
    }
}
