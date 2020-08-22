#[inline]
// pub fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }
pub fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

#[inline]
pub fn linear_search<T: Ord + std::fmt::Debug>(v: &[T], i: T) -> Result<usize, usize> {

    for (p, x) in v.iter().enumerate() {
        if *x == i {
            return Ok(p)
        } else if *x > i {
            return Err(p)
        }
    }
    Err(0)
}

#[inline]
pub fn binary_search<T: Ord + std::fmt::Debug>(v: &[T], i: T) -> Result<usize, usize> {
    let mut first = 0;
    let mut last = v.len();

    while first < last {
        let pivot = (first+last)>>1;
        if i == v[pivot] {
            return Ok(pivot)
        } else if i < v[pivot] {
            last = pivot - 1;
        } else {
            first = pivot + 1;
        }
        
    }
    Err((first+last)>>1 + 1)
}
