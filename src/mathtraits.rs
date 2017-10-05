use std::ops::{Add, Sub, Mul, Div, Neg};
use std::f64::consts;
use std::marker;
use super::wectors::{Wector, Point};

pub trait Arithmetic<T>:
Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T>
    where Self: marker::Sized {
}

pub trait Math<T>:
Arithmetic<T> + SquareRoot<T> + Trigonometry<T> +
    Zero<Out = T> + One<Out = T> + PartialEq
    where Self: marker::Sized {
}

impl Arithmetic<f64> for f64 { }
impl Math<f64> for f64 { }

pub trait Components {
    type Out;
    fn x(&self) -> Self::Out;
    fn y(&self) -> Self::Out;
    fn z(&self) -> Self::Out;
}

pub trait SquareRoot<T> {
    fn squareroot(&self) -> T;
}

impl SquareRoot<f64> for f64 {
    fn squareroot(&self) -> f64 {
        self.sqrt()
    }
}

pub trait Trigonometry<T> {
    fn arctangent2(&self, y: T) -> T;
    fn sine(&self) -> T;
    fn cosine(&self) -> T;
    fn tangent(&self) -> T;
    fn pi() -> T;
}

impl Trigonometry<f64> for f64 {
    fn arctangent2(&self, y: f64) -> f64 {
        self.atan2(y)
    }

    fn sine(&self) -> f64 {
        self.sin()
    }

    fn cosine(&self) -> f64 {
        self.cos()
    }

    fn tangent(&self) -> f64 {
        self.tan()
    }

    fn pi() -> f64 {
        consts::PI
    }
}

pub trait Zero {
    type Out;
    fn zero() -> Self::Out;
}

impl Zero for f64 {
    type Out = f64;
    fn zero() -> f64 {
        0.0
    }
}

pub trait One {
    type Out;
    fn one() -> Self::Out;
}

impl One for f64 {
    type Out = f64;
    fn one() -> f64 {
        1.0
    }
}

pub enum Sign {
    Negative,
    Zero,
    Positive
}

pub trait Signed {
    fn sign(&self) -> Sign;
}

impl Signed for f64 {
    fn sign(&self) -> Sign {
        if 0.0 == *self || -0.0 == *self || self.is_nan(){
            Sign::Zero
        } else if self.is_sign_negative() {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }
}

pub trait PointConstruction<T> {
    type Out;
    fn from_triple(a: T, b: T, c: T) -> Self::Out;
}

pub trait Polygon {
    type Out;
    fn area(&self) -> Self::Out;
    fn perimeter(&self) -> Self::Out;
    fn centroid(&self) -> Point<Self::Out>;
    fn normal(&self) -> Wector<Self::Out>;
}

#[derive(Debug, Clone, Copy)]
pub enum PolygonError {
    BadContsruction,
    NonOrthogonal
}

