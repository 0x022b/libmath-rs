//! Rounding functions
extern crate rand;

/// Round up.
///
/// Round `value` up to `scale` number of decimal digits.
///
/// # Arguments
///
/// * `value` - value to round
/// * `scale` - number of decimal digits
///
/// # Example
///
/// ```
/// use math::round;
///
/// let rounded = round::ceil(3.14159, 3);
/// assert_eq!(rounded, 3.142);
/// ```
pub fn ceil(value: f64, scale: u8) -> f64 {
	let multiplier = 10i64.pow(scale as u32) as f64;
	(value * multiplier).ceil() / multiplier
}

/// Round down.
///
/// Round `value` down to `scale` number of decimal digits.
///
/// # Arguments
///
/// * `value` - value to round
/// * `scale` - number of decimal digits
///
/// # Example
///
/// ```
/// use math::round;
///
/// let rounded = round::floor(3.14159, 3);
/// assert_eq!(rounded, 3.141);
/// ```
pub fn floor(value: f64, scale: u8) -> f64 {
	let multiplier = 10i64.pow(scale as u32) as f64;
	(value * multiplier).floor() / multiplier
}

/// Round half away from zero.
///
/// Round `value` to `scale` number of decimal digits
/// rounding half away from zero.
///
/// # Arguments
///
/// * `value` - value to round
/// * `scale` - number of decimal digits
///
/// # Example
///
/// ```
/// use math::round;
///
/// let rounded = round::half_away_from_zero(3.14159, 3);
/// assert_eq!(rounded, 3.142);
/// ```
pub fn half_away_from_zero(value: f64, scale: u8) -> f64 {
	towards_zero(value, scale, false)
}

/// Round half down.
///
/// Round `value` to `scale` number of decimal digits
/// rounding half down.
///
/// # Arguments
///
/// * `value` - value to round
/// * `scale` - number of decimal digits
///
/// # Example
///
/// ```
/// use math::round;
///
/// let rounded = round::half_down(3.14159, 3);
/// assert_eq!(rounded, 3.141);
/// ```
pub fn half_down(value: f64, scale: u8) -> f64 {
	up_or_down(value, scale, false)
}

/// Round half to nearest even number.
///
/// Round `value` to `scale` number of decimal digits
/// rounding half to nearest even number.
///
/// # Arguments
///
/// * `value` - value to round
/// * `scale` - number of decimal digits
///
/// # Example
///
/// ```
/// use math::round;
///
/// let rounded = round::half_to_even(3.14159, 3);
/// assert_eq!(rounded, 3.142);
/// ```
pub fn half_to_even(value: f64, scale: u8) -> f64 {
	even_or_odd(value, scale, true)
}

/// Round half to nearest odd number.
///
/// Round `value` to `scale` number of decimal digits
/// rounding half to nearest odd number.
///
/// # Arguments
///
/// * `value` - value to round
/// * `scale` - number of decimal digits
///
/// # Example
///
/// ```
/// use math::round;
///
/// let rounded = round::half_to_odd(3.14159, 3);
/// assert_eq!(rounded, 3.141);
/// ```
pub fn half_to_odd(value: f64, scale: u8) -> f64 {
	even_or_odd(value, scale, false)
}

/// Round half towards zero.
///
/// Round `value` to `scale` number of decimal digits
/// rounding half towards zero.
///
/// # Arguments
///
/// * `value` - value to round
/// * `scale` - number of decimal digits
///
/// # Example
///
/// ```
/// use math::round;
///
/// let rounded = round::half_towards_zero(3.14159, 3);
/// assert_eq!(rounded, 3.141);
/// ```
pub fn half_towards_zero(value: f64, scale: u8) -> f64 {
	towards_zero(value, scale, true)
}

/// Round half up.
///
/// Round `value` to `scale` number of decimal digits
/// rounding half up.
///
/// # Arguments
///
/// * `value` - value to round
/// * `scale` - number of decimal digits
///
/// # Example
///
/// ```
/// use math::round;
///
/// let rounded = round::half_up(3.14159, 3);
/// assert_eq!(rounded, 3.142);
/// ```
pub fn half_up(value: f64, scale: u8) -> f64 {
	up_or_down(value, scale, true)
}

/// Round half randomly up or down.
///
/// Round `value` to `scale` number of decimal digits
/// rounding half randomly up or down.
///
/// # Arguments
///
/// * `value` - value to round
/// * `scale` - number of decimal digits
///
/// # Example
///
/// ```
/// use math::round;
///
/// let rounded = round::stochastic(3.14159, 3);
/// assert_eq!(rounded == 3.141 || rounded == 3.142, true);
/// ```
pub fn stochastic(value: f64, scale: u8) -> f64 {
	let digits = significant_digits(value, scale);
	to_nearest(value, scale, digits.1)
}

