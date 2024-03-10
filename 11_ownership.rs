// Moving Ownership
/* 
struct Foo {
    x: i32,
}

fn do_something(f: Foo){
    println!("{}", f.x)
    //f dropped
}

fn main() {
    let foo = Foo { x: 42};
    // foo is moved to do_someting
    do_something(foo)
    // foo can no longer be used


}
*/

// Returing Ownership
/*
struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // ownership is moved out
}

fn main() {
    let foo = do_something();
    // foo becomes the owner
    // foo is dropped because of end of function scope
}
*/


// Borrowing Ownersip
/*
struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x);
    // f is dropped here
    // foo is dropped here
}
*/



// Borrowing Mutable Ownership with References

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}


fn modify(input_string: &mut String) {
    input_string.push_str(", Sarah Connor")
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // FAILURE: do_something(foo) would fail because
    // foo cannot be moved while mutably borrowed

    // FAILURE: foo.x = 13; would fail here because
    // foo is not modifiable while mutably borrowed

    f.x = 13;
    // f is dropped here because it's no longer used after this point
    
    println!("{}", foo.x);
    
    // this works now because all mutable references were dropped
    foo.x = 7;
    
    // move foo's ownership to a function
    do_something(foo);


    let mut greeting_msg = String::from("Greetings from the future");

    modify(&mut greeting_msg);
    // mutable borrowing allows change in value of the greeting_msg 
    // if that is nomalowner transfer then value could not be changed
    println!("{}", greeting_msg);
    // greeting_msg dropped 


    // let foo = Foo {x: 32};
    // Will throw error in this function as tried to modify value as this is not mutable transfer. So, change not allowed
    // malupulate_foo(foo);

    deferencing();
}
/*
fn malupulate_foo(f: Foo){
    f.x = 12;
    /*
    error[E0594]: cannot assign to `f.x`, as `f` is not declared as mutable
   --> 11_ownership.rs:110:5
        |
    110 |     f.x = 12;
        |     ^^^^^^^^ cannot assign
        |
    help: consider changing this to be mutable
        |
    109 | fn malupulate_foo(mut f: Foo){
        |                   +++

    error: aborting due to 1 previous error
     */
    println!("{}", f.x);

}
 */
fn deferencing() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // get a copy of the owner's value
    *f = 13;      // set the reference's owner's value
    println!("{}", bar);
    println!("{}", foo);
}

/*
Passing Around Borrowed Data

Rust's rules for references might best be summarized by:

Rust only allows there to be one mutable reference or multiple non-mutable references but not both.
A reference must never live longer than its owner.
This doesn't tend to be a problem when passing around references to functions.

Memory details:

The first rule of references prevents data races. What's a data race? A data race when reading from data has the possibility of being out of sync due to the existence of a writer to the data at the same time. This happens often in multi-threaded programming.
The second rule of references prevents the misuse of references that refer to non-existent data (called dangling pointers in C).
*/

/*
struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) {
    f.x += 1;
    // mutable reference f is dropped here
}

fn main() {
    let mut foo = Foo { x: 42 };
    do_something(&mut foo);
    // because all mutable references are dropped within
    // the function do_something, we can create another.
    do_something(&mut foo);
    // foo is dropped here
}

*/