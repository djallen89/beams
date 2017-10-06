use super::wectors::{Wector};
use super::geometry::{Shape};
use super::mathtraits::{Math, Polygon};
use std::ops::Neg;
use std::cmp::PartialOrd;

struct XSection<T> {
    neutral_axis: Wector<T>,
    shapes: Vec<Shape<T>>,
}

impl<T: Math<T> + Polygon<Out = T> + Clone + PartialOrd> XSection<T> {
    pub fn new(na: Wector<T>, s: Vec<Shape<T>>) -> XSection<T> {
        if s.iter().any(|x| (x.normal() != na.clone() && x.normal() != na.clone().neg())) {
            panic!("Mismatched norms!")
        }

        XSection {
            neutral_axis: na,
            shapes: s
        }
    }

    pub fn area(&self) -> T {
        s.iter().fold(T::zero(), |acc, x| acc + x.area())
    }
}
