fn sum(a: &[u32]) -> u32{
    let mut sum = 0;
    for v in a {
        sum += v;
    }
    sum
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = [2, 0,12, 2, 3, 4, 5];
    let c = [];
    println!("Sum: {}", sum(&a));
    println!("Sum: {}", sum(&b));
    println!("Sum: {}", sum(&c));
}
