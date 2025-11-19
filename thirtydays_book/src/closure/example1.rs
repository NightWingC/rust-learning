fn main() {
    let mut count = 0;
    let multiplier = 2;

    let mut calculate = || {
        count += multiplier * 3;
        println!("Internal count: {}", count);
    };

    calculate();
    calculate();

    println!("Final count: {}", count);
}