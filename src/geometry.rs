use std::iter::Sum;
use std::cmp::{Ord, Ordering};
use std::cmp::Ordering::{Less, Greater, Equal};
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::f64::consts;

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

#[derive(Clone, Debug)]
pub struct Point<T> {
    x: T,
    y: T,
    z: T
}

impl<T> Point<T> {
    pub fn new(a: T, b: T, c: T) -> Point<T> {
        Point { x: a, y: b, z: c }
    }
}

impl<T: Clone> PointConstruction<T> for Point<T> {
    type Out = Point<T>;
    fn from_triple(a: T, b: T, c: T) -> Point<T> {
        Point::new(a, b, c)
    }
}

impl<T: Add<Output = T> + Clone> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, other: Point<T>) -> Point<T> {
        Point { x: self.x() + other.x(), y: self.y() + other.y(), z: self.z() + other.z() }
    }
}

impl<T: Div<Output = T> + Clone> Point<T> {
    pub fn div_by(&self, other: &T) -> Point<T> {
        Point { x: self.x() / other.clone(), y: self.y() / other.clone(),
                z: self.z() / other.clone() }
    }
}

impl<T: Clone> Components for Point<T> {
    type Out = T;
    fn x(&self) -> T {
        self.x.clone()
    }

    fn y(&self) -> T {
        self.y.clone()
    }

    fn z(&self) -> T {
        self.z.clone()
    }

}

impl<T> Point<T> {
    pub fn setvar(&mut self, var: usize, val: T) {
        match var {
            1 => self.x = val,
            2 => self.y = val,
            _ => self.z = val,
        }
    }
}

impl<T: Ord + Eq + PartialOrd + PartialEq> Point<T> {
    fn comp(&self, other: &Point<T>) -> Ordering {
        let xord = self.x.cmp(&other.x);
        let yord = self.y.cmp(&other.y);
        let zord = self.z.cmp(&other.z);
        match xord {
            Less => Less,
            Equal => match yord {
                Less => Less,
                Equal => zord,
                Greater => Greater
            },
            Greater => Greater
        }
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z)
    }
}

impl<T: Eq> Eq for Point<T> {}

impl<T: Ord + Eq> Ord for Point<T> {
    fn cmp(&self, other: &Point<T>) -> Ordering {
        (*self).comp(other)
    }
}

impl<T: Ord + Eq> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
        Some(self.comp(other))
    }
}

impl<T: Sub<Output=T> + Clone + Clone> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> where T: Sub<T>{
        let new = self.clone();
        Point{ x: (new.x() - other.x()),
               y: (new.y() - other.y()),
               z: (new.z() - other.z())}
    }
}

impl<T: Neg<Output=T> + Clone> Neg for Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Point<T> {
        Point { x: -self.x, y: -self.y, z: -self.z }
    }
}

#[derive(Debug, Clone)]
pub struct Wector<T> {
    x: T,
    y: T,
    z: T
}

impl<T: Clone> PointConstruction<T> for Wector<T> {
    type Out = Wector<T>;
    fn from_triple(a: T, b: T, c: T) -> Wector<T> {
        Wector::new(a, b, c)
    }
}

impl<T: Clone> Wector<T> {
    pub fn new(a: T, b: T, c: T) -> Wector<T> {
        Wector { x: a, y: b, z: c }
    }

    pub fn from_point(p: &Point<T>) -> Wector<T> {
        Wector { x: p.x(), y: p.y(), z: p.z() }
    }

    pub fn set_vals(&mut self, a: &T, b: &T, c: &T) {
        self.x = a.clone();
        self.y = b.clone();
        self.z = c.clone();
    }
}

impl<T: Clone> Components for Wector<T> {
    type Out = T;
    fn x(&self) -> T {
        self.x.clone()
    }

    fn y(&self) -> T {
        self.y.clone()
    }

    fn z(&self) -> T {
        self.z.clone()
    }
}

impl<T: Div<Output = T> + Clone> Wector<T> {
    pub fn div_by(&self, other: &T) -> Wector<T> {
        Wector { x: self.x() / other.clone(), y: self.y() / other.clone(),
                 z: self.z() / other.clone() }
    }
}

impl<T: Add<Output = T> + Clone> Add for Wector<T> {
    type Output = Wector<T>;
    fn add(self, v2: Wector<T>) -> Wector<T> where T: Add<T> {
        Wector { x: self.x() + v2.y(), y: self.y() + v2.y(), z: self.z() + v2.z() }
    }
}

