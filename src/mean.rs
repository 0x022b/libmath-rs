//! Functions for calculating mean

use std::f64::NAN;

/// Calculates arithmetic mean (AM) of data set `slice`.
///
/// # Arguments
///
/// * `slice` - collection of values
///
/// # Example
///
/// ```
/// use math::mean;
///
/// let slice = [8., 16.];
/// assert_eq!(mean::arithmetic(&slice), 12.);
/// ```
pub fn arithmetic(slice: &[f64]) -> f64 {
	slice.iter().fold(0., |a, b| a + b) / slice.len() as f64
}

/// Calculate geometric mean (GM) of data set `slice`.
///
/// If the result would be imaginary, function returns `NAN`.
///
/// # Arguments
///
/// * `slice` - collection of values
///
/// # Example
///
/// ```
/// use math::mean;
///
/// let slice = [9., 16.];
/// assert_eq!(mean::geometric(&slice), 12.);
/// ```
pub fn geometric(slice: &[f64]) -> f64 {
	let product = slice.iter().fold(1., |a, b| a * b);
	match product < 0. {
		true => NAN,
		false => product.powf(1. / slice.len() as f64),
	}
}

/// Calculate harmonic mean (HM) of data set `slice`.
///
/// # Arguments
///
/// * `slice` - collection of values
///
/// # Example
///
/// ```
/// use math::mean;
///
/// let slice = [1., 7.];
/// assert_eq!(mean::harmonic(&slice), 1.75);
/// ```
pub fn harmonic(slice: &[f64]) -> f64 {
	slice.len() as f64 / slice.iter().fold(0., |a, b| a + 1. / b)
}

#[cfg(test)]
mod tests {
	use std::f64::{ NAN, INFINITY, NEG_INFINITY };
	use round;

	#[test]
	fn arithmetic() {
		let tests: [([f64; 5], f64); 9] = [
			([-7., -4., 1., 3., 8.], 0.2),
			([-4., 1., 3., 8., 12.], 4.),
			([0., 0., 0., 0., 0.], 0.),
			([0., 4., 7., 9., 17.], 7.4),
			([1., 2., 6., 4., 13.], 5.2),
			([1., 5., 10., 20., 25.], 12.2),
			([2., 3., 5., 7., 11.], 5.6),
			([NEG_INFINITY, 1., 2., 3., 4.], NEG_INFINITY),
			([1., 2., 3., 4., INFINITY], INFINITY),
		];

		for test in &tests {
			assert_eq!(round::half_up(super::arithmetic(&test.0), 4), test.1);
		}
	}

	#[test]
	fn geometric() {
		let tests: [([f64; 5], f64); 9] = [
			([-7., -4., 1., 3., 8.], 3.6768),
			([-4., 1., 3., 8., 12.], NAN),
			([0., 0., 0., 0., 0.], 0.),
			([0., 4., 7., 9., 17.], 0.),
			([1., 2., 6., 4., 13.], 3.6227),
			([1., 5., 10., 20., 25.], 7.5786),
			([2., 3., 5., 7., 11.], 4.7068),
			([NEG_INFINITY, 1., 2., 3., 4.], NAN),
			([1., 2., 3., 4., INFINITY], INFINITY),
		];

		for test in &tests {
			let result = super::geometric(&test.0);
			match result.is_nan() {
				true => assert_eq!(test.1.is_nan(), true),
				false => assert_eq!(round::half_up(result, 4), test.1),
			}
		}
	}

	#[test]
	fn harmonic() {
		let tests: [([f64; 5], f64); 9] = [
			([-7., -4., 1., 3., 8.], 4.69274),
			([-4., 1., 3., 8., 12.], 3.87097),
			([0., 0., 0., 0., 0.], 0.),
			([0., 4., 7., 9., 17.], 0.),
			([1., 2., 6., 4., 13.], 2.50804),
			([1., 5., 10., 20., 25.], 3.59712),
			([2., 3., 5., 7., 11.], 3.94602),
			([NEG_INFINITY, 1., 2., 3., 4.], 2.4),
			([1., 2., 3., 4., INFINITY], 2.4),
		];

		for test in &tests {
			assert_eq!(round::half_up(super::harmonic(&test.0), 5), test.1);
		}
	}
}
