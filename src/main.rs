use fastnum::dec256;

mod vm;
use vm::*;

fn main() {
    let num_val = Val::number(dec256!(42).with_ctx(NO_TRAPS));
    let str_val = Val::string("Hello, BASIC!");

    println!("Number: {:?}", num_val);
    println!("Number: {}", num_val);
    println!("String: {:?}", str_val);

    let ten = Val::number(dec256!(10.0).with_ctx(NO_TRAPS));
    let zero = Val::number(dec256!(0.0).with_ctx(NO_TRAPS));

    match ten / zero {
        Ok(result) => println!("Result: {:?}", result),
        Err(error) => println!("Error: {:?}", error),
    }
}
