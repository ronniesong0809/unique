#[cfg(test)]
mod test_cases {

    use unique::Unique;

    #[test]
    pub fn empty() {
        let nums = vec![];
        let even: fn(&&usize) -> bool = |&&n| n % 2 == 0;
        assert_eq!(None, nums.iter().unique(even));
    }

    #[test]
    pub fn unique_even() {
        let mut nums = vec![];
        let even: fn(&&usize) -> bool = |&&n| n % 2 == 0;
        nums.push(1);
        nums.push(3);
        nums.push(4);
        nums.push(5);
        nums.push(7);
        nums.push(9);
        assert_eq!(Some(&4), nums.iter().unique(even));
    }

    #[test]
    pub fn unique_odd() {
        let mut nums = vec![];
        let odd: fn(&&usize) -> bool = |&&n| n % 2 != 0;
        nums.push(0);
        nums.push(2);
        nums.push(4);
        nums.push(5);
        nums.push(6);
        nums.push(8);
        assert_eq!(Some(&5), nums.iter().unique(odd));
    }

    #[test]
    pub fn nonunique_even_or_odd() {
        let mut nums = vec![];
        let even: fn(&&usize) -> bool = |&&n| n % 2 == 0;
        let odd: fn(&&usize) -> bool = |&&n| n % 2 != 0;
        nums.push(1);
        nums.push(2);
        nums.push(3);
        nums.push(4);
        assert_eq!(None, nums.iter().unique(even));
        assert_eq!(None, nums.iter().unique(odd));
    }

    #[test]
    pub fn unique_number() {
        let mut nums = vec![];
        let find_3: fn(&&usize) -> bool = |&&n| n == 3;
        nums.push(0);
        nums.push(0);
        nums.push(0);
        nums.push(0);
        nums.push(3);
        nums.push(0);
        assert_eq!(Some(&3), nums.iter().unique(find_3));
    }

    #[test]
    pub fn nonunique_number() {
        let mut nums = vec![];
        let find_0: fn(&&usize) -> bool = |&&n| n == 0;
        nums.push(0);
        nums.push(0);
        nums.push(0);
        nums.push(0);
        nums.push(3);
        nums.push(0);
        assert_eq!(None, nums.iter().unique(find_0));
    }

    #[test]
    pub fn unique_name() {
        let mut names = vec![];
        let find_ronnie: fn(&&&str) -> bool = |&&n| n == "Ronnie";
        names.push("Ronnie");
        names.push("John");
        names.push("David");
        names.push("David");
        assert_eq!(Some(&"Ronnie"), names.iter().unique(find_ronnie));
    }

    #[test]
    pub fn nonunique_name() {
        let mut names = vec![];
        let find_david: fn(&&&str) -> bool = |&&n| n == "David";
        names.push("Ronnie");
        names.push("John");
        names.push("David");
        names.push("David");
        assert_eq!(None, names.iter().unique(find_david));
    }
}
