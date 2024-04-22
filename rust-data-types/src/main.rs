fn main() {
    // unsigned ints
    let x: u8 = 255; // 0-255
    println!("x is {x}");
    let x: u16 = 65535; // 0-65536
    println!("x is {x}");
    // signed ints
    let x: i8 = -128; // -2^8 to 2^8-1 or -128 to 127
    println!("x is {x}");
    let x: i8 = 127;
    println!("x is {x}");
    // floating
    let x: f32 = -34.32;
    println!("x is {x}");
    // platform specific integers
    let arch_1: usize = 5;
    println!("arch_1 is {arch_1}");
    let arch_1: isize = 5;
    println!("arch_1 is {arch_1}");
    // chars
    let x: char = 'a';
    println!("x is {x}");
    // boolean 
    let x: bool = true;
    println!("x is {x}");
    // type aliasing
    // just like in GoLang
    type Age = u8;
    let jack_age: Age = 23;
    println!("Jack's age is {jack_age}");

    // type conversion
    let a: i64 = 345;
    println!("a is {a}");
    let b = a as f32;
    println!("b is {b}");
    let c = b as i8; // automatically lowered to max value of i8 as 345 is out of bound
    println!("c is {c}");

    // compound data type
    // &str and String
    let fixed_str = "Hi!!";
    println!("fixed len string : {fixed_str}");
    let mut flexible_str = String::from("Hi im flexible string");
    flexible_str.push(':');
    flexible_str.push_str("hahaha");
    println!("{flexible_str}");

    // arrays
    let mut arr:[i32; 5] = [1,2,3,4,5]; // [data_type; size]
    arr[0] = 10; // possible because we declared arr as mutable
    println!("arr is {:?}", arr);
    // creating an array with default value
    let arr = [0; 10];
    println!("arr is {:?}", arr);
    // we cannot expand size of arrays

    // Vectors
    let mut vec: Vec<i64> = vec![1,2,3,4,5];
    println!("vec is {:?}", vec);
    vec.push(10);
    println!("vec is {:?}",vec);

    // Tuples
    let my_info = ("Salary","1M","Age",23);
    let salary_val = my_info.1;
    let (salary, salary_val, age, age_val) = my_info; // destructuring
    println!("my_info is {:?}", my_info);
    println!("{salary} is {salary_val}, {age} is {age_val}");

    let unit:() = (); // zero size, treat it like a Void type (c,c++, java)
    /*
        The () type, also called “unit”.
        The () type has exactly one value (), 
        and is used when there is no other meaningful value that could be returned.
    */
    println!("unit type is {:?}", unit);
    let x = {
        println!("hi");
    };
    // here x is not returning anything so its return type is () or unit
    // learn more here: https://stackoverflow.com/questions/24842271/what-is-the-purpose-of-the-unit-type-in-rust
    
}
/* 
129
127
256
*/