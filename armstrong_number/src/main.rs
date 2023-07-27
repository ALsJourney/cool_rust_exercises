fn main() {
    println!("{}", is_armstrong_number(127));
}

fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;

    let mut sum: u32 = 0;

    for digit_char in num_str.chars() {
        // Convert digit character to a number
        let digit = digit_char.to_digit(10).expect("Invalid digit found");

        // Check for overflow errors
        match digit.checked_pow(num_digits) {
            Some(digit_powered) => {
                // Add the digit raised to the power of num_digits to the sum
                //
                match sum.checked_add(digit_powered) {
                    Some(new_sum) => sum = new_sum,
                    None => return false, // Overflow occured
                }
            }
            None => return false,
        }
    }
    num == sum
}
