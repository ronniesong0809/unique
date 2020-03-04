use unique::Unique;

pub fn main() {
    let mut nums = vec![];
    let even: fn(&&usize) -> bool = |&&n| n % 2 == 0;
    assert_eq!(None, nums.iter().unique(even));
    nums.push(1);
    assert_eq!(None, nums.iter().unique(even));
    nums.push(0);
    assert_eq!(Some(&0), nums.iter().unique(even));
    nums.push(3);
    nums.push(2);
    nums.push(5);
    assert_eq!(None, nums.iter().unique(even));
}
