/// Partition a slice in place
/// Returns the `pivot` index (the first index that was on the second partition)
pub fn partition<T, P>(data: &mut [T], predicate: P) -> usize
where
    P: Fn(&T) -> bool,
{
    let len = data.len();
    if len == 0 {
        return 0;
    }
    let (mut l, mut r) = (0, len - 1);
    loop {
        while l < len && predicate(&data[l]) {
            l += 1;
        }
        while r > 0 && !predicate(&data[r]) {
            r -= 1;
        }
        if l >= r {
            return l;
        }
        data.swap(l, r);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn partition() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let ret = super::partition(&mut v, |n| n % 2 == 0);
        assert_eq!(v, vec![8, 2, 6, 4, 5, 3, 7, 1, 9]);
        assert_eq!(ret, 4);

        let mut v = vec![9, 9, 9, 9, 1, 1, 2, 2];
        let ret = super::partition(&mut v, |n| n % 2 == 0);
        assert_eq!(ret, 2);
    }
}
