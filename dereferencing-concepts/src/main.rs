fn main() {
    // dereferencing
    // Its a process of accessing the value pointed by the reference
    // ref -> 0x210
    // 0x210 holds 20
    // dereferencing ref will give 20
    let mut data: i8 = 127;
    let ref_data = &mut data;
    let copy_data = *ref_data;
    println!("copy data is {copy_data}");
    *ref_data = -128; // since ref_data is a ref to original data, value of data will be changed
    println!("data is {data}");

    /*
        ! NOTE !
        Although references resides on stack, it cannot be copied but can only be moved.
        If we allow copying of references then we'll be voilating the rust borrowing rule;
        References can only be moved but not copied
    */
    let mut vec_a = vec![1, 2, 3, 4];
    let ref_a = &mut vec_a;
    // let deref_copy = *ref_a;
    /*
        The above line means, we are first dereferencing to get original vector(which is heap allocated)
        Assigining heap allocated variable to other variable means -> taking ownership
        so deref_copy is trying to take the ownership of vector owned by vec_a
        Which violets rust borrowing rules
            -> there cannot be multiple mutable references
            -> even if deref_copy takes the ownership, vec_a will no longer have the ownership to the vector
             making ref_a an invalid reference.

    */
    let deref_copy = ref_a.clone();
    /*
        ! NOTE !
        Mutable references when assigned are moved
        while Immutable references when assigned are copied
    */
    let vec_b = vec![7, 8, 9];
    let ref_b_1 = &vec_b;
    let ref_b_2 = &vec_b;
    let ref_b_3 = &vec_b;
    println!(
        "vec_b is {:?}, ref_b_1 is {:?}, ref_b_2 is {:?}, ref_b_3 is {:?}",
        vec_b, ref_b_1, ref_b_2, ref_b_3
    );

    let s1 = String::from("hello");
    let s2 = s1;
    // view(s1); since s1 is moved (s2 has the ownership of whatever content s1 was originally pointing to), the rustcompiler will say : "Use of moved value"
    // view_as_ref(&s1);
    /*
        This view fn takes string ref as its argument meaning borrows string value
        Since the ownership of the "hello" string is changed (s2 owns the value), s1 is invalid reference
        So rustcompiler will say: borrow of moved value 's1'
    */
    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s2;
    let s4 = &s3;
    let is_equal = ***s4 == "Hello".to_string();
    println!("is_equal: {is_equal}");
}
fn view(s: String) {
    println!("string is {s}");
}
fn view_as_ref(s: &String) {
    println!("string is {s}");
}
