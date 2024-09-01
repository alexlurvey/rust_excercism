#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base == 1 || from_base == 0 {
        return Err(Error::InvalidInputBase);
    }

    if to_base == 1 || to_base == 0 {
        return Err(Error::InvalidOutputBase);
    }

    if number.is_empty() || number.iter().all(|x| *x == 0) {
        return Ok(vec![0]);
    }

    if let Some(invalid_digit) = number.iter().find(|x| **x >= from_base) {
        return Err(Error::InvalidDigit(*invalid_digit));
    }

    let mut val = number.iter().enumerate().fold(0i32, |acc, (i, x)| {
        let pow = (number.len() - i - 1) as u32;
        let add = *x * from_base.pow(pow);
        acc + add as i32
    });

    let mut result: Vec<u32> = vec![];
    let mut idx = (val as f32).log(to_base as f32).floor() as u32 + 1;

    while val > 0 {
        idx -= 1;

        let n = (0..to_base).rev().fold(0, |acc, x| {
            if acc > 0 {
                return acc;
            }

            let check: i32 = x as i32 * (to_base.pow(idx) as i32);

            if val - check >= 0 {
                return x;
            }

            acc
        });

        result.push(n);

        val -= (n as i32) * (to_base.pow(idx) as i32);
    }

    while idx > 0 {
        result.push(0);
        idx -= 1;
    }

    Ok(result)
}
