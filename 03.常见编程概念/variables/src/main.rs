fn main() {
    let mut x1 = 5;
    println!("The value of x1 is: {x1}");
    x1 = 6;
    println!("The value of x1 is: {x1}");

    // ----

    let x2 = 5;

    let x2 = x2 + 1;
    
    {
        let x2 = x2 * 2;
        println!("The value of x2 in the inner scope is: {x2}");
    }

    println!("The value of x is: {x2}");

    // ---

    let _spaces = "   ";
    let _spaces = _spaces.len();
}

