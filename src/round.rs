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
	let digits = significant_digits(value, scale);
	match digits.1 == 5 {
		true => round(value, scale, up),
		false => to_nearest(value, scale, digits.1),
	}
}

#[cfg(test)]
mod tests {
	use std::f64::{ NAN, INFINITY, NEG_INFINITY };

	macro_rules! test_round {
		($func:path [ $($name:ident: $params:expr,)* ]) => {
		$(
			#[test]
			fn $name() {
				let (value, scale, expected): (f64, u8, f64) = $params;
				let result = $func(value, scale);
				match result.is_nan() {
					true => assert_eq!(expected.is_nan(), true),
					false => assert_eq!(result, expected),
				}
			}
		)*
		}
	}

	macro_rules! test_digits {
		($func:path [ $($name:ident: $params:expr,)* ]) => {
		$(
			#[test]
			fn $name() {
				let (value, scale, expected): (f64, u8, (u8, u8)) = $params;
				assert_eq!($func(value, scale), expected);
			}
		)*
		}
	}

	test_round! { super::ceil [
		ceil_1: (-1.03, 1, -1.),
		ceil_2: (-1.05, 1, -1.),
		ceil_3: (-1.07, 1, -1.),
		ceil_4: (-1.13, 1, -1.1),
		ceil_5: (-1.15, 1, -1.1),
		ceil_6: (-1.17, 1, -1.1),
		ceil_7: (-1.23, 1, -1.2),
		ceil_8: (-1.25, 1, -1.2),
		ceil_9: (-1.27, 1, -1.2),
		ceil_10: (-1.33, 1, -1.3),
		ceil_11: (-1.35, 1, -1.3),
		ceil_12: (-1.37, 1, -1.3),
		ceil_13: (1.03, 1, 1.1),
		ceil_14: (1.05, 1, 1.1),
		ceil_15: (1.07, 1, 1.1),
		ceil_16: (1.13, 1, 1.2),
		ceil_17: (1.15, 1, 1.2),
		ceil_18: (1.17, 1, 1.2),
		ceil_19: (1.23, 1, 1.3),
		ceil_20: (1.25, 1, 1.3),
		ceil_21: (1.27, 1, 1.3),
		ceil_22: (1.33, 1, 1.4),
		ceil_23: (1.35, 1, 1.4),
		ceil_24: (1.37, 1, 1.4),
		ceil_25: (INFINITY, 1, INFINITY),
		ceil_26: (NAN, 1, NAN),
		ceil_27: (NEG_INFINITY, 1, NEG_INFINITY),
	]}

	test_round! { super::floor [
		floor_1: (-1.03, 1, -1.1),
		floor_2: (-1.05, 1, -1.1),
		floor_3: (-1.07, 1, -1.1),
		floor_4: (-1.13, 1, -1.2),
		floor_5: (-1.15, 1, -1.2),
		floor_6: (-1.17, 1, -1.2),
		floor_7: (-1.23, 1, -1.3),
		floor_8: (-1.25, 1, -1.3),
		floor_9: (-1.27, 1, -1.3),
		floor_10: (-1.33, 1, -1.4),
		floor_11: (-1.35, 1, -1.4),
		floor_12: (-1.37, 1, -1.4),
		floor_13: (1.03, 1, 1.),
		floor_14: (1.05, 1, 1.),
		floor_15: (1.07, 1, 1.),
		floor_16: (1.13, 1, 1.1),
		floor_17: (1.15, 1, 1.1),
		floor_18: (1.17, 1, 1.1),
		floor_19: (1.23, 1, 1.2),
		floor_20: (1.25, 1, 1.2),
		floor_21: (1.27, 1, 1.2),
		floor_22: (1.33, 1, 1.3),
		floor_23: (1.35, 1, 1.3),
		floor_24: (1.37, 1, 1.3),
		floor_25: (INFINITY, 1, INFINITY),
		floor_26: (NAN, 1, NAN),
		floor_27: (NEG_INFINITY, 1, NEG_INFINITY),
	]}

