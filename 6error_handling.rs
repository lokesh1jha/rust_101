/*
Rust has a built in generic enum called Result that allows us to return a value that has the possibility of failing. It is the idiomatic way in which the language does error handling.

enum Result<T, E> {
    Ok(T),
    Err(E),
}
Note that our generics type has multiple parameterized types separated by a comma.

This enum is so common, instances of the enum can be created anywhere with the enum variants Ok and Err.
*/

fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))   
    }
}

// fn main() {
//     let result = do_something_that_might_fail(12);

//     // match lets us deconstruct Result elegantly and ensure we handle all cases!
//     match result {
//         Ok(v) => println!("found {}", v),
//         Err(e) => println!("Error: {}",e),
//     }
// }

// Main returns no value, but could return an error!
// fn main() -> Result<(), String> {
//     let result = do_something_that_might_fail(12);

//     match result {
//         Ok(v) => println!("found {}", v),
//         Err(_e) => {
//             // handle this error gracefully
            
//             // return a new error from main that said what happened!
//             return Err(String::from("something went wrong in main!"));
//         }
//     }

//     // Notice we use a unit value inside a Result Ok
//     // to represent everything is fine
//     Ok(())
// }



fn main() -> Result<(), String> {
    // Look at how much code we saved!
    let v = do_something_that_might_fail(42)?;
    println!("found {}", v);
    Ok(())
}
