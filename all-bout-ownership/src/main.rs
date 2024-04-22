/*
    ! Only applicable to heap memory space !
    Each value has a variable that its owner
    A value can have one owner at a time
    if the owner gets out of scoped, the value is cleaned up
*/
fn main() {
    let s1 = String::from("Hi"); // s1 is owner of string "Hi"
    let s2 = s1; // ownership is transfered to s2
                 // at this point we cant access s1
                 // in other words, s1 was first pointing to "Hi" in heap
                 // and next line transfers ownership from s1 to s2
                 // In Rust, when you transfer ownership, the original owner (in this case, s1) is no longer valid.
                 // It doesn't point to the heap memory where the string is stored. This avoids double-free errors,
                 // ensuring that there's always a single owner for a given piece of data.
                 // Ownership ensures only one variable has control over a piece of memory at a time
    println!("s2 is {s2}");
    // we can avoid this by creating a deep copy in heap
    let s3 = s2.clone();
    println!("s3 is {s3}");
    // third rule is - out of scoped
    // as soon as scope ends, all the references are dropped (from the heap as well from the stack)
    {
        let s4 = s3.clone();
        println!("s4 is {s4}");
    }
    // println!("s4 is {s4}"); // this line will give error, as s4 is out of scoped
    println!("s3 is {s3}");

    // primitive types
    let x = 32;
    let y = x; // no ownership transfered happened
    println!("y is {x}");

    // ! Ownership in functions !
    let vec_a: Vec<i32> = vec![10, 20, 30, 40];
    let vec_b = vec_a.clone();
    takes_ownership(vec_a);
    // println!("vec_a is {:?}", vec_a);// this line will give error,
    // as the funtion takes_ownership took the ownership of vec_a
    takes_ownership(vec_b.clone());
    println!("vec_b is {:?}", vec_b); // will work
    let vec_c = gives_ownership();
    println!("vec_c is {:?}", vec_c);
    let mut vec_d: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec_e = gives_and_takes_ownership(vec_d);
    // println!("vec_d is {:?}", vec_d);// this will give error as on return ownership of vec_d is transfered to vec_e
    println!("vec_d is {:?}", vec_e);
}
fn takes_ownership(vec: Vec<i32>) {
    println!("vec is {:?}", vec);
}
fn gives_ownership() -> Vec<i32> {
    vec![40, 50, 60]
}
fn gives_and_takes_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(100);
    vec
}
