fn main() {
    let mut text = String::from("first: ");
    //let mut text = format!("first: ");
    let inp = std::io::stdin();
    inp.read_line(&mut text).unwrap();
    text.push_str("second: ");
    inp.read_line(&mut text).unwrap();
    println!("{}: {} bytes", text, text.len());
}
