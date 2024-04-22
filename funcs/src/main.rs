fn my_func(){
    println!("From my_func")
}
fn greet_user(uname: &str){
    // a function returns a unit type by default
    println!("Hi {uname}");
}
fn add(a: i64, b: i64)->i128{
    return (a as i128)+(b as i128); // will avoid i64 overflows
}
fn main() {
    println!("Hello, world!");
    my_func();
    greet_user("Buddy");
    let result = add(10,12);
    println!("Add 10+12 = {result}");
    // code blocks
    // they are not reusable
    let full_name = {
        let f_name = "Anurag";
        let l_name = "Verma";
        format!("{f_name} {l_name}") // code blocks can also return, this is actaully a Display trait
    };
    let tmp =  {
        123;
    };
    println!("Full name is {full_name}")
}
