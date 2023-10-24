fn power(base: u32, mut exponent: u32) -> u32 {
    let mut result = 1;

    while exponent > 0 {
        // If the least significant bit is 1, multiply the result by base
        if exponent & 1 != 0 {
            result *= base;
        }

        // Shift bits to the right to consider the next bit
        exponent >>= 1;

        // Square the base (equivalent to raising it to the power of 2)
        base *= base;
    }

    result
}

fn main() {
    let base_number = 2;
    let exponent_value = 5;
    let result = power(base_number, exponent_value);
    println!("{}^{} is: {}", base_number, exponent_value, result);
}