impl<T: Clone + Neg<Output = T> + Mul<Output = T> + Add<Output = T> + Sub<Output = T>> Wector<T> {
    pub fn from_pair(p1: Point<T>, p2: Point<T>) -> Wector<T> {
        let diff = p2 - p1;
        Wector{ x: diff.x(), y: diff.y(), z: diff.z }
    }

    pub fn dot(&self, v2: & Wector<T>) -> T {
        self.x() * v2.x() + self.y() * v2.y() + self.z() * v2.z()
    }

    pub fn cross(&self, v2: &Wector<T>) -> Wector<T> {
        /* |i, j, k|
         * |a, b, c|
         * |x, y, z| = (bz - cy)i - (az - cx)j - (ay - bx)k 
         */
        let a = self.y() * v2.z() - self.z() * v2.y();
        let b = -(self.x() * v2.z() - self.z() * v2.x());
        let c = self.x() * v2.y() - self.y() * v2.x();
        Wector::new(a, b, c)
    }
}

impl<T:SquareRoot<T> + Mul<Output = T> + Clone + Add<Output = T>> Wector<T> {
    pub fn norm(&self) -> T {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).squareroot()
    }
}

impl<T: Neg<Output=T> + Clone + PartialEq + PartialOrd> Neg for Wector<T> {
    type Output = Wector<T>;

    fn neg(self) -> Wector<T> {
        Wector { x: -self.x(), y: -self.y(), z: -self.z() }
    }
}

