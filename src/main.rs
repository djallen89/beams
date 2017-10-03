mod geometry {
    use std::cmp::{Ord, Ordering};
    use std::cmp::Ordering::{Less, Greater, Equal};
    use std::ops::{Add, Sub, Mul, Div, Neg};

    pub trait Components {
        type Out;
        fn x(&self) -> Self::Out;
        fn y(&self) -> Self::Out;
        fn z(&self) -> Self::Out;
    }

    pub trait Sqrt<T> {
        fn sqrt(&self) -> T;
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

    #[derive(Debug)]
    pub struct Wector<T> {
        x: T,
        y: T,
        z: T
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

    impl<T: Clone + Mul<Output = T> + Add<Output = T> + Sub<Output = T>> Wector<T> {
        pub fn from_pair(p1: Point<T>, p2: Point<T>) -> Wector<T> {
            let diff = p2 - p1;
            Wector{ x: diff.x(), y: diff.y(), z: diff.z }
        }

        pub fn dot(&self, v2: & Wector<T>) -> T {
            self.x() * v2.x() + self.y() * v2.y() + self.z() * v2.z()
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
    enum Coordinates {
        Cartesian,
        Cylindrical,
        Spherical
    }

    impl Coordinates {
        pub fn into_cartesian<U: Components<Out = T> + Clone,T: Clone>(self, b: &U) -> U {
            match self {
                Coordinates::Cartesian => b.clone(),
                _ => b.clone(),
            }
        }
        
        pub fn into_cylindrical<U: Components<Out = T> + Clone,T: Clone + Mul<Output = T> + Add<Output = T> + Sqrt<T>>(self, b: &U) -> U{
            match self {
                Coordinates::Cartesian | _ => {
                    let r = (b.x()*b.x() + b.y()*b.y()).sqrt();
                }
            }
            b.clone()
        }
    }
    pub struct Triangle<T> {
        coords: Coordinates,
        pos: Vec<Point<T>>
    }

    pub struct Rectangle<T> {
        coords: Coordinates,
        pos: Vec<Point<T>>
    }

    pub struct Circle<T> {
        coords: Coordinates,
        pos: Vec<Point<T>>
    }
}

mod sam {
}

#[cfg(test)]
mod tests {
    use super::geometry::{Point, Wector, Components};

    #[test]
    fn create_new_wectors() {
        let p1 = Point::new(10isize, 3, 3);
        let p2 = Point::new(7isize, 3, -2);
        assert!(p1 > p2);
        assert_eq!(p1.y(), p2.y());
        let w1 = Wector::new(-3isize, 0, -5);
        let w2 = Wector::from_point(&(p2.clone() - p1.clone()));
        let w3 = Wector::from_pair(p1.clone(), p2.clone());
        let w4 = Wector::from_pair(p2.clone(), p1.clone());
        assert_eq!(w1, w2);
        assert_eq!(w1, w3);
        assert_eq!(w3, -w4);
    }

    #[test]
    fn wectors_maths() {
        let w1 = Wector::new(3isize, 4, 0);
        let w2 = Wector::new(-3isize, 4, 0);
        let w3 = Wector::new(1isize, 1, 1);

        assert_eq!(w1.dot(&w3), w3.dot(&w1));
        assert_eq!(w1.dot(&w2), -9 + 16);
    }
}

fn main() {
    println!("Hello, world!");
}
