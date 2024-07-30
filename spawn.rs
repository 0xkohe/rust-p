fn main() {
    let handler = std::thread::spawn(worker);

    match handler.join() {
        Ok(result) => {
            println!("result: {}", result);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    
    }
}

fn worker() -> u32 {
    println!("worker");
    100
}
