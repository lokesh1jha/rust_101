/*
Escaping Characters
It's challenging to visually represent certain characters, so escape codes allow us to put a symbol in their place.

Rust supports the common escape codes from C-based languages:

\n - newline
\r - carriage return
\t - tab
\\ - backslash
\0 - null
\' - single-quote

Use a \ at the end of a line if you don't want a line break.
println!("hello \
    world") // notice that the spacing before w is ignored


##Raw String Literals
Raw strings allow us to write a sequence of characters verbatim by starting with r#" and ending with "#. It lets us insert characters that might otherwise confuse a normal string as literals (like double quotes and backslashes).

fn main() {
    let a: &'static str = r#"
        <div class="advice">
            Raw strings are useful for some situations.
        </div>
        "#;
    println!("{}", a);
}


If you have some very large text, consider using the macro include_str! to include text from local files in your program:

let hello_html = include_str!("hello.html");



## String Slice
A string slice is a reference to a sequence of bytes in memory that must always be valid utf-8.

A string slice (a sub-slice) of a str slice, must also be valid utf-8.

Common methods of &str:

len gets the length of the string literal in bytes (not number of characters).
starts_with/ends_with for basic testing.
is_empty returns true if zero length.
find returns an Option<usize> of the first position of some text.

fn main() {
    let a = "hi 🦀";
    println!("{}", a.len());
    let first_word = &a[0..2];
    let second_word = &a[3..7];
    // let half_crab = &a[3..5]; FAILS
    // Rust does not accept slices of invalid unicode characters
    println!("{} {}", first_word, second_word);
}

Output: 
7
hi 🦀



Chars
With so much difficulty in working with Unicode, Rust offers a way to retrieve a sequence of utf-8 bytes as a vector of characters of type char.

A char is always 4 bytes long (allowing for efficient lookup of individual characters).

fn main() {
    // collect the characters as a vector of char
    let chars = "hi 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len()); // should be 4
    // since chars are 4 bytes we can convert to u32
    println!("{}", chars[3] as u32);
}


Chars
With so much difficulty in working with Unicode, Rust offers a way to retrieve a sequence of utf-8 bytes as a vector of characters of type char.

A char is always 4 bytes long (allowing for efficient lookup of individual characters).

fn main() {
    // collect the characters as a vector of char
    let chars = "hi 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len()); // should be 4
    // since chars are 4 bytes we can convert to u32
    println!("{} {}", chars[3] as u32);
}

4
129408

*/




/*
String
A String is a struct that owns a sequence of utf-8 bytes in heap memory.

Because its memory is on the heap, it can be extended, modified, etc. in ways string literals cannot.

Common methods:

push_str to add more utf-8 bytes to the end of a string.
replace to replace sequences of utf-8 bytes with others.
to_lowercase/to_uppercase for case changes.
trim for trimming space
When a String is dropped, its heap memory is also dropped.

String has a + operator that extends the string with a &str and returns itself, but it might not be as ergonomic as you hope for.

*/


fn main() {
    let mut helloworld = String::from("hello");
    helloworld.push_str(" world");
    helloworld = helloworld + "!";
    println!("{}", helloworld);

    build_string()
}



/*
Building Strings
concat and join are two simple but powerful ways for building strings.

*/
fn build_string() {
    let helloworld = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}",abc);

    formatting_string()
}


fn formatting_string() {
    let a = 42;
    let f = format!("secret to life: {}",a);
    println!("{}",f);

    parsing_string().unwrap();
}

fn parsing_string() -> Result<(), std::num::ParseIntError> {
    let a = 42;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a, b);
    Ok(())
}
