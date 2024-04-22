fn main() {
    let mut i = 0;
    const MAX:i32 = 4;
    // loops
    loop {
        println!("Hi!!");
        i+=1;
        if i > MAX {
            break;
        }
    }
    // labeling loops
    'outer: loop{
        println!("Outer");
        let mut j = 0;
        'inner: loop{
            println!("inner j:{j}");
            if j > MAX {
                break 'outer;
            }
            j+=1;
        }
    }
    // for loop
    let vec = vec![1,2,3,4,5];
    for i in vec {
        println!("{i}");
    }
    i = 0;
    while i < 3 {
        println!("i: {i}");
        i+=1;
    }
    // loops returns value as well
    let val = loop {
        // In Rust, break can be used with a value to terminate the loop and return that value.
        break 5; // So, break 5; terminates the loop and returns the value 5.
    };
    println!("val is {val}");

    // printing positional arguments
    println!(
        "My name is {0} {1}",
        "Anurag", "Verma"
    );
    // named args
    println!("Language = {lang}, Year = {year}", lang="Rust", year=2024);

    // input/output
    let mut input = String::new();
    println!("Input: ");
    std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to get the input!");
    println!("You entered: {input}");

    // Result(is an Enum) has two variants - Ok and Error
    // Ok -> operation success (contains success value)
    // Error -> operation failed (contains Error value)
    // more on this later... 🐍

    // let's convert the input into the appropriate type..
    // whatever type we want the input to be parsed, we can mention it in the variable declaration
    let input_num: i32 = input.trim().parse().expect("Invalid input type.\nExpected: i32");
    println!("{input_num}");

    // to fix the warning of "unused variable/expression" one can add underscore at the start
    let _unused_vaiable = 23.12;
    // to increase readability of huge numbers we can add underscore in between
    let huge_num = 100_000;
    println!("huge_num is {huge_num}");

    static WELCOME: &str = "Welcome to Rust!";
    let welcome_msg = WELCOME;
    println!("{welcome_msg}");
}
