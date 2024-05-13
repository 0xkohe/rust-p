fn main() -> Result<(), String> {
    fn incremented(n:i32) -> Result<i32, String> {
        if n < 0 {
            Err("Negative number".to_string())
        } else {
            Ok(n + 1)
        }
    }

    println!("{:?}", incremented(1)?);
    println!("{:?}", incremented(-3)?);
    println!("{:?}", incremented(1)?);
    Ok(())
}
