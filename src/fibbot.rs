use num_bigint::{BigUint, ToBigUint};

/// Computes the nth Fibonacci number using BigInt.
pub fn fibonacci(n: u32) -> BigUint {
    let mut a = 0.to_biguint().unwrap();
    let mut b = 1.to_biguint().unwrap();

    if n == 0 {
        a
    } else if n == 1 {
        b
    } else {
        for _ in 2..=n {
            let next = &a + &b;
            a = b;
            b = next;
        }

        b
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            fibonacci(100),
            BigUint::from_str("354224848179261915075").unwrap()
        );
        assert_eq!(
            fibonacci(1000),
            BigUint::from_str("43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875").unwrap()
        );
    }
}