fn even_or_odd(value: f64, scale: u8, even: bool) -> f64 {
	let digits = significant_digits(value, scale);
	match digits.1 == 5 {
		true => round(value, scale, (value < 0.) ^ even ^ (digits.0 % 2 == 0)),
		false => to_nearest(value, scale, digits.1),
	}
}

fn round(value: f64, scale: u8, up: bool) -> f64 {
	match up {
		true => ceil(value, scale),
		false => floor(value, scale),
	}
}

fn significant_digits(value: f64, scale: u8) -> (u8, u8) {
	if value.is_nan() || value.is_infinite() {
		return (0, 0);
	}
	let x = (value * 10f64.powi(scale as i32 + 2)) as i64;
	let y = ((x - x / 1000 * 1000).abs() / 10) as u8;
	(y / 10, y % 10)
}

fn to_nearest(value: f64, scale: u8, digit: u8) -> f64 {
	let up = match digit == 5 {
		true => rand::random::<bool>(),
		false => (value < 0.) ^ (digit > 5),
	};
	round(value, scale, up)
}

fn towards_zero(value: f64, scale: u8, towards: bool) -> f64 {
	let digits = significant_digits(value, scale);
	match digits.1 == 5 {
		true => round(value, scale, (value < 0.) ^ !towards),
		false => to_nearest(value, scale, digits.1),
	}
}

fn up_or_down(value: f64, scale: u8, up: bool) -> f64 {
	let digit = significant_digits(value, scale);
	match digit.1 == 5 {
		true => round(value, scale, up),
		false => to_nearest(value, scale, digit.1),
	}
}

#[cfg(test)]
mod tests {
	use std::f64::{ NAN, INFINITY, NEG_INFINITY };

	#[test]
	fn ceil() {
		let tests = [
			(-1.03, 1, -1.),
			(-1.05, 1, -1.),
			(-1.07, 1, -1.),
			(-1.13, 1, -1.1),
			(-1.15, 1, -1.1),
			(-1.17, 1, -1.1),
			(-1.23, 1, -1.2),
			(-1.25, 1, -1.2),
			(-1.27, 1, -1.2),
			(-1.33, 1, -1.3),
			(-1.35, 1, -1.3),
			(-1.37, 1, -1.3),

			(1.03, 1, 1.1),
			(1.05, 1, 1.1),
			(1.07, 1, 1.1),
			(1.13, 1, 1.2),
			(1.15, 1, 1.2),
			(1.17, 1, 1.2),
			(1.23, 1, 1.3),
			(1.25, 1, 1.3),
			(1.27, 1, 1.3),
			(1.33, 1, 1.4),
			(1.35, 1, 1.4),
			(1.37, 1, 1.4),

			(INFINITY, 1, INFINITY),
			(NAN, 1, NAN),
			(NEG_INFINITY, 1, NEG_INFINITY),
		];

		for test in tests.iter() {
			let result = super::ceil(test.0, test.1);
			match result.is_nan() {
				true => assert_eq!(test.2.is_nan(), true),
				false => assert_eq!(result, test.2),
			}
		}
	}

	#[test]
	fn floor() {
		let tests = [
			(-1.03, 1, -1.1),
			(-1.05, 1, -1.1),
			(-1.07, 1, -1.1),
			(-1.13, 1, -1.2),
			(-1.15, 1, -1.2),
			(-1.17, 1, -1.2),
			(-1.23, 1, -1.3),
			(-1.25, 1, -1.3),
			(-1.27, 1, -1.3),
			(-1.33, 1, -1.4),
			(-1.35, 1, -1.4),
			(-1.37, 1, -1.4),

			(1.03, 1, 1.),
			(1.05, 1, 1.),
			(1.07, 1, 1.),
			(1.13, 1, 1.1),
			(1.15, 1, 1.1),
			(1.17, 1, 1.1),
			(1.23, 1, 1.2),
			(1.25, 1, 1.2),
			(1.27, 1, 1.2),
			(1.33, 1, 1.3),
			(1.35, 1, 1.3),
			(1.37, 1, 1.3),

			(INFINITY, 1, INFINITY),
			(NAN, 1, NAN),
			(NEG_INFINITY, 1, NEG_INFINITY),
		];

		for test in tests.iter() {
			let result = super::floor(test.0, test.1);
			match result.is_nan() {
				true => assert_eq!(test.2.is_nan(), true),
				false => assert_eq!(result, test.2),
			}
		}
	}

