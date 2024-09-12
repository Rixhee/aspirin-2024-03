//! FizzBuzz implementation

pub fn print_fizz_buzz(max_num: u32) {
    /// This function prints runs through the numbers from 1 to max_num, and
    /// print "Fizz" if the number is divisible by 3, "Buzz" if the number is
    /// divisible by 5, and "FizzBuzz" if the number is divisible by both 3 and
    /// 5. Otherwise, it prints the number itself.
    ///
    /// # Arguments
    ///
    /// * `max_num`: u32 - The maximum number to check
    ///
    /// # Returns
    ///
    /// None
    for i in 1..=max_num {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
