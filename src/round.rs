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
	let decimal = decimal_after_scale(value, scale);
	to_nearest(value, scale, decimal)
}

fn decimal_after_scale(value: f64, scale: u8) -> f64 {
	let x = (value * 10i64.pow(scale as u32 + 1) as f64) as i64;
	let y = (value * 10i64.pow(scale as u32) as f64) as i64 * 10;
	((x - y).abs() as f64 / 10.0)
}

fn even_or_odd(value: f64, scale: u8, even: bool) -> f64 {
	let decimal = decimal_after_scale(value, scale);
	match decimal == 0.5 {
		true => round(value, scale,
			(value < 0.0) ^ even ^ (value as i32 % 2 == 0)),
		false => to_nearest(value, scale, decimal),
	}
}

fn round(value: f64, scale: u8, up: bool) -> f64 {
	match up {
		true => ceil(value, scale),
		false => floor(value, scale),
	}
}

fn to_nearest(value: f64, scale: u8, decimal: f64) -> f64 {
	let up = match decimal == 0.5 {
		true => rand::random::<bool>(),
		false => (value < 0.0) ^ (decimal > 0.5),
	};
	round(value, scale, up)
}

fn towards_zero(value: f64, scale: u8, towards: bool) -> f64 {
	let decimal = decimal_after_scale(value, scale);
	match decimal == 0.5 {
		true => round(value, scale, (value < 0.0) ^ !towards),
		false => to_nearest(value, scale, decimal),
	}
}

fn up_or_down(value: f64, scale: u8, up: bool) -> f64 {
	let decimal = decimal_after_scale(value, scale);
	match decimal == 0.5 {
		true => round(value, scale, up),
		false => to_nearest(value, scale, decimal),
	}
}

#[cfg(test)]
mod tests {
	use std::f64::consts::PI;

	#[test]
	fn ceil() {
		let tests = [
			(-PI, 0, -3.0),
			(-PI, 1, -3.1),
			(-PI, 2, -3.14),
			(-PI, 3, -3.141),
			(-PI, 4, -3.1415),
			(-PI, 5, -3.14159),
			(-PI, 6, -3.141592),
			(-PI, 7, -3.1415926),
			(-PI, 8, -3.14159265),
			(-PI, 9, -3.141592653),

			(PI, 0, 4.0),
			(PI, 1, 3.2),
			(PI, 2, 3.15),
			(PI, 3, 3.142),
			(PI, 4, 3.1416),
			(PI, 5, 3.1416),
			(PI, 6, 3.141593),
			(PI, 7, 3.1415927),
			(PI, 8, 3.14159266),
			(PI, 9, 3.141592654),
		];

		for test in &tests {
			assert_eq!(super::ceil(test.0, test.1), test.2);
		}
	}

	#[test]
	fn decimal_after_scale() {
		let tests = [
			(-1.1234567890, 0, 0.1),
			(-1.1234567890, 1, 0.2),
			(-1.1234567890, 2, 0.3),
			(-1.1234567890, 3, 0.4),
			(-1.1234567890, 4, 0.5),
			(-1.1234567890, 5, 0.6),
			(-1.1234567890, 6, 0.7),
			(-1.1234567890, 7, 0.8),
			(-1.1234567890, 8, 0.9),
			(-1.1234567890, 9, 0.0),

			(1.1234567890, 0, 0.1),
			(1.1234567890, 1, 0.2),
			(1.1234567890, 2, 0.3),
			(1.1234567890, 3, 0.4),
			(1.1234567890, 4, 0.5),
			(1.1234567890, 5, 0.6),
			(1.1234567890, 6, 0.7),
			(1.1234567890, 7, 0.8),
			(1.1234567890, 8, 0.9),
			(1.1234567890, 9, 0.0),
		];

		for test in &tests {
			assert_eq!(super::decimal_after_scale(test.0, test.1), test.2);
		}
	}

	#[test]
	fn even_or_odd() {
		let tests = [
			(-2.5, 0, true, -2.0),
			(-2.5, 0, false, -3.0),
			(-1.4, 0, true, -1.0),
			(-1.4, 0, false, -1.0),
			(-1.5, 0, true, -2.0),
			(-1.5, 0, false, -1.0),
			(-1.6, 0, true, -2.0),
			(-1.6, 0, false, -2.0),

			(1.4, 0, true, 1.0),
			(1.4, 0, false, 1.0),
			(1.5, 0, true, 2.0),
			(1.5, 0, false, 1.0),
			(1.6, 0, true, 2.0),
			(1.6, 0, false, 2.0),
			(2.5, 0, true, 2.0),
			(2.5, 0, false, 3.0),
		];

		for test in &tests {
			assert_eq!(super::even_or_odd(test.0, test.1, test.2), test.3);
		}
	}

