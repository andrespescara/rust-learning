use std::io;

// Floats
fn floats_dt() {
    let x = 2.0;

    let y: f32 = 3.5;

    println!("The sum of {x} and {y} is: {}", x + y)
}

// Operations
fn operations_dt() {
    // Operations must be of the same dt
    // floats with floats and
    // int with int
    let _sum = 5 + 10; // use _ prefix for unnused vars

    let _difference = 102.2 - 8.2;

    let _product = 15 * 4;

    // divitions deppend on data type for result
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // int division gets int

    let _remainder = 43 % 5;
}

// Booleans
fn boolean_dt() {
    // Both work
    let _t = true;
    let _f: bool = false;
}

// Characte, like literal character
fn char_dt() {
    let _c = 'c';
    let _z: char = 'Z';
    let wird_cat = '😻';

    println!("{}", wird_cat) // printing is kind of a pain
}

// Tuples
fn tuple_dt() {
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    // non implicit
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let one = tup.2;

    println!("The value of one is: {one}");
}

// Arrays
fn array_dt() {
    // fixed data type and size
    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // explicit
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // repeat value for full lenght
    let _a = [3; 5];

    let _first = a[0];
    let _second = a[1];
}

fn test_array_dt() {
    let a = [1, 2, 3, 4, 5];

    println!("pls enter an index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index must be a number");

    let element = a[index];

    println!("The searched value {index} is: {element}");
}

fn main() {
    floats_dt();
    operations_dt();
    boolean_dt();
    char_dt();
    tuple_dt();
    array_dt();
    test_array_dt();
}
