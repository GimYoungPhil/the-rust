fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {guess}");

    let signed = -100i8;
    println!("signed: {signed}");

    let unsigned = 100u8;
    println!("unsigned: {unsigned}");

    let arch = 100usize;
    println!("arch: {arch}");
}
