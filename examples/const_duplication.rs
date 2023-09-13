//! This example illustrates how using a &str constant as the base value makes it easy to 
//! duplicate the same const value as an integer.
use intify_str::intify_str_unsigned;



const VERSION_STR: &'static str = "1";
const VERSION: u32 = intify_str_unsigned!(VERSION_STR);

pub fn main() {
    println!("str: {VERSION_STR}, int: {VERSION}");
}
