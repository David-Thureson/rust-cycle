#![allow(unused_variables)]

use std::f64::consts::PI;
use std::ops::{Add, Neg, Sub};

use crate::*;
use std::iter::Sum;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    radians: f64,
    length: f64,
}

impl Vector {
    pub fn new_radians(radians: f64, length: f64) -> Self {
        Self {
            radians: if is_approx_equal!(length, 0.0) { 0.0 } else { normalize_radians(radians) },
            length,
        }
    }

    pub fn new_fraction(fraction: f64, length: f64) -> Self {
        Self::new_radians(fraction_to_radians(fraction), length)
    }

    pub fn new_degrees(degrees: f64, length: f64) -> Self {
        Self::new_radians(degrees_to_radians(degrees), length)
    }

    pub fn new_xy(x: f64, y: f64) -> Self {
        let x_is_zero = is_approx_equal!(x, 0.0);
        let y_is_zero = is_approx_equal!(y, 0.0);
        //bg!(x, y, x_is_zero, y_is_zero);
        if x_is_zero && y_is_zero {
            Self::new_zero()
        } else {
            let angle = if x_is_zero {
                if y > 0.0 {
                    PI / 2.0
                } else {
                    (3.0 * PI) / 2.0
                }
            } else {
                if y_is_zero {
                    if x > 0.0 {
                        0.0
                    } else {
                        PI
                    }
                } else {
                    let mut angle = (y / x).atan();
                    //bg!(angle, radians_to_degrees(angle));
                    if x < 0.0 {
                        // Second or third quadrant.
                        angle += PI;
                        //bg!(angle, radians_to_degrees(angle));
                    }
                    angle
                }
            };
            let length = (x.powi(2) + y.powi(2)).sqrt();
            //bg!(angle, length);
            Self::new_radians(angle, length)
        }
    }

    pub fn new_zero() -> Self {
        Self::new_radians(0.0, 0.0)
    }

    pub fn fraction(&self) -> f64 {
        let fraction = radians_to_fraction(self.radians);
        assert_fraction(fraction);
        fraction
    }

    pub fn degrees(&self) -> f64 {
        let degrees = radians_to_degrees(self.radians);
        assert_degrees(degrees);
        degrees
    }

    pub fn x(&self) -> f64 {
        self.radians.cos() * self.length
    }

    pub fn y(&self) -> f64 {
        self.radians.sin() * self.length
    }

    pub fn xy(&self) -> (f64, f64) {
        (self.x(), self.y())
    }

    pub fn print_indent(&self, depth: usize) {
        let i0 = indent(depth);
        let i1 = indent(depth + 1);
        let (x, y) = self.xy();
        println!("\n{}Vector {{", i0);
        println!("{}radians:  {:?}", i1, self.radians);
        println!("{}fraction: {:?}", i1, self.fraction());
        println!("{}degrees:  {:?}", i1, self.degrees());
        println!("{}length:   {:?}", i1, self.length);
        println!("{}x:        {:?}", i1, x);
        println!("{}y:        {:?}", i1, y);
        println!("{}}}", i0);
    }

}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self::new_radians(self.radians + PI, self.length)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.xy();
        let (x2, y2) = rhs.xy();
        //bg!(&self, &rhs, x1, y1, x2, y2);
        Self::Output::new_xy(x1 + x2, y1 + y2)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<'a> Sum<&'a Self> for Vector {
    fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::new_zero(), |a, b| a + *b)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        is_approx_equal!(self.radians, other.radians) && is_approx_equal!(self.length, other.length)
    }
}

impl Eq for Vector{}

pub fn fraction_to_radians(fraction: f64) -> f64 {
    let radians = fraction * 2.0 * PI;
    radians
}

pub fn radians_to_fraction(radians: f64) -> f64 {
    let fraction = radians / (2.0 * PI);
    fraction
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    let radians = (degrees / 360.0) * 2.0 * PI;
    radians
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    let degrees = (radians / (2.0 * PI)) * 360.0;
    degrees
}

