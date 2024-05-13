fn main() {
    f(&3.1);

    use std::mem::*;
    println!("{} {} {} {}",
        size_of::<isize>(),
        size_of::<usize>(),
        size_of::<&i8>(),
        size_of::<&u32>(),
        );

    let mut v = Vec::with_capacity(800);
    let mut prev_capacity = std::usize::MAX;
    for i in 0..1_000 {
        let cap = v.capacity();
        if cap != prev_capacity {
            println!("{}: {}", i, cap);
            prev_capacity = cap;
        }
        v.push("a");
    }

    let v = vec![1,2,3];
    for i in v {
        println!("{}", i);
    }

}

fn f(p: &f64) {
    let a = Box::new(*p);
    {
        let b = Box::new([1,2,3]);
        println!("{} {:?}", *a, *b);
    }

    let c = Box::new(true);
    println!("{} {}", a, c);
}
