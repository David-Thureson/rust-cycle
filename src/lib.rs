extern crate float_cmp;

pub mod cycle_series;

pub mod data_source;

pub mod time_series;
pub use time_series::*;

pub mod trig;
pub use trig::*;

pub fn indent(depth: usize) -> String {
    "\t".repeat(depth)
}

#[macro_export]
macro_rules! is_approx_equal {
    ( $x1:expr, $x2:expr ) => {
        float_cmp::approx_eq!(f64, $x1, $x2, epsilon = 0.000001, ulps = 20)
    };
}

#[macro_export]
macro_rules! assert_approx_equal {
    ( $x1:expr, $x2:expr ) => {
        assert!(float_cmp::approx_eq!(f64, $x1, $x2, epsilon = 0.000001, ulps = 20));
    };
}