pub fn normalize_radians(radians: f64) -> f64 {
    let mut norm_radians= radians % (2.0 * PI);
    if norm_radians == 2.0 * PI {
        norm_radians = 0.0;
    } else if norm_radians < 0.0 {
        norm_radians += 2.0 * PI;
    }
    assert_radians(norm_radians);
    norm_radians
}

pub fn assert_fraction(val: f64) {
    assert!(val.is_finite());
    assert!(val >= 0.0);
    assert!(val <= 1.0);
}

pub fn assert_radians(val: f64) {
    assert!(val.is_finite());
    assert!(val >= 0.0);
    assert!(val <= 2.0 * PI);
}

pub fn assert_degrees(val: f64) {
    assert!(val.is_finite());
    assert!(val >= 0.0);
    assert!(val <= 360.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_to_radians() {
        assert_approx_equal!(0.0, fraction_to_radians(0.0));
        assert_approx_equal!(2.0 * PI, fraction_to_radians(1.0));
        assert_approx_equal!(PI, fraction_to_radians(0.5));
        assert_approx_equal!(PI / 2.0, fraction_to_radians(0.25));
        assert_approx_equal!((3.0 * PI) / 2.0, fraction_to_radians(0.75));
    }

    #[test]
    fn test_radians_to_fraction() {
        assert_approx_equal!(0.0, radians_to_fraction(0.0));
        assert_approx_equal!(0.5, radians_to_fraction(PI));
        assert_approx_equal!(1.0, radians_to_fraction(2.0 * PI));
        assert_approx_equal!(0.375, radians_to_fraction((3.0 * PI) / 4.0));
    }

    #[test]
    fn test_degrees_to_radians() {
        assert_approx_equal!(0.0, degrees_to_radians(0.0));
        assert_approx_equal!(PI, degrees_to_radians(180.0));
        assert_approx_equal!(2.0 * PI, degrees_to_radians(360.0));
        assert_approx_equal!((3.0 * PI) / 2.0, degrees_to_radians(270.0));
        assert_approx_equal!((7.0 * PI) / 4.0, degrees_to_radians(315.0));
    }

    #[test]
    fn test_radians_to_degrees() {
        assert_approx_equal!(0.0, radians_to_degrees(0.0));
        assert_approx_equal!(360.0, radians_to_degrees(2.0 * PI));
        assert_approx_equal!(180.0, radians_to_degrees(PI));
    }

    #[test]
    fn test_normalize_radians() {
        assert_approx_equal!(0.0, normalize_radians(0.0));
        assert_approx_equal!(0.0, normalize_radians(2.0 * PI));
        assert_approx_equal!(0.0, normalize_radians(100.0 * PI));
        assert_approx_equal!(0.0, normalize_radians(-18.0 * PI));
        assert_approx_equal!(PI, normalize_radians(PI));
        assert_approx_equal!(PI, normalize_radians(-PI));
        assert_approx_equal!(PI, normalize_radians(3.0 * PI));
        assert_approx_equal!(PI, normalize_radians(-9.0 * PI));
        assert_approx_equal!((3.0 * PI) / 2.0, normalize_radians((3.0 * PI) / 2.0));
        assert_approx_equal!((3.0 * PI) / 2.0, normalize_radians((7.0 * PI) / 2.0));
        assert_approx_equal!((3.0 * PI) / 2.0, normalize_radians((-113.0 * PI) / 2.0));
    }

    #[test]
    fn test_vector_equal() {
        // This function also tests the radians, fraction, ond degrees constructors.

        let vector = Vector::new_radians(PI, 10.0);
        assert_eq!(vector, Vector::new_fraction(0.5, 10.0));
        assert_eq!(vector, Vector::new_fraction(2.5, 10.0));
        assert_eq!(vector, Vector::new_fraction(-19.5, 10.0));
        assert_ne!(vector, Vector::new_fraction(0.50001, 10.0));
        assert_ne!(vector, Vector::new_fraction(0.5, 9.9999));
        assert_eq!(vector, Vector::new_degrees(180.0, 10.0));
        assert_eq!(vector, Vector::new_degrees(540.0, 10.0));
        assert_eq!(vector, Vector::new_degrees(-180.0, 10.0));
        assert_ne!(vector, Vector::new_degrees(179.999, 10.0));
        assert_ne!(vector, Vector::new_degrees(180.0, 10.0001));

        let vector = Vector::new_fraction(0.75, 1_500.0);
        assert_eq!(vector, Vector::new_radians((3.0 * PI) / 2.0, 1_500.0));
        assert_eq!(vector, Vector::new_radians((7.0 * PI) / 2.0, 1_500.0));
        assert_eq!(vector, Vector::new_radians(-PI / 2.0, 1_500.0));
        assert_ne!(vector, Vector::new_radians((3.0 * PI) / 2.0001, 1_500.0));
        assert_ne!(vector, Vector::new_radians((3.0 * PI) / 2.0, 1_500.001));
        assert_eq!(vector, Vector::new_degrees(270.0, 1_500.0));
        assert_eq!(vector, Vector::new_degrees(630.0, 1_500.0));
        assert_eq!(vector, Vector::new_degrees(-450.0, 1_500.0));
        assert_ne!(vector, Vector::new_degrees(270.001, 1_500.0));
        assert_ne!(vector, Vector::new_degrees(270.0, 1_499.99));
    }

    #[test]
    fn test_vector_negate() {
        assert_eq!(Vector::new_degrees(85.0, 7.0), -Vector::new_degrees(265.0, 7.0));
        assert_eq!(Vector::new_degrees(250.0, 12.44), -Vector::new_degrees(70.0, 12.44));
        assert_eq!(Vector::new_zero(), -Vector::new_zero());
    }

    #[test]
    fn test_vector_new_xy() {
        let sqrt_2 = 2.0f64.sqrt();
        assert_eq!(Vector::new_zero(), Vector::new_xy(0.0, 0.0));
        assert_eq!(Vector::new_degrees(0.0, 5.0), Vector::new_xy(5.0, 0.0));
        assert_eq!(Vector::new_degrees(90.0, 5.0), Vector::new_xy(0.0, 5.0));
        assert_eq!(Vector::new_degrees(180.0, 5.0), Vector::new_xy(-5.0, 0.0));
        assert_eq!(Vector::new_degrees(270.0, 5.0), Vector::new_xy(0.0, -5.0));
        assert_eq!(Vector::new_degrees(315.0, 3.0 * sqrt_2), Vector::new_xy(3.0, -3.0));
    }

    #[test]
    fn test_vector_add() {
        let right_1 = Vector::new_degrees(0.0, 1.0);
        let up_1 = Vector::new_degrees(90.0, 1.0);
        let left_1 = Vector::new_degrees(180.0, 1.0);
        let down_1 = Vector::new_degrees(270.0, 1.0);
        let sqrt_2 = 2.0f64.sqrt();

        assert_eq!(Vector::new_degrees(0.0, 2.0), right_1 + right_1);
        assert_eq!(Vector::new_degrees(0.0, 2.0), right_1 - left_1);
        assert_eq!(Vector::new_degrees(0.0, 2.0), right_1 + right_1 + right_1 + left_1);
        assert_eq!(Vector::new_degrees(45.0, sqrt_2), right_1 + up_1);
        assert_eq!(Vector::new_zero(), left_1 + right_1);
        assert_eq!(Vector::new_zero(), left_1 - left_1);
        assert_eq!(Vector::new_degrees(225.0, sqrt_2), left_1 + down_1);
    }

    #[test]
    fn test_vector_sum() {
        let right_1 = Vector::new_degrees(0.0, 1.0);
        let up_1 = Vector::new_degrees(90.0, 1.0);
        let left_1 = Vector::new_degrees(180.0, 1.0);
        let down_1 = Vector::new_degrees(270.0, 1.0);
        let sqrt_2 = 2.0f64.sqrt();

        let sum = [right_1, up_1].iter().sum::<Vector>();
        assert_eq!(sum, Vector::new_degrees(45.0, sqrt_2));

        let sum = [right_1, up_1, right_1, left_1, -left_1, down_1].iter().sum::<Vector>();
        assert_eq!(sum, Vector::new_degrees(0.0, 2.0));
    }
}
