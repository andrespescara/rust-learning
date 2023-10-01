fn main() {
    let x = 5;

    println!("The value of x is: {x}");

    let x = x + 1;

    println!("The value of x is: {x}");

    // inner definitions doesnt affect outer if not defined
    {
        let x = x * 2;
        println!("The value of inner x is: {x}");
    }

    println!("But the outer value of x is still: {x}");

    // this works:
    let spaces = "   ";
    let spaces = spaces.len();

    // this doesnt:
    //let mut spaces = "   ";
    //spaces = spaces.len();

    // mut lets you change the value but no the type
    // let lets you change value and type because you only reuse the var name
}
