fn main() {
    let x: i16 = 10;
    println!("x is {x}");
    // x = 12; // cant do it, since we x is immutable
    let mut y: i16 = 10;
    println!("y is {y}");
    y = 21;
    println!("y is {y}");


    // scope
    {
        let z = 30;
    }
    // let s = z; // cannot use z here
    // shadowing
    let t = 10;
    let t = t+10; // this varible 't' is replacing the previous 't'
    println!("t is {t}");
    let t = 13.12; // type can also be changed
    println!("t is {t}"); 

    // scope + shadowing
    let v = 18;
    {
        let v = 19.23; // shadowing prev v, but visible in this scope only
        println!("inner v is {v}")
    }
    println!("v is {v}"); // will print 18

    // constants
    const MAX_VALUE: u32 = 100; // requires explicit type declaration
   println!("MAX_VALUE {MAX_VALUE}");
}
