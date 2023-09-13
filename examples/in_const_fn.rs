//! This example show that it is possible to use these macros from a `const fn`
use intify_str::intify_str_unsigned;

const SOME_NUM: u32 = try_me();

const fn try_me() -> u32 {
    intify_str_unsigned!("1")
}

fn main() {
    println!("{}/{SOME_NUM}", try_me());
}
