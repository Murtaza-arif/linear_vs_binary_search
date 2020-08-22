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
