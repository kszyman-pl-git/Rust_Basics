fn main() {
    let mut nums = vec![];
    nums.push(10);
    nums.push(20);
    nums.push(30);
    println!("{:?}", nums);

    let pop = nums.pop();
    println!("{:?}", pop);
    println!("{:?}", nums);
}