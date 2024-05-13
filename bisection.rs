fn f(x: f64, r: f64) -> f64 {
    x * x - r
}
fn bisection(mut a: f64, mut b:f64, r:f64) -> f64 {
    let e = 1e-7;
    while (a-b).abs() > e {
        let c = (a+b)/2.;
        if f(a,r) * f(c,r) < 0. {
            b = c;
        } else {
            a = c;
        }
    }
    (a+b)/2.
}

fn bisection_rec(a:f64, b:f64, r:f64) -> f64 {
    let e = 1e-7;
    if (a-b).abs() < e {
        return (a+b)/2.;
    }
    let c = (a+b)/2.;
    if f(a,r) * f(c,r) < 0. {
        bisection_rec(a,c,r)
    } else {
        bisection_rec(c,b,r)
    }
}

fn main() {
    let r = 2.;
    let a = 0.;
    let b = r;
    let x = bisection(a,b,r);
    println!("x = {} (loop)", x);

    let x = bisection_rec(a,b,r);
    println!("x = {} (rec)", x);
}
