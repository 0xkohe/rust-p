fn main() {
    println!("Hello, world!");
    print!("{}, {}","Hello","world\n");
    println!("{}", 000140);
    println!("{}", 2.7 * 1.);
    println!("{}", 80.3 + 34.9);
    println!("{}", "test \
         aiueo \
         aaa
             a");

    let number = 12;
    let otehr_number = 53;
    println!("{}", number + otehr_number);

    let mut number_2 = 12;

    println!("{}", number_2);
    number_2 = 53;
    println!("{}", number_2 + otehr_number);

    println!("{} {}", str::len("abcde"), "abcde".len());

    for n in 1..11 {
        print!("{} ", n*n);
    }
    println!();
    for n in 1..=10 {
        print!("{} ", n*n);
    }
    println!();

    let mut limit = 4;
    for  n in 1..limit + 2 {
        limit -=1;
        print!("{} {}, ", limit, n);
    }
    println!();
    println!("{}", limit);

    let mut x = [1.; 5000];
    x[2000] = 3.14;
    println!("{}, {}", x[1000], x[2000]);

    let mut fib = [1; 12];
    for i in 2..fib.len() {
        fib[i] = fib[i-2] + fib[i-1];
    }

    for i in 0..fib.len() {
        print!("{} ", fib[i]);
    }

    println!();
    let mut x = [[[23; 4]; 8]; 15];
    x[14][7][3] = 56;
    println!("{}, {}",x[0][0][0], x[14][7][3]);

    let x = vec!["this", "is"];
    println!("{}, {}. length: {}", x[0], x[1], x.len());
    println!("{:?} {:?}", [1,2,3],vec![4.5]);

    #[allow(dead_code)]
    enum Continent {
        Asia,
        Europe,
        Africa,
        America,
        Oceania,
    }
    let mut contain = Continent::Asia;
    #[allow(unused_assignments)]
    match contain {
        Continent::Asia => {
            contain = Continent::Asia;
            println!("E");
        },
        Continent::Europe => println!("Europe"),
        Continent::Africa => println!("Africa"),
        Continent::America => println!("America"),
        _ => {},
    }

    for n in -2..5 {
        println!("{} is {}", n,
                 match n {
                     0 => "zero",
                     1 => "one",
                     _ if n < 0 => "negative",
                     _ => "positive",
                 }
                 );
    }

    struct SomeData {
        integer: i32,
        fractional: f32,
        character: char,
        five_bytes: [u8;5],
    }
    let data = SomeData {
        integer: 123,
        fractional: 3.14,
        character: 'R',
        five_bytes: [0,1,2,3,4],
    };

    println!("{} {} {} {:?}", data.integer, data.fractional, data.character, data.five_bytes);

    struct NoData{}
    let _no_data = NoData{};

    f();

    show_devide(3., 2.);
    show_devide(3., 0.);

    let r1 = devide(8.,2.);
    if r1.is_ok() {
        println!("Result: {}", r1.unwrap());
    } 

}
/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
fn show_devide(num:f64, den:f64) {
    match devide(num, den) {
        Ok(result) => println!("Result: {}", result),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn devide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Division by zero: {}", numerator))
    } else {
        Ok(numerator / denominator)
    }
}

fn f() {println!("1");}
fn _f1() -> i32 {
    4.5;
    "abc";
    73i32
}
