/*
When a struct is dropped, the struct itself is dropped first, then its children are dropped individually, and so on.

Memory Details:

1. By automatically freeing memory Rust helps ensure that there are fewer memory leaks.
2. Memory resources can only be dropped once.
*/ 

struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn main() {
    let foo = Foo { bar: Bar { x: 42}};
    println!("{}", foo.bar.x);

    // Now DROPPING as we reached the end of scope
    // 1st => foo will be dropped
    // 2nd => foo.bar will be dropped
}