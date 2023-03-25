fn sum(nums: &[u32]) -> Option<u32> {
    let mut r = 0u32;
    for &n in nums {
        match r.checked_add(n) {
            Some(add_ret) => r = add_ret,
            None => return None,
        }
    }
    Some(r)
}

#[test]
fn test_sum() {
    let nums = [1, 2, 3, 4];
    assert_eq!(sum(&nums), Some(10));
    println!("Input nums : {:?}", nums);
    println!("SUM Result: {:?}", sum(&nums));
}

#[test]
fn test_sum_overflow() {
    let nums_max = [u32::max_value(), 1];
    assert_eq!(sum(&nums_max), None);
    println!("Input nums : {:?}", nums_max);
    println!("SUM Result: {:?}", sum(&nums_max));
}
