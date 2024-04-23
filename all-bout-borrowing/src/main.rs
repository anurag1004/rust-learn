/*

! Borrowing !
Establishing a reference to some data
Just like pointers with some rules
Does not take ownership

Why you want to pass a reference to a funtion?
Just like in any other languages to save unncesary overhead of copying
passing an address is quick compare to passing a whole object

Rules of Borrowing (Prevents data races at compile time and dangling references)
- At any time, you can have either one mutable reference or any number of immutable references
- References must always be valid
- You cannot borrow and access a variable as immutable which is also borrowed as mutable or vice versa, at the same time

Solves two problems:
- Data races
- Dangaling references
    - Rust ensures references are not used after their corresponding data is deallocated
*/
fn main() {
    println!("Hello, world!");
    let vec: Vec<i32> = vec![1, 2, 3, 4];
    print_vec(&vec); // passing the reference of this vector
    println!("Inside main -> vec is {:?}", vec);

    // RULES
    let mut vec_a = vec![1, 2, 3, 4];
    let ref_a = &mut vec_a; // a mutable reference
    let ref_b = &mut vec_a; // another mutable reference, thus voilating the rules of borrowing
                            // println!("ref_a is {:?}, ref_b is {:?}", ref_a, ref_b); // will give an error, when multiple references are accessed
                            // The code would not compile if you tried to access both ref_a and ref_b at the same time, because it could lead to undefined behavior.
                            // But it'll compile if we don't use or access both the references at the same time
                            // WHY???
                            // Because it can't detect simultaneous usage of multiple mutable references.
                            // The compiler checks for conflicting usage patterns, so if there's no indication of multiple mutable references
                            // being used at the same time, it might allow compilation. However, accessing multiple mutable references
                            // could still cause undefined behavior, so it is best to avoid it.
                            // multiple immutable references
    let ref_c = &vec_a;
    let ref_d = &vec_a;
    let ref_e = &vec_a;
    println!(
        "ref_c is {:?}, ref_d is {:?}, ref_e is {:?}",
        ref_c, ref_d, ref_e
    );
    // println!("ref_a is {:?}, ref_c is {:?}", ref_a, ref_c); // using mutable and immutable reference borrows at the same time, will give error
    // vec_a was previously borrowed as mutable reference by ref_a and later borrowed as immutable by ref_c
    // however one can access under reference scope of each variable
    let mut vec_b = vec![0, 1, 0, 0, 1, 1, 0, 1];

    let ref_f = &vec_b;
    let ref_g = &vec_b;

    println!("ref_f is {:?}, ref_g is {:?}", ref_f, ref_g); // at this point of time we have multiple immutable references only
    let ref_h = &mut vec_b;
    println!("ref_h is {:?}", ref_h); // this will not give error as at this point of time we have only one mutable

    // dangling references
    let vec_i = {
        let vec_j = vec![9, 0, 0, 1, 1, 0];
        vec_j
    }; // This code snippet transfers ownership of vec_j to vec_i. Even though vec_j goes out of scope, vec_i now owns the vector, keeping it alive.
       // The memory associated with the vector is not deallocated because ownership was transferred.
       // As a result, accessing vec_i is safe.
    println!("vec_i is {:?}", vec_i);
    // however borring doesnt work in the same way
    // let vec_i = {
    //     let vec_j = vec![9, 0, 0, 1, 1, 0];
    //     &vec_j
    // }; // this will give error, as its a dangling reference
    // HOW?
    // This code snippet attempts to return a reference to vec_j.
    // Because vec_j goes out of scope when the block ends, its memory is deallocated.
    // If vec_i holds a reference to vec_j, it's a dangling reference, pointing to invalid or deallocated memory.
    // Rust's compiler catches this issue and prevents the code from compiling, ensuring safety.
    //
    /*
        DEFINATIONS:-
        In Rust, when a variable goes out of scope, its associated memory is deallocated.
        This behavior is governed by Rust's ownership system.
        When you transfer ownership, the data remains valid as long as the new owner is still in scope.
        However, if you only have a reference to data that goes out of scope, you end up with a dangling reference, leading to undefined behavior.
    */
}
fn print_vec(vec: &Vec<i32>) {
    // here we dont want this funtion to take ownership
    println!("Inside print_vec -> vec is {:?}", vec);
}