	#[test]
	fn half_away_from_zero() {
		let tests = [
			(-1.03, 1, -1.),
			(-1.05, 1, -1.1),
			(-1.07, 1, -1.1),
			(-1.13, 1, -1.1),
			(-1.15, 1, -1.2),
			(-1.17, 1, -1.2),
			(-1.23, 1, -1.2),
			(-1.25, 1, -1.3),
			(-1.27, 1, -1.3),
			(-1.33, 1, -1.3),
			(-1.35, 1, -1.4),
			(-1.37, 1, -1.4),

			(1.03, 1, 1.),
			(1.05, 1, 1.1),
			(1.07, 1, 1.1),
			(1.13, 1, 1.1),
			(1.15, 1, 1.2),
			(1.17, 1, 1.2),
			(1.23, 1, 1.2),
			(1.25, 1, 1.3),
			(1.27, 1, 1.3),
			(1.33, 1, 1.3),
			(1.35, 1, 1.4),
			(1.37, 1, 1.4),

			(INFINITY, 1, INFINITY),
			(NAN, 1, NAN),
			(NEG_INFINITY, 1, NEG_INFINITY),
		];

		for test in tests.iter() {
			let result = super::half_away_from_zero(test.0, test.1);
			match result.is_nan() {
				true => assert_eq!(test.2.is_nan(), true),
				false => assert_eq!(result, test.2),
			}
		}
	}

	#[test]
	fn half_down() {
		let tests = [
			(-1.03, 1, -1.),
			(-1.05, 1, -1.1),
			(-1.07, 1, -1.1),
			(-1.13, 1, -1.1),
			(-1.15, 1, -1.2),
			(-1.17, 1, -1.2),
			(-1.23, 1, -1.2),
			(-1.25, 1, -1.3),
			(-1.27, 1, -1.3),
			(-1.33, 1, -1.3),
			(-1.35, 1, -1.4),
			(-1.37, 1, -1.4),

			(1.03, 1, 1.),
			(1.05, 1, 1.),
			(1.07, 1, 1.1),
			(1.13, 1, 1.1),
			(1.15, 1, 1.1),
			(1.17, 1, 1.2),
			(1.23, 1, 1.2),
			(1.25, 1, 1.2),
			(1.27, 1, 1.3),
			(1.33, 1, 1.3),
			(1.35, 1, 1.3),
			(1.37, 1, 1.4),

			(INFINITY, 1, INFINITY),
			(NAN, 1, NAN),
			(NEG_INFINITY, 1, NEG_INFINITY),
		];

		for test in tests.iter() {
			let result = super::half_down(test.0, test.1);
			match result.is_nan() {
				true => assert_eq!(test.2.is_nan(), true),
				false => assert_eq!(result, test.2),
			}
		}
	}

	#[test]
	fn half_to_even() {
		let tests = [
			(-1.03, 1, -1.),
			(-1.05, 1, -1.),
			(-1.07, 1, -1.1),
			(-1.13, 1, -1.1),
			(-1.15, 1, -1.2),
			(-1.17, 1, -1.2),
			(-1.23, 1, -1.2),
			(-1.25, 1, -1.2),
			(-1.27, 1, -1.3),
			(-1.33, 1, -1.3),
			(-1.35, 1, -1.4),
			(-1.37, 1, -1.4),

			(1.03, 1, 1.),
			(1.05, 1, 1.),
			(1.07, 1, 1.1),
			(1.13, 1, 1.1),
			(1.15, 1, 1.2),
			(1.17, 1, 1.2),
			(1.23, 1, 1.2),
			(1.25, 1, 1.2),
			(1.27, 1, 1.3),
			(1.33, 1, 1.3),
			(1.35, 1, 1.4),
			(1.37, 1, 1.4),

			(INFINITY, 1, INFINITY),
			(NAN, 1, NAN),
			(NEG_INFINITY, 1, NEG_INFINITY),
		];

		for test in tests.iter() {
			let result = super::half_to_even(test.0, test.1);
			match result.is_nan() {
				true => assert_eq!(test.2.is_nan(), true),
				false => assert_eq!(result, test.2),
			}
		}
	}

	#[test]
	fn half_to_odd() {
		let tests = [
			(-1.03, 1, -1.),
			(-1.05, 1, -1.1),
			(-1.07, 1, -1.1),
			(-1.13, 1, -1.1),
			(-1.15, 1, -1.1),
			(-1.17, 1, -1.2),
			(-1.23, 1, -1.2),
			(-1.25, 1, -1.3),
			(-1.27, 1, -1.3),
			(-1.33, 1, -1.3),
			(-1.35, 1, -1.3),
			(-1.37, 1, -1.4),

			(1.03, 1, 1.),
			(1.05, 1, 1.1),
			(1.07, 1, 1.1),
			(1.13, 1, 1.1),
			(1.15, 1, 1.1),
			(1.17, 1, 1.2),
			(1.23, 1, 1.2),
			(1.25, 1, 1.3),
			(1.27, 1, 1.3),
			(1.33, 1, 1.3),
			(1.35, 1, 1.3),
			(1.37, 1, 1.4),

			(INFINITY, 1, INFINITY),
			(NAN, 1, NAN),
			(NEG_INFINITY, 1, NEG_INFINITY),
		];

		for test in tests.iter() {
			let result = super::half_to_odd(test.0, test.1);
			match result.is_nan() {
				true => assert_eq!(test.2.is_nan(), true),
				false => assert_eq!(result, test.2),
			}
		}
	}

