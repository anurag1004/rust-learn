fn main() {
    // Everything in Rust is an expression, and expressions returns value
    let num = 21;
    if num < 20 {
        println!("num {num} is < 20");
    } else {
        println!("num {num} is >= 20");
    }
    let marks = 68;
    let grade = if marks >= 80 {
        'A' // all return values must have same type
    } else if marks >= 70 && marks < 80 {
        'B' // returning values are without the semicolons
    } else if marks >= 60 && marks < 70 {
        println!("this will not be returneded");
        'C'
    } else {
        'D'
    };
    println!("Grade is {grade}");
    // match , just like switch case
    // in rust whenever we need to define range we can do so by using this syntax
    // start..=end (equal to means we are including end value)
    let grade = match marks {
        80..=100 => 'A',
        70..=79 => 'B',
        60..=69 => 'C',
        50..=58 => 'D',
        _ => 'F',
    };
    println!("Grade is {grade}");
}