	test_round! { super::half_away_from_zero [
		half_away_from_zero_1: (-1.03, 1, -1.),
		half_away_from_zero_2: (-1.05, 1, -1.1),
		half_away_from_zero_3: (-1.07, 1, -1.1),
		half_away_from_zero_4: (-1.13, 1, -1.1),
		half_away_from_zero_5: (-1.15, 1, -1.2),
		half_away_from_zero_6: (-1.17, 1, -1.2),
		half_away_from_zero_7: (-1.23, 1, -1.2),
		half_away_from_zero_8: (-1.25, 1, -1.3),
		half_away_from_zero_9: (-1.27, 1, -1.3),
		half_away_from_zero_10: (-1.33, 1, -1.3),
		half_away_from_zero_11: (-1.35, 1, -1.4),
		half_away_from_zero_12: (-1.37, 1, -1.4),
		half_away_from_zero_13: (1.03, 1, 1.),
		half_away_from_zero_14: (1.05, 1, 1.1),
		half_away_from_zero_15: (1.07, 1, 1.1),
		half_away_from_zero_16: (1.13, 1, 1.1),
		half_away_from_zero_17: (1.15, 1, 1.2),
		half_away_from_zero_18: (1.17, 1, 1.2),
		half_away_from_zero_19: (1.23, 1, 1.2),
		half_away_from_zero_20: (1.25, 1, 1.3),
		half_away_from_zero_21: (1.27, 1, 1.3),
		half_away_from_zero_22: (1.33, 1, 1.3),
		half_away_from_zero_23: (1.35, 1, 1.4),
		half_away_from_zero_24: (1.37, 1, 1.4),
		half_away_from_zero_25: (INFINITY, 1, INFINITY),
		half_away_from_zero_26: (NAN, 1, NAN),
		half_away_from_zero_27: (NEG_INFINITY, 1, NEG_INFINITY),
	]}

	test_round! { super::half_down [
		half_down_1: (-1.03, 1, -1.),
		half_down_2: (-1.05, 1, -1.1),
		half_down_3: (-1.07, 1, -1.1),
		half_down_4: (-1.13, 1, -1.1),
		half_down_5: (-1.15, 1, -1.2),
		half_down_6: (-1.17, 1, -1.2),
		half_down_7: (-1.23, 1, -1.2),
		half_down_8: (-1.25, 1, -1.3),
		half_down_9: (-1.27, 1, -1.3),
		half_down_10: (-1.33, 1, -1.3),
		half_down_11: (-1.35, 1, -1.4),
		half_down_12: (-1.37, 1, -1.4),
		half_down_13: (1.03, 1, 1.),
		half_down_14: (1.05, 1, 1.),
		half_down_15: (1.07, 1, 1.1),
		half_down_16: (1.13, 1, 1.1),
		half_down_17: (1.15, 1, 1.1),
		half_down_18: (1.17, 1, 1.2),
		half_down_19: (1.23, 1, 1.2),
		half_down_20: (1.25, 1, 1.2),
		half_down_21: (1.27, 1, 1.3),
		half_down_22: (1.33, 1, 1.3),
		half_down_23: (1.35, 1, 1.3),
		half_down_24: (1.37, 1, 1.4),
		half_down_25: (INFINITY, 1, INFINITY),
		half_down_26: (NAN, 1, NAN),
		half_down_27: (NEG_INFINITY, 1, NEG_INFINITY),
	]}

