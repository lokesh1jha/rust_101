struct Foo {
 x: i32,
}

fn main() {
    // Instantiating a type and binding it to a variable for the lifetime.
    let foo = Foo { x : 42 };

    println!("{}", foo.x);

    //Scope-Based Resource Management
    /*
    Rust uses the end of scope as the place to deconstruct and deallocate a resource.
    The term for this deconstruction and deallocation is called a drop.

    Memory detail:

    1. Rust does not have garbage collection.
    2. This is also called Resource Acquisition Is Initialization ( RAII ) in C++.
    */

    //since we have reached the end of scope
    // NOW: foo will be dropped here !!!!

}