impl<T: PartialEq + PartialOrd + Clone> PartialEq for Wector<T> {
    fn eq(&self, other: &Wector<T>) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl<T: Ord + Eq + Clone> PartialOrd for Wector<T> {
    fn partial_cmp(&self, other: &Wector<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + PartialOrd + Clone> Eq for Wector<T> {}

impl<T: Eq + PartialOrd + Ord + Clone> Ord for Wector<T>{
    fn cmp(&self, other: &Wector<T>) -> Ordering {
        let xord = self.x.cmp(&other.x);
        let yord = self.y.cmp(&other.y);
        let zord = self.z.cmp(&other.z);
        match xord {
            Less => Less,
            Equal => match yord {
                Less => Less,
                Equal => zord,
                Greater => Greater
            },
            Greater => Greater
        }
    }
}

impl<T: Sub<Output = T> + Clone> Sub for Wector<T> {
    type Output = Wector<T>;

    fn sub(self, v2: Wector<T>) -> Wector<T> where T: Sub<T>{
        Wector { x: self.x() - v2.x(), y: self.y() - v2.y(), z: self.z() - v2.z() }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Coordinates {
    Cartesian,
    Cylindrical,
    Spherical
}

impl Coordinates {
    pub fn into_cartesian<U: Components<Out = T> + Clone + PointConstruction<T, Out = U>,
                          T: Clone + Trigonometry<T> + Mul<Output = T>>(self, b: &U) -> U {
        match self {
            Coordinates::Cartesian => b.clone(),
            Coordinates::Cylindrical => {
                /* x = rcosine(theta), y = rsine(theta), z = z */
                let x = b.x() * b.y().cosine();
                let y = b.x() * b.y().sine();
                U::from_triple(x, y, b.z())
            },
            Coordinates::Spherical => {
                /* x = rsine(phi)cosine(theta), y = rsine(phi)sine(theta), z = rcosine(phi)*/
                let x = b.x() * b.y().cosine() * b.z().sine();
                let y = b.x() * b.y().sine() * b.z().sine();
                let z = b.x() * b.z().cosine();
                U::from_triple(x, y, z)
            }
        }
    }
    
    pub fn into_cylindrical<U: Components<Out = T> + Clone + PointConstruction<T, Out = U>,
                            T: Clone + Mul<Output = T> + Add<Output = T> +SquareRoot<T> + 
                            Trigonometry<T>>(self, b: &U) -> U{
        match self {
            Coordinates::Cartesian => {
                /* r = squareroot(x^2 + y^2), theta = arctangent2(y/x), z = z */
                let r = (b.x() * b.x() + b.y() * b.y()).squareroot();
                let theta = b.y().arctangent2(b.x());
                U::from_triple(r, theta, b.z())
            },
            Coordinates::Cylindrical => b.clone(),
            Coordinates::Spherical => {
                /* r = rho * sine(phi), theta = theta, z = rho * cosine(phi)*/
                let r = b.x() * b.z().sine();
                let z = b.x() * b.z().cosine();
                U::from_triple(r, b.y(), z)
            }
        }
    }

    pub fn into_spherical<U: Components<Out = T> + Clone + PointConstruction<T, Out = U>,
                          T: Clone + Mul<Output = T> + Add<Output = T> +SquareRoot<T> + 
                          Trigonometry<T>>(self, b: &U) -> U{
        match self {
            Coordinates::Cartesian => {
                let xsqr = b.x() * b.x();
                let ysqr = b.y() * b.y();
                let rho = (xsqr.clone() + ysqr.clone() + b.z() * b.z()).squareroot();
                let theta =  b.y().arctangent2(b.x());
                let phi = (xsqr + ysqr).squareroot().arctangent2(b.z());
                U::from_triple(rho, theta, phi)
            },
            Coordinates::Cylindrical => {
                let rho = (b.x() * b.x() + b.z() * b.z()).squareroot();
                let phi = b.x().arctangent2(b.z());
                U::from_triple(rho, b.y(), phi)
            },
            Coordinates::Spherical => b.clone()
        }
    }
}

pub trait Polygon {
    type Out;
    fn area(&self) -> Self::Out;
    fn perimeter(&self) -> Self::Out;
    fn centroid(&self) -> Point<Self::Out>;
}

pub struct Triangle<T> {
    coords: Coordinates,
    pos: Vec<Point<T>>
}

pub fn parameterization<T: Clone + Neg<Output = T> + Mul<Output = T> + Add<Output = T> + Trigonometry<T> + Sub<Output = T> +SquareRoot<T>>
    (s: Coordinates, u: Coordinates, v: &Vec<Point<T>>) -> Vec<Point<T>> {
    v.iter().map(|p| match s {
        Coordinates::Cartesian => u.into_cartesian(p),
        Coordinates::Cylindrical => u.into_cylindrical(p),
        Coordinates::Spherical => u.into_spherical(p)
    }).collect()
}

impl<T: Clone + Neg<Output = T> + Mul<Output = T> + Add<Output = T> + Trigonometry<T> + Sub<Output = T> +SquareRoot<T>> Triangle<T> {
    pub fn new(a: Point<T>, b: Point<T>, c: Point<T>, s: Coordinates) -> Triangle<T> {
        Triangle {
            coords: s,
            pos: vec!(a.clone(), b.clone(), c.clone())
        }
    }
    pub fn reparm(&self, s: Coordinates) -> Vec<Point<T>> {
        parameterization(self.coords, s, &self.pos)
    }

    pub fn deparm(&self, s: Coordinates) -> Vec<Point<T>> {
        parameterization(s, self.coords, &self.pos)
    }

    pub fn segments(&self) -> Vec<Wector<T>> {
        let reparm_pos = self.reparm(Coordinates::Cartesian);
        let ab = Wector::from_pair(reparm_pos[0].clone(), reparm_pos[1].clone());
        let bc = Wector::from_pair(reparm_pos[1].clone(), reparm_pos[2].clone());
        let ca = Wector::from_pair(reparm_pos[2].clone(), reparm_pos[0].clone());
        vec!(ab, bc, ca)
    }
}

impl<T: Clone + One<Out = T> + Zero<Out = T> + Div<Output = T> + Add<Output = T> + Neg<Output = T> +SquareRoot<T> +
     Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Trigonometry<T>> Polygon for Triangle<T> {
    type Out = T;
    fn area(&self) -> T {
        let segs = self.segments();
        let two: T = T::one() + T::one();
        let ac = Wector::new(T::zero(), T::zero(), T::zero()) - segs[2].clone();
        (segs[0].cross(&ac)).norm() / two
    }

    fn perimeter(&self) -> T {
        let segs = self.segments();
        segs[0].norm() + segs[1].norm() + segs[2].norm()
    }

    fn centroid(&self) -> Point<T> {
        let three = T::one() + T::one() + T::one();
        let pos = self.reparm(Coordinates::Cartesian);
        let c = (pos[0].clone() + pos[1].clone() + pos[2].clone()).div_by(&three);
        c
    }
}

pub struct Rectangle<T> {
    coords: Coordinates,
    pos: Vec<Point<T>>
}

pub struct Circle<T> {
    coords: Coordinates,
    pos: Vec<Point<T>>
}
