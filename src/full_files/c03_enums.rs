/// This module shows some KEY concepts of Rust:
///     enums
///     Option
///     pattern-matching
///     Result & error handling
///     error handling in Vec

/// Material for this module:
///
///     https://doc.rust-lang.org/book/ch06-00-enums.html
///     https://doc.rust-lang.org/std/option/enum.Option.html
///     https://doc.rust-lang.org/book/ch18-00-patterns.html?highlight=pattern%20ma#patterns-and-matching
///     https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

// enums define a type that has multiple possible variants.
// Enums are a feature in many languages, but their capabilities differ in each language.
// Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.

// the `IpAddrKind` enum defines 2 different values: `V4` and `V6`
pub enum IpAddrKind {
    V4,
    V6,
}
// the `IpAddr` enum defines 3 values:
//   V4 has 4 i32 fields, V6 has a String field and V0 has none
enum IpAddr {
    V4(i32,i32,i32,i32),
    V6(String),
    V0(),
}
// the first enum is accessible from other modules, and both `V4` and `V6` are public types


/// This function showcases Rust Enums and how to use them
pub fn enum_usage(){
    // we can create values of each type declared in the enum
    let _four = IpAddrKind::V4;
    // here we say that `_four` is a value of a certain variant (V4)
    let _six = IpAddrKind::V6;

    let loopback = IpAddr::V6(String::from("::1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    // we can access these fields with pattern-matching, which we describe in a second
}


/* ==== Option Types ====
   ====================== */
// https://doc.rust-lang.org/std/option/enum.Option.html
// The Option type is used in many places
// it encodes the very common scenario in which a value could be something or it could be nothing.
// Benefit:
//      the Rust type system lets us express whether a value can be something or nothing
//      so the compiler can check whether you’ve handled all the cases you should be handling;
// Design choice:
//      Rust doesn’t have the null feature that many other languages have.
//      In languages with null, variables can always be in one of two states: null or not-null.
//      but if you try to use a null value as a not-null value, you’ll get an error of some kind.
//      This leads to many mistakes
// Rust does not have nulls, but it does have the enum `Option<T>`,
// which lets one express the same idea of null values, but with type system guarantees
// The `<T>` syntax is a feature of Rust we haven’t talked about yet called generic type parameter,
// we'll talk about them later
// Option<T> is defined by the standard library as follows,
/*
enum Option<T> {
    None,
    Some(T),
}
 */

/// This function showcases Rust Options and how to use them
// let's look at Option usages
pub fn option(){
    // here we instantiate the type parameter T with i8
    // it is kind of like calling functions: T is a formal parameter and its actual parameter here is i8
    // Note that `Option<T>` is not the same type as `T`.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // QUIZ: can i do this:
    let sum = x + y.unwrap();
    // Y / N



    //
    // DNC: error[E0277]: cannot add `Option<i8>` to `i8`
    // Option<i8> is like String, Vec, Bool, it is effectively another type,
    // look where it is placed in the syntax, right after the " : "

    // // options have a number of specific destructors and error handling methods
    let nopt : Option<i32> = None;
    let opt = Some(10);
    // if nopt.is_nons
    // QUIZ: what will this expression do?
    // let v = nopt.unwrap();

    //
    // RTE: thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'
    // // RTE = runtime error
    // let v = opt.unwrap();
    // println!("Some of {}",v);
}

/// This function showcases Pattern matching in Rust
pub fn patternmatching(){
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // match allows you to compare some value against a series of pattens
    // and execute code based on which pattern matches.
    // Patterns can be made up of literal values, variable names, wildcards, and many other things;
    //      Chapter 18 covers all the different kinds of patterns and what they do.
    // The power of match comes from the expressiveness of the patterns
    // and the fact that the compiler confirms that all possible cases are handled.

    // QUIZ: is this ok?
    // match home {
    //     IpAddr::V4(a, b, c, d) => println!("Is V4"),
    //     IpAddr::V6(a) => println!("Is V6")
    // };
    // Y / N


    //
    // DNC: error[E0004]: non-exhaustive patterns: `V0` not covered
    match home {
        // matches any V4, must be after the previous or that becomes unreachable
        IpAddr::V4(a, b, c, d) => println!("Is V4"),
        // matches any V4 whose first field is 127
        IpAddr::V4(127, b, c, d) => println!("Is V4 loopback"),
        // matches any V6
        IpAddr::V6(a) => println!("Is V6"),
        // the " _ " matches anything
        _ => println!(" catch all")
    };
    // pattern-matching can return values, so it can be used to set variables
    let _variable = match loopback {
        IpAddr::V4(127, b, c, d) => Some(loopback),
        _ => None
    };
    let firstfield = match IpAddr::V4(10,20,30,40){
        IpAddr::V4(a,_,_,_) => a,
        _ => 0,
    };
    println!("The first field is: {}", firstfield);

    // since pattern-matching works on Enums, it works on options too, even when combined,
    // like in a tuple: here the patters to test are 4
    let nopt : Option<i32> = None;
    let opt : Option<i32> = Some(3);
    let test_eq = match (opt, nopt) {
        (Some(o),Some(n)) => {o == n},
        // unused arguments can be made irrelevant with " _ "
        (Some(_),None) => {false},
        (None,Some(_)) => {false},
        (None, None) => {false},
    };
    println!("Are they the same? {}", test_eq);

    // For specific Enums, like Option, pattern-matching is not the only way
    // to use values of their type:
    // we can check if an Option is Some or None, unwrap its content, and
    let issome = nopt.is_some();
    let isnone = opt.is_none();
    // unwrap gets out the content of a Some
    if isnone {
        let content = opt.unwrap();
    }
    // careful, using `unwrap` can panic when called on Nones
    let exp = nopt.expect("insert error message here");
    println!("nopt {}",exp);
    // `expect` is like unwrap but with a specific message

    // there are many more ways to use an Option, check out
    // .and_then
    // .is_some_and
    // .ok_or_else
    // .zip
}

/// This function showcases Rust errors
pub fn errors() {
    // Rust groups errors into two major categories:
    //      recoverable and
    //      unrecoverable errors.
    // recoverable error: e.g., a file not found error, it’s reasonable to report the problem to the user and retry the operation.
    // Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.
    // For recoverable errors, Rust doesn’t have exceptions like other languages.
    //      Instead, it has the type `Result<T, E>`
    //      https://doc.rust-lang.org/std/result/enum.Result.html
    // For unrecoverable errors, Rust has the panic! macro that stops execution.
    // Result enum is defined as having two variants, Ok and Err, as follows:
    /*
    ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
     */
    use std::fs::File;

    // erase file and comment line to see panic
    File::create("hello.txt");

    let f = File::open("hello.txt");
    // as always, `match` needs to be exhaustive
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // We can also nest our matching expressions to match with a specific kind of error.
    use std::io::ErrorKind;
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    // The type of the value that File::open returns inside the Err variant is io::Error,
    // which is a struct provided by the standard library.
    // This struct has a method kind that we can call to get an io::ErrorKind value.
    // The enum io::ErrorKind is provided by the standard library and has variants
    // representing the different kinds of errors that might result from an io operation.
    // The variant we want to use is ErrorKind::NotFound,
    // which indicates the file we’re trying to open doesn’t exist yet.
    // So we match on f, but we also have an inner match on error.kind().

    // Using match can sometimes be a bit verbose, so we can use the `unwrap()` or `expect()` methods.
    // let f = File::open("hello.txt").unwrap();
    // If we run this code and the file doesn't exist, it will panic with the following error.
    //      thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/full_files/c03_enums.rs:179:23
    // We can also use `expect()`, to print a specific message when our code runs.
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // We briefly above saw the `panic!` macro.
    // This macro is used to generate an unrecoverable error:
    // panic!("crash and burn");

    //How do you know when to call `panic!` and when to return a `Result`?
    // Result:
    //      you give the calling code options rather than making the decision for it.
    //      The calling code could choose to attempt to recover in a way that’s
    //      appropriate for its situation, or it could decide that an
    //      Err value in this case is unrecoverable, so it can call panic!
    //      and turn your recoverable error into an unrecoverable one.
    //      Therefore, returning Result is a good default choice when you’re defining a function that might fail.
    // Panic:
    //      you deny users of your code the option to recover
}

/// This function showcases errors in Rust collections (Vec)
pub fn collectionerrors(){
    // many Rust collections make extensive usage of Options and Results
    let num = vec![10, 20];

    println!("num[0]: {}", num[0]);
    println!("num[1]: {}", num[1]);
    // here we are accessing vector elements using .get
    //      the difference with [.] is that .get returns an Option
    //
    println!("num[2]: {}", num.get(0).unwrap());
    println!("num[3]: {}", num.get(1).unwrap());

    // so we can use pattern matching or any learnt trick on .get
    match num.get(2) {
        Some(n) => {
            println!("Found a value at index");
        },
        None =>{
            println!("Found no value at index");
        }
    }
    // or alternatively
    if num.get(2).is_some(){
        println!("Found a value at index");
    }else{
        println!("Found no value at index");
    }

    // let temp = num[2];
    // This causes an error:
    //      thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 2', src/full_files/c03_enums.rs:259:16
}




// ? is an error propagation expression.
// as such it only propagates the error part of Options or Results: None / Err
// the Some / Ok part is unwrapped correctly
fn qm() -> Option<i32> {
    // look at the type of the return: it's an option
    // same type of retn
    let r = retn()?;
    // look at the type of `r`: it's already an i32!
    // what happens when i remove the `?` ?
    return Some(4);
    // how can this return statement be reached ?
}

pub fn testqm() {
    let r = qm();
    println!("Received {:?}",r);
    let r = retop();
    println!("Received {:?}",r);
}


fn retop() -> Option<i32>{
    return Some(3);
}
fn retn() -> Option<i32> {
    return None;
}


//
use std::fs::File;
use std::io::{Read, Write};
use std::io::prelude::*;

pub fn readfilecontent () -> Result<(),String>{

    // create a new file X -> deal with the Result
    // do not unwrap immediately -> issues when unwrapping later
    // write to it using write_all and a 'b' buffer -> deal with the Result
    // read its content
    // feed to calculateS, with String parameter (not &String) -> Ownership!
    // printout the content and the s's

    let mut file = File::create("foo.txt").unwrap();
    file.write_all(b"ssSSSss");
    let mut file = File::open("foo.txt").unwrap();
    let mut contents = String::new();
    let r = file.read_to_string(&mut contents);
    if r.is_err(){
        return Err(String::from("asd"));
    }
    let tots = calculateS(&contents);

    println!("Contents: {}, with {}s",contents, tots);
    return Ok(());
}
// write out calculateS
// use chars iterator
// use eq_ignore_ascii_case
fn calculateS(s : &String) -> i32{
    let mut tot = 0;
    for c in s.chars(){
        if c.eq_ignore_ascii_case(&'s') {
            tot +=1;
        }
    }
    return tot;
}