
/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut bools = vec![false, false];
    for i in 2..(n+1) {
        bools.push(true);
    }
    // println!("{:?}", bools);
    for i in 0..(n/2) {
        let index = i as usize;
        if bools[index] {
            let mut j = index * index;
            let limit = n as usize;
            while j <= limit {
                bools[j] = false;
                j = j + index;
            }
        }
    }
    let mut v = Vec::new();
    for (i, prime) in bools.into_iter().enumerate() {
        if prime {
            v.push(i as u32)
        }
    }
    v
}

fn is_prime(n: u32) -> bool {
    // Two is a prime
    if n == 2 {
        return true;
    }
    // Work up to sqrt(n - 1) to see if prime
    let root = n / 2;
    // let root = fl.sqrt() as u32;
    let mut i = 2;
    while i < root {
        if n % i == 0 {
            return false;
        } else {
            i += 1;
        }
    }
    return true;

    
}