fn bubble(xs: &mut [i32])  {
    for i in 0..xs.len() {
        for i2 in 0..xs.len() - (i + 1) {
            if xs[i2] > xs[i2 + 1] {
                xs.swap(i2, i2 + 1);
            }
        }
    }
}

fn main() {
    let mut xs = [4, 2, 5, 1, 3];
    bubble(&mut xs);
    println!("{:?}", xs);

    let mut xs = [-4, 2, 5, 1, 3];
    bubble(&mut xs);
    println!("{:?}", xs);
}
