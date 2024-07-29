use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::from_iter([2, 3, 5, 7, 11, 13]);

    if n < primes.len() as u32 {
        return primes[n as usize];
    }

    while primes.len() - 1 < n as usize {
        let prior_prime = primes.last().unwrap().to_owned();

        for x in (prior_prime + 1).. {
            let mut is_prime = true;

            for y in 2..prior_prime {
                if x % y == 0 {
                    is_prime = false;
                    break;
                }
            }

            if is_prime {
                primes.push(x);
                break;
            }
        }
    }

    primes[n as usize]
}
