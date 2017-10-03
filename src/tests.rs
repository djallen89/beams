#[cfg(test)]
use super::geometry::{Point, Wector, Components, Triangle, Coordinates, Polygon};

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

#[test]
fn triangle_area() {
    let abc = Triangle::new(Point::new(0f64, 0.0, 0.0), Point::new(3.0, 0.0, 0.0), Point::new(3.0, 4.0, 0.0), Coordinates::Cartesian);
    assert_eq!(abc.area(), 6.0);
    assert_eq!(abc.perimeter(), 12.0);
    assert_eq!(abc.centroid(), Point::new(2.0f64, 4.0/3.0, 0.0));
}