	test_round! { super::half_to_even [
		half_to_even_1: (-1.03, 1, -1.),
		half_to_even_2: (-1.05, 1, -1.),
		half_to_even_3: (-1.07, 1, -1.1),
		half_to_even_4: (-1.13, 1, -1.1),
		half_to_even_5: (-1.15, 1, -1.2),
		half_to_even_6: (-1.17, 1, -1.2),
		half_to_even_7: (-1.23, 1, -1.2),
		half_to_even_8: (-1.25, 1, -1.2),
		half_to_even_9: (-1.27, 1, -1.3),
		half_to_even_10: (-1.33, 1, -1.3),
		half_to_even_11: (-1.35, 1, -1.4),
		half_to_even_12: (-1.37, 1, -1.4),
		half_to_even_13: (1.03, 1, 1.),
		half_to_even_14: (1.05, 1, 1.),
		half_to_even_15: (1.07, 1, 1.1),
		half_to_even_16: (1.13, 1, 1.1),
		half_to_even_17: (1.15, 1, 1.2),
		half_to_even_18: (1.17, 1, 1.2),
		half_to_even_19: (1.23, 1, 1.2),
		half_to_even_20: (1.25, 1, 1.2),
		half_to_even_21: (1.27, 1, 1.3),
		half_to_even_22: (1.33, 1, 1.3),
		half_to_even_23: (1.35, 1, 1.4),
		half_to_even_24: (1.37, 1, 1.4),
		half_to_even_25: (INFINITY, 1, INFINITY),
		half_to_even_26: (NAN, 1, NAN),
		half_to_even_27: (NEG_INFINITY, 1, NEG_INFINITY),
		half_to_even_28: (2.221000, 3, 2.221),
	]}

	test_round! { super::half_to_odd [
		half_to_odd_1: (-1.03, 1, -1.),
		half_to_odd_2: (-1.05, 1, -1.1),
		half_to_odd_3: (-1.07, 1, -1.1),
		half_to_odd_4: (-1.13, 1, -1.1),
		half_to_odd_5: (-1.15, 1, -1.1),
		half_to_odd_6: (-1.17, 1, -1.2),
		half_to_odd_7: (-1.23, 1, -1.2),
		half_to_odd_8: (-1.25, 1, -1.3),
		half_to_odd_9: (-1.27, 1, -1.3),
		half_to_odd_10: (-1.33, 1, -1.3),
		half_to_odd_11: (-1.35, 1, -1.3),
		half_to_odd_12: (-1.37, 1, -1.4),
		half_to_odd_13: (1.03, 1, 1.),
		half_to_odd_14: (1.05, 1, 1.1),
		half_to_odd_15: (1.07, 1, 1.1),
		half_to_odd_16: (1.13, 1, 1.1),
		half_to_odd_17: (1.15, 1, 1.1),
		half_to_odd_18: (1.17, 1, 1.2),
		half_to_odd_19: (1.23, 1, 1.2),
		half_to_odd_20: (1.25, 1, 1.3),
		half_to_odd_21: (1.27, 1, 1.3),
		half_to_odd_22: (1.33, 1, 1.3),
		half_to_odd_23: (1.35, 1, 1.3),
		half_to_odd_24: (1.37, 1, 1.4),
		half_to_odd_25: (INFINITY, 1, INFINITY),
		half_to_odd_26: (NAN, 1, NAN),
		half_to_odd_27: (NEG_INFINITY, 1, NEG_INFINITY),
	]}

	test_round! { super::half_towards_zero [
		half_towards_zero_1: (-1.03, 1, -1.),
		half_towards_zero_2: (-1.05, 1, -1.),
		half_towards_zero_3: (-1.07, 1, -1.1),
		half_towards_zero_4: (-1.13, 1, -1.1),
		half_towards_zero_5: (-1.15, 1, -1.1),
		half_towards_zero_6: (-1.17, 1, -1.2),
		half_towards_zero_7: (-1.23, 1, -1.2),
		half_towards_zero_8: (-1.25, 1, -1.2),
		half_towards_zero_9: (-1.27, 1, -1.3),
		half_towards_zero_10: (-1.33, 1, -1.3),
		half_towards_zero_11: (-1.35, 1, -1.3),
		half_towards_zero_12: (-1.37, 1, -1.4),
		half_towards_zero_13: (1.03, 1, 1.),
		half_towards_zero_14: (1.05, 1, 1.),
		half_towards_zero_15: (1.07, 1, 1.1),
		half_towards_zero_16: (1.13, 1, 1.1),
		half_towards_zero_17: (1.15, 1, 1.1),
		half_towards_zero_18: (1.17, 1, 1.2),
		half_towards_zero_19: (1.23, 1, 1.2),
		half_towards_zero_20: (1.25, 1, 1.2),
		half_towards_zero_21: (1.27, 1, 1.3),
		half_towards_zero_22: (1.33, 1, 1.3),
		half_towards_zero_23: (1.35, 1, 1.3),
		half_towards_zero_24: (1.37, 1, 1.4),
		half_towards_zero_25: (INFINITY, 1, INFINITY),
		half_towards_zero_26: (NAN, 1, NAN),
		half_towards_zero_27: (NEG_INFINITY, 1, NEG_INFINITY),
	]}

