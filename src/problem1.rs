// problem1.rs

/// Functions are  private (only availible to this module) by default
/// use the `pub` keyword to mark this function public
pub fn sum(slice: &[i32]) -> i32 {
    let mut total = 0;
    for n in slice {
        total = total + n;
    }
    total
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut v = Vec::new();
    for el in vs {
        if !v.contains(el) {
            v.push(*el)
        }
    }
    v
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut v = Vec::new();
    for el in vs {
        if pred(*el) {
            v.push(*el)
        }
    }
    v
}