	#[test]
	fn floor() {
		let tests = [
			(-PI, 0, -4.0),
			(-PI, 1, -3.2),
			(-PI, 2, -3.15),
			(-PI, 3, -3.142),
			(-PI, 4, -3.1416),
			(-PI, 5, -3.1416),
			(-PI, 6, -3.141593),
			(-PI, 7, -3.1415927),
			(-PI, 8, -3.14159266),
			(-PI, 9, -3.141592654),

			(PI, 0, 3.0),
			(PI, 1, 3.1),
			(PI, 2, 3.14),
			(PI, 3, 3.141),
			(PI, 4, 3.1415),
			(PI, 5, 3.14159),
			(PI, 6, 3.141592),
			(PI, 7, 3.1415926),
			(PI, 8, 3.14159265),
			(PI, 9, 3.141592653),
		];

		for test in &tests {
			assert_eq!(super::floor(test.0, test.1), test.2);
		}
	}

	#[test]
	fn half_away_from_zero() {
		let tests = [
			(-1.7, 0, -2.0),
			(-1.5, 0, -2.0),
			(-0.5, 0, -1.0),
			(-0.2, 0, 0.0),

			(0.2, 0, 0.0),
			(0.5, 0, 1.0),
			(1.5, 0, 2.0),
			(1.7, 0, 2.0),
		];

		for test in &tests {
			assert_eq!(super::half_away_from_zero(test.0, test.1), test.2);
		}
	}

	#[test]
	fn half_down() {
		let tests = [
			(-1.7, 0, -2.0),
			(-1.5, 0, -2.0),
			(-0.5, 0, -1.0),
			(-0.2, 0, 0.0),

			(0.2, 0, 0.0),
			(0.5, 0, 0.0),
			(1.5, 0, 1.0),
			(1.7, 0, 2.0),
		];

		for test in &tests {
			assert_eq!(super::half_down(test.0, test.1), test.2);
		}
	}

	#[test]
	fn half_to_even() {
		let tests = [
			(-1.7, 0, -2.0),
			(-1.5, 0, -2.0),
			(-0.5, 0, -0.0),
			(-0.2, 0, 0.0),

			(0.2, 0, 0.0),
			(0.5, 0, 0.0),
			(1.5, 0, 2.0),
			(1.7, 0, 2.0),
		];

		for test in &tests {
			assert_eq!(super::half_to_even(test.0, test.1), test.2);
		}
	}

	#[test]
	fn half_to_odd() {
		let tests = [
			(-1.7, 0, -2.0),
			(-1.5, 0, -1.0),
			(-0.5, 0, -1.0),
			(-0.2, 0, 0.0),

			(0.2, 0, 0.0),
			(0.5, 0, 1.0),
			(1.5, 0, 1.0),
			(1.7, 0, 2.0),
		];

		for test in &tests {
			assert_eq!(super::half_to_odd(test.0, test.1), test.2);
		}
	}

	#[test]
	fn half_towards_zero() {
		let tests = [
			(-1.7, 0, -2.0),
			(-1.5, 0, -1.0),
			(-0.5, 0, -0.0),
			(-0.2, 0, 0.0),

			(0.2, 0, 0.0),
			(0.5, 0, 0.0),
			(1.5, 0, 1.0),
			(1.7, 0, 2.0),
		];

		for test in &tests {
			assert_eq!(super::half_towards_zero(test.0, test.1), test.2);
		}
	}

	#[test]
	fn half_up() {
		let tests = [
			(-1.7, 0, -2.0),
			(-1.5, 0, -1.0),
			(-0.5, 0, -0.0),
			(-0.2, 0, 0.0),

			(0.2, 0, 0.0),
			(0.5, 0, 1.0),
			(1.5, 0, 2.0),
			(1.7, 0, 2.0),
		];

		for test in &tests {
			assert_eq!(super::half_up(test.0, test.1), test.2);
		}
	}

	#[test]
	fn round() {
		let tests = [
			(-1.4, 0, true, -1.0),
			(-1.4, 0, false, -2.0),
			(-1.5, 0, true, -1.0),
			(-1.5, 0, false, -2.0),
			(-1.6, 0, true, -1.0),
			(-1.6, 0, false, -2.0),

			(1.4, 0, true, 2.0),
			(1.4, 0, false, 1.0),
			(1.5, 0, true, 2.0),
			(1.5, 0, false, 1.0),
			(1.6, 0, true, 2.0),
			(1.6, 0, false, 1.0),
		];

		for test in &tests {
			assert_eq!(super::round(test.0, test.1, test.2), test.3);
		}
	}

	#[test]
	fn to_nearest() {
		let tests = [
			(-1.4, 0, 0.4, -1.0),
			(-1.6, 0, 0.6, -2.0),

			(1.4, 0, 0.4, 1.0),
			(1.6, 0, 0.6, 2.0),
		];

		for test in &tests {
			assert_eq!(super::to_nearest(test.0, test.1, test.2), test.3);
		}
	}

	#[test]
	fn towards_zero() {
		let tests = [
			(-1.4, 0, true, -1.0),
			(-1.4, 0, false, -1.0),
			(-1.5, 0, true, -1.0),
			(-1.5, 0, false, -2.0),
			(-1.6, 0, true, -2.0),
			(-1.6, 0, false, -2.0),

			(1.4, 0, true, 1.0),
			(1.4, 0, false, 1.0),
			(1.5, 0, true, 1.0),
			(1.5, 0, false, 2.0),
			(1.6, 0, true, 2.0),
			(1.6, 0, false, 2.0),
		];

		for test in &tests {
			assert_eq!(super::towards_zero(test.0, test.1, test.2), test.3);
		}
	}
}
