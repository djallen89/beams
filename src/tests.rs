#[cfg(test)]
use super::wectors::{Point, Wector, Coordinates};
use super::wectors::Coordinates::{Spherical, Cylindrical, Cartesian};
use super::mathtraits::{Components, Polygon};
use super::geometry::{Triangle};

#[test]
fn create_new_wectors() {
    let p1 = Point::new(10f64, 3.0, 3.0);
    let p2 = Point::new(7f64, 3.0, -2.0);
    assert_eq!(p1.y(), p2.y());
    let w1 = Wector::new(-3f64, 0.0, -5.0);
    let w2 = Wector::from_point(&(p2.clone() - p1.clone()));
    let w3 = Wector::from_pair(p1.clone(), p2.clone());
    let w4 = Wector::from_pair(p2.clone(), p1.clone());
    assert_eq!(w1, w2);
    assert_eq!(w1, w3);
    assert_eq!(w3, -w4);
}

#[test]
fn wectors_maths() {
    let w1 = Wector::new(3f64, 4.0, 0.0);
    let w2 = Wector::new(-3f64, 4.0, 0.0);
    let w3 = Wector::new(1f64, 1.0, 1.0);

    assert_eq!(w1.dot(&w3), w3.dot(&w1));
    assert_eq!(w1.dot(&w2), -9.0 + 16.0);
}

#[test]
fn coordination() {
    let p0 = Point::new(0.0f64, 0.0, 0.0);
    let p1 = Point::new(1.0f64, 0.0, 0.0);
    let p2 = Point::new(0.0f64, 1.0, 0.0);
    let p3 = Point::new(0.0f64, 0.0, 1.0);

    assert_eq!(Cartesian.into_cylindrical(&p0), p0);
    assert_eq!(Cartesian.into_spherical(&p0), p0);
    assert_eq!(Cylindrical.into_cartesian(&p0), p0);
    assert_eq!(Cylindrical.into_spherical(&p0), p0);
    assert_eq!(Spherical.into_cartesian(&p0), p0);
    assert_eq!(Spherical.into_cylindrical(&p0), p0);

    assert_eq!(Cartesian.into_cylindrical(&p1), p1);
}

#[test]
fn triangle_area() {
    let abc = Triangle::new(Point::new(0f64, 0.0, 0.0), Point::new(3.0, 0.0, 0.0), Point::new(3.0, 4.0, 0.0), Coordinates::Cartesian);
    assert_eq!(abc.area(), 6.0);
    assert_eq!(abc.perimeter(), 12.0);
    assert_eq!(abc.centroid(), Point::new(2.0f64, 4.0/3.0, 0.0));
}