	test_round! { super::half_up [
		half_up_1: (-1.03, 1, -1.),
		half_up_2: (-1.05, 1, -1.),
		half_up_3: (-1.07, 1, -1.1),
		half_up_4: (-1.13, 1, -1.1),
		half_up_5: (-1.15, 1, -1.1),
		half_up_6: (-1.17, 1, -1.2),
		half_up_7: (-1.23, 1, -1.2),
		half_up_8: (-1.25, 1, -1.2),
		half_up_9: (-1.27, 1, -1.3),
		half_up_10: (-1.33, 1, -1.3),
		half_up_11: (-1.35, 1, -1.3),
		half_up_12: (-1.37, 1, -1.4),
		half_up_13: (1.03, 1, 1.),
		half_up_14: (1.05, 1, 1.1),
		half_up_15: (1.07, 1, 1.1),
		half_up_16: (1.13, 1, 1.1),
		half_up_17: (1.15, 1, 1.2),
		half_up_18: (1.17, 1, 1.2),
		half_up_19: (1.23, 1, 1.2),
		half_up_20: (1.25, 1, 1.3),
		half_up_21: (1.27, 1, 1.3),
		half_up_22: (1.33, 1, 1.3),
		half_up_23: (1.35, 1, 1.4),
		half_up_24: (1.37, 1, 1.4),
		half_up_25: (INFINITY, 1, INFINITY),
		half_up_26: (NAN, 1, NAN),
		half_up_27: (NEG_INFINITY, 1, NEG_INFINITY),
	]}

	test_digits! { super::significant_digits [
		significant_digits_1: (-1.1234567890, 0, (1, 1)),
		significant_digits_2: (-1.1234567890, 1, (1, 2)),
		significant_digits_3: (-1.1234567890, 2, (2, 3)),
		significant_digits_4: (-1.1234567890, 3, (3, 4)),
		significant_digits_5: (-1.1234567890, 4, (4, 5)),
		significant_digits_6: (-1.1234567890, 5, (5, 6)),
		significant_digits_7: (-1.1234567890, 6, (6, 7)),
		significant_digits_8: (-1.1234567890, 7, (7, 8)),
		significant_digits_9: (-1.1234567890, 8, (8, 9)),
		significant_digits_10: (-1.1234567890, 9, (9, 0)),
		significant_digits_11: (1.1234567890, 0, (1, 1)),
		significant_digits_12: (1.1234567890, 1, (1, 2)),
		significant_digits_13: (1.1234567890, 2, (2, 3)),
		significant_digits_14: (1.1234567890, 3, (3, 4)),
		significant_digits_15: (1.1234567890, 4, (4, 5)),
		significant_digits_16: (1.1234567890, 5, (5, 6)),
		significant_digits_17: (1.1234567890, 6, (6, 7)),
		significant_digits_18: (1.1234567890, 7, (7, 8)),
		significant_digits_19: (1.1234567890, 8, (8, 9)),
		significant_digits_20: (1.1234567890, 9, (9, 0)),
		significant_digits_21: (-1.15, 1, (1, 5)),
		significant_digits_22: (1.15, 1, (1, 5)),
		significant_digits_23: (1.9999, 3, (9, 9)),
		significant_digits_24: (INFINITY, 1, (0, 0)),
		significant_digits_25: (NAN, 1, (0, 0)),
		significant_digits_26: (NEG_INFINITY, 1, (0, 0)),
	]}
}