	#[test]
	fn half_towards_zero() {
		let tests = [
			(-1.03, 1, -1.),
			(-1.05, 1, -1.),
			(-1.07, 1, -1.1),
			(-1.13, 1, -1.1),
			(-1.15, 1, -1.1),
			(-1.17, 1, -1.2),
			(-1.23, 1, -1.2),
			(-1.25, 1, -1.2),
			(-1.27, 1, -1.3),
			(-1.33, 1, -1.3),
			(-1.35, 1, -1.3),
			(-1.37, 1, -1.4),

			(1.03, 1, 1.),
			(1.05, 1, 1.),
			(1.07, 1, 1.1),
			(1.13, 1, 1.1),
			(1.15, 1, 1.1),
			(1.17, 1, 1.2),
			(1.23, 1, 1.2),
			(1.25, 1, 1.2),
			(1.27, 1, 1.3),
			(1.33, 1, 1.3),
			(1.35, 1, 1.3),
			(1.37, 1, 1.4),

			(INFINITY, 1, INFINITY),
			(NAN, 1, NAN),
			(NEG_INFINITY, 1, NEG_INFINITY),
		];

		for test in tests.iter() {
			let result = super::half_towards_zero(test.0, test.1);
			match result.is_nan() {
				true => assert_eq!(test.2.is_nan(), true),
				false => assert_eq!(result, test.2),
			}
		}
	}

	#[test]
	fn half_up() {
		let tests = [
			(-1.03, 1, -1.),
			(-1.05, 1, -1.),
			(-1.07, 1, -1.1),
			(-1.13, 1, -1.1),
			(-1.15, 1, -1.1),
			(-1.17, 1, -1.2),
			(-1.23, 1, -1.2),
			(-1.25, 1, -1.2),
			(-1.27, 1, -1.3),
			(-1.33, 1, -1.3),
			(-1.35, 1, -1.3),
			(-1.37, 1, -1.4),

			(1.03, 1, 1.),
			(1.05, 1, 1.1),
			(1.07, 1, 1.1),
			(1.13, 1, 1.1),
			(1.15, 1, 1.2),
			(1.17, 1, 1.2),
			(1.23, 1, 1.2),
			(1.25, 1, 1.3),
			(1.27, 1, 1.3),
			(1.33, 1, 1.3),
			(1.35, 1, 1.4),
			(1.37, 1, 1.4),

			(INFINITY, 1, INFINITY),
			(NAN, 1, NAN),
			(NEG_INFINITY, 1, NEG_INFINITY),
		];

		for test in tests.iter() {
			let result = super::half_up(test.0, test.1);
			match result.is_nan() {
				true => assert_eq!(test.2.is_nan(), true),
				false => assert_eq!(result, test.2),
			}
		}
	}

	#[test]
	fn significant_digits() {
		let tests = [
			(-1.1234567890, 0, (1, 1)),
			(-1.1234567890, 1, (1, 2)),
			(-1.1234567890, 2, (2, 3)),
			(-1.1234567890, 3, (3, 4)),
			(-1.1234567890, 4, (4, 5)),
			(-1.1234567890, 5, (5, 6)),
			(-1.1234567890, 6, (6, 7)),
			(-1.1234567890, 7, (7, 8)),
			(-1.1234567890, 8, (8, 9)),
			(-1.1234567890, 9, (9, 0)),

			(1.1234567890, 0, (1, 1)),
			(1.1234567890, 1, (1, 2)),
			(1.1234567890, 2, (2, 3)),
			(1.1234567890, 3, (3, 4)),
			(1.1234567890, 4, (4, 5)),
			(1.1234567890, 5, (5, 6)),
			(1.1234567890, 6, (6, 7)),
			(1.1234567890, 7, (7, 8)),
			(1.1234567890, 8, (8, 9)),
			(1.1234567890, 9, (9, 0)),

			(-1.15, 1, (1, 5)),
			(1.15, 1, (1, 5)),
			(1.9999, 3, (9, 9)),

			(INFINITY, 1, (0, 0)),
			(NAN, 1, (0, 0)),
			(NEG_INFINITY, 1, (0, 0)),
		];

		for test in tests.iter() {
			let result = super::significant_digits(test.0, test.1);
			assert_eq!(result, test.2);
		}
	}
}
