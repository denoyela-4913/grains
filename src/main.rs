pub fn square(s: u32) -> u64 {
    // Number of grains on square s is 2^(s-1)
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // Sum of grains on all 64 squares is 2^64 - 1
    // 2^0 + 2^1 + ... + 2^63 = 2^64 - 1
    /* let mut sum: u64 = 0;
    for i in 1..=64 {
        sum += square(i);
    }
    sum */
    (2u128.pow(64) - 1) as u64
}



fn main() {
    println!("Hello, world! Grains (of sand on a chessboard.)");
    let a = square (12);
    println!("Grains on square 12: {a}");
    let b = total();
    println!("Total grains on the chessboard: {b}");    
}
