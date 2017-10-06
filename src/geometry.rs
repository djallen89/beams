use super::mathtraits::{Math, Zero, One, Polygon, PolygonError, Components};
use super::wectors::{Axis, Point, Wector, Coordinates};

pub enum Shape<T> {
    Triangle(Triangle<T>),
    Rectangle(Rectangle<T>),
    Circle(Circle<T>)
}

impl<T: Math<T> + Polygon<Out = T> + Clone> Shape<T> {
    pub fn normal(&self) -> Wector<T> {
        match self {
            &Shape::Triangle(ref t) => t.normal(),
            &Shape::Rectangle(ref r) => r.normal(),
            &Shape::Circle(ref c) => c.normal()
        }
    }
}

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

    fn normal(&self) -> Wector<T> {
        let segs = self.segments();
        let cross = segs[0].cross(&segs[1]);
        cross.unit()
    }
}

pub struct Rectangle<T> {
    coords: Coordinates,
    position: Wector<T>,
    pub width: Wector<T>,
    pub height: Wector<T>,
    normal: Wector<T>,
    thin_wall: Option<Axis>
}

impl<T: Math<T> + Clone> Rectangle<T> {
    pub fn new(s: Coordinates, position: Wector<T>, width: Wector<T>,
               length: Wector<T>, twall: Option<Axis>) -> Result<Rectangle<T>, PolygonError> {
        if length.is_orthogonal_to(&width) {
            let normal = length.cross(width);
            Ok(Rectangle {
                coords: s,
                position: position,
                width: width,
                height: length,
                normal: normal
            })
        } else {
            Err(PolygonError::NonOrthogonal)
        }
    }

    pub fn len

    /* Ratio of ab to bc */
    pub fn ratio(&self, lxw: bool) -> T {
        if lxw {
            self.length.norm() / self.width.norm()
        } else {
            self.width.norm() / self.length.norm()
        }
    }

    pub fn I(&self, a: Axis, thin_wall) -> T {
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
    
    fn normal(&self) -> Wector<T> {
        let segs = self.segments();
        let cross = segs[0].cross(&segs[1]);
        cross.unit()
    }
}


pub struct Circle<T> {
    coords: Coordinates,
    position: Wector<T>,
    radius: T,
    normal: Wector<T>
}

impl<T: Math<T> + Clone> Circle<T> {
    pub fn new(s: Coordinates, p: Wector<T>, r: T, n: Wector<T>) -> Circle<T>{
        Circle {
            coords: s,
            position: p,
            radius: r,
            normal: n
        }
    }

    pub fn radius(&self) -> T {
        self.radius.clone()
    }
}

impl<T: Math<T> + Clone> Polygon for Circle<T> {
    type Out = T;
    fn area(&self) -> T {
        T::pi() * self.radius() * self.radius()
    }


    fn perimeter(&self) -> T {
        (T::one() + T::one()) * T::pi() * self.radius()
    }

    fn centroid(&self) -> Point<T> {
        Point::new(self.position.x(), self.position.y(), self.position.z())
    }

    fn normal(&self) -> Wector<T> {
        self.normal.clone()
    }
}
