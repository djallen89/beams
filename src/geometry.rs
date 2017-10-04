use super::mathtraits::{Math, Zero, One, Polygon, PolygonError};
use super::wectors::{Point, Wector, Coordinates};

pub struct Triangle<T> {
    coords: Coordinates,
    pos: Vec<Point<T>>
}

pub fn parameterization<T: Clone + Math<T>>(s: Coordinates,
                                            u: Coordinates,
                                            v: &Vec<Point<T>>) -> Vec<Point<T>> {
    v.iter().map(|p| match s {
        Coordinates::Cartesian => u.into_cartesian(p),
        Coordinates::Cylindrical => u.into_cylindrical(p),
        Coordinates::Spherical => u.into_spherical(p)
    }).collect()
}

pub fn lengths<T: Math<T> + Clone>(points: Vec<Point<T>>) -> Vec<Wector<T>> {
    let mut shifted: Vec<Point<T>> = points.iter().skip(1).cloned().collect();
    shifted.push(points[0].clone());
    points.iter().zip(shifted.iter()).map(|(a, b)| {
        Wector::from_pair(a.clone(), b.clone())
    }).collect()
}

impl<T: Clone + Math<T>> Triangle<T> where T: PartialEq{
    pub fn new(a: Point<T>, b: Point<T>, c: Point<T>, s: Coordinates) -> Triangle<T> {
        Triangle {
            coords: s,
            pos: vec!(a, b, c)
        }
    }
    pub fn reparm(&self, s: Coordinates) -> Vec<Point<T>> {
        parameterization(self.coords, s, &self.pos)
    }

    pub fn deparm(&self, s: Coordinates) -> Vec<Point<T>> {
        parameterization(s, self.coords, &self.pos)
    }

    pub fn segments(&self) -> Vec<Wector<T>> {
        lengths(self.reparm(Coordinates::Cartesian))
    }
}

impl<T: Clone + Math<T> + Zero + One> Polygon for Triangle<T> where T: PartialEq{
    type Out = T;
    fn area(&self) -> T {
        let segs = self.segments();
        let two: T = T::one() + T::one();
        let mut ac = segs[2].clone();
        ac = ac.clone() - ac.clone() - ac.clone();
        //let ac = Wector::new(T::zero(), T::zero(), T::zero()) - segs[2].clone();
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

impl<T: Math<T> + Clone> Rectangle<T> {
    pub fn new(a: Point<T>, b: Point<T>, c: Point<T>, d: Point<T>, s: Coordinates) -> Result<Rectangle<T>, PolygonError> {
        let positions = vec!(a, b, c, d);
        let segments = lengths(positions.clone());
        if segments[0].is_orthogonal_to(&segments[1]) && segments[2].is_orthogonal_to(&segments[3]) &&
            segments[0] == segments[2] && segments[1] == segments[3]
        {
            Ok(Rectangle {
                coords: s,
                pos: positions
            })
        } else {
            Err(PolygonError::NonOrthogonal)
        }
    }

    pub fn segments(&self) -> Vec<Wector<T>> {
        lengths(parameterization(self.coords, Coordinates::Cartesian, &self.pos))
    }
}

impl<T: Clone + Math<T>> Polygon for Rectangle<T> {
    type Out = T;

    fn area(&self) -> T {
        let segs = self.segments();
        segs[0].cross(&segs[1]).norm()
    }

    fn perimeter(&self) -> T {
        let segs = self.segments();
        segs.iter().fold(T::zero(), |acc, x| acc + x.norm())
    }

    fn centroid(&self) -> Point<T> {
        let four = (0 .. 4).fold(T::zero(), |acc, _| acc + T::one());
        self.pos.iter().fold(Point::zero(), |acc, x| acc + x.clone()).div_by(&four)
    }
}
/*
pub struct Circle<T> {
    coords: Coordinates,
    pos: Vec<Point<T>>
}

impl<T: Clone + One<Out = T> + Zero<Out = T> + Div<Output = T> + Add<Output = T> + Neg<Output = T> +SquareRoot<T> +
     Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Trigonometry<T>> Polygon for Triangle<T> {
}*/
