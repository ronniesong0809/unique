use unique::Unique;

pub fn main() {
    unique_num();
    unique_name();
}

fn unique_num() {
    let mut nums = vec![];
    let even: fn(&&usize) -> bool = |&&n| n % 2 == 0;
    let odd: fn(&&usize) -> bool = |&&n| n % 2 != 0;
    let find_3: fn(&&usize) -> bool = |&&n| n == 3;

    println!("{:?}", nums.iter().unique(even));
    println!("{:?}", nums.iter().unique(odd));
    nums.push(1);
    nums.push(0);
    println!("{:?}", nums.iter().unique(even));
    println!("{:?}", nums.iter().unique(odd));
    nums.push(2);
    nums.push(5);
    println!("{:?}", nums.iter().unique(even));
    println!("{:?}", nums.iter().unique(odd));
    nums.push(3);
    println!("{:?}", nums.iter().unique(find_3));
    nums.push(3);
    println!("{:?}", nums.iter().unique(find_3));
}

fn unique_name() {
    let mut names = vec![];
    let find_ronnie: fn(&&&str) -> bool = |&&n| n == "Ronnie";
    let find_david: fn(&&&str) -> bool = |&&n| n == "David";

    names.push("Ronnie");
    names.push("John");
    names.push("David");
    println!("{:?}", names.iter().unique(find_ronnie));
    println!("{:?}", names.iter().unique(find_david));
    names.push("David");
    println!("{:?}", names.iter().unique(find_ronnie));
    println!("{:?}", names.iter().unique(find_david));
}
