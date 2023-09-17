// Add this to your Cargo.toml:
// [dependencies]
// anyhow = "1.0"

use anyhow::{Result, anyhow};

fn divide(a: i32, b: i32) -> Result<f32> {
    if b == 0 {
        // Create an error using the `anyhow!` macro
        return Err(anyhow!("Division by zero is not allowed"));
    }

    let result = a as f32 / b as f32;
    Ok(result)
}

fn main() -> Result<()> {
    let a = 10;
    let b = 0;

    // Attempt to divide and handle errors
    let result = divide(a, b)?;

    println!("Result: {}", result);

    Ok(())
}
