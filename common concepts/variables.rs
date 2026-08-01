fn main() {
    // Variables are immutable by default
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 10;
    println!("The value of x is: {x}");

    // Constants are always immutable and must have a type annotation
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");

    let y = 5;
    let y = y + 1; // Shadowing allows us to reuse the variable name
    {
        let y = y * 2; // This y is a new variable, shadowing the previous y
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y in the outer scope is: {y}");
}
