use num_bigint::BigInt;
use num_traits::Pow;
use std::str::FromStr;

/// Multiplies two numbers using Karatsuba's multiplication method.
///
/// No error handling for simplicity and to focus on the algorithm, will panic if something fails.
pub fn multiply(x: &BigInt, y: &BigInt) -> BigInt {
    // base case: if we only have 2 digits, multiply them
    if x < &BigInt::from(100) || y < &BigInt::from(100) {
        return x * y;
    }

    let x_str = x.to_string();
    let y_str = y.to_string();

    let full_len: usize = std::cmp::min(x_str.len(), y_str.len());
    let half_len: usize = full_len.checked_div(2).expect("checked divison failed");

    let a = BigInt::from_str(&x_str[..half_len]).unwrap();
    let b = BigInt::from_str(&x_str[half_len..]).unwrap();
    let c = BigInt::from_str(&y_str[..half_len]).unwrap();
    let d = BigInt::from_str(&y_str[half_len..]).unwrap();

    // step 1, recursively compute a.c
    let ac = multiply(&a, &c);

    // step 2, recursively compute b.d
    let bd = multiply(&b, &d);

    // step 3, recursively compute (a+b)(c+d)
    let a_plus_b_c_plus_d = multiply(&(&a + &b), &(&c + &d));

    // Gauss' trick, (ad + bc) - ac - bd = ad + bc
    let ad_plus_bc = &a_plus_b_c_plus_d - &ac - &bd;

    // karatsuba: (10^n * ac) + 10^(n/2)(ad+bc) + (bd)
    ((BigInt::from(10).pow(full_len)) * ac) + ((BigInt::from(10).pow(half_len)) * ad_plus_bc) + bd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_case() {
        assert_eq!(
            multiply(&BigInt::from(1), &BigInt::from(2)),
            BigInt::from(2)
        );
    }

    #[test]
    fn test_medium_case() {
        assert_eq!(
            multiply(&BigInt::from(5678), &BigInt::from(1234)),
            BigInt::from(7006652)
        );
    }
}
