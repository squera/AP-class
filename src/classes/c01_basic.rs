/// This module shows some of the basic concepts of Rust:
///     variables,
///     assignment,
///     mutability,
///     base and compound types,
///     expressions and
///     commands
/// Then it shows a tiny bit of the Rust
///     testing infrastructure
// do not care about this line
use std::io;

/// This function shows Rust variables, assignment and mutability
pub fn var_ass_mut(){
    /* ==== Variables, Assignments and Mutability ====
       =============================================== */
    // variables are defined via keyword `let`
    // variables can have their type annotation, or it can be inferred
    let y : i32 = 0;
    let x = y +1 ;
    println!("Values of x and y: {} and {}", x, y);

    // variables can be rebound, though their type **changes**,
    // this is called shadowing
    let x = 'c';
    println!("Value of x: {}", x);
    // println!("Value of x: {}", x+1);
    // DNC: error[E0369]: cannot add `{integer}` to `&str`
    // DNC == Does Not Compile

    // By default, variables are **immutable**, except you specify it.
    let mut y = 0;
    y += 1;
    // DNC: error[E0384]: cannot assign twice to immutable variable `y`
    let z = 1;
    println!("Value of z: {}",z);
    println!("Value of z: {}",z+1);

    // constants **must** declare their types
    const _TRUE : i32 = 1;
    // this constant is local to the function, `FALSE` is shared by all functions in this file
    // the _ underscore before the name tells the rust compiler to not worry about the const usage
}

const _FALSE : i32 = 0;
// QUIZ: can i use const FALSE from `src/main.rs` ?

/// This function showcases Rust base and compound types
/// https://doc.rust-lang.org/book/ch03-02-data-types.html
pub fn vals_types(){
    /* ==== Base Types ====
       ==================== */
    //  - Integers: usize, i32, i64
    //  - Floats: f32, f64
    // usize is the size of a pointer to memory in bytes (4 for 32bits archs, 8 for 64bit archs)
    //  it gives you the guarantee to be always big enough to hold any pointer or any offset in a data structure
    let x : i32 = 10;
    let y : i64 = 20;
    // Rust does not do implicit casting, you have to explicitly cast, with `as X`
    println!("Value of +: {}", x+(y as i32));
    let z : f32 = 1.2;
    let u : f64 = 3.45;
    println!("Value of +: {}", (z as f64)+u);

    // - Booleans
    let t : bool = true;
    let f : bool = false;
    // with the usual boolean destructor: if-then-else
    if t == f {
        println!("True is false");
    } else {
        println!("True is not false");
    }

    // - Characters, which include things like emojis
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Some chars: {}, {}, and {}", c, z, heart_eyed_cat);

    /* ==== Compound Types ====
       ======================== */
    //  - Tuples: fixed-size, each field can have its type, allocated on the stack
    // tuples are destructed via pattern-matching or with dot-notation
    // also with pattern matching, as with most things in Rust,
    // you don't need to spell out the types
    // and you can add an `_` before the name if the variable is not going to be used,
    // to avoid warnings
    // when pattern-matching, you can write _ for any element you will not care about
    // because their types are known statically,
    // RR can suggest what fields you can extract from a struct
    let t = (1,'c',false);
    let tt = t;
    let last = t.2;        // uncomment and see RR's suggestions
    let (a,b,c) = t;
    println!("First element: {} = {} = {}", a,b,c);


    //  - Arrays: fixed length, each field has the same type, allocated on the stack
    // the i32 annotation is the type of each field, the 5 annotation is the length
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // this initialises the array with length 5 to all values being 3
    let _a = [3; 5];
    // this on below is the same as the line above
    let a = [3, 3, 3, 3, 3];
    // The usual array destruction is index accessing via square-bracket notation
    let a1 = a[0];
    let a2 = a[1];
    println!("Array Elements: {} and {}", a1, a2);

    // There are **NO** buffer overflows in Rust (in the worst case it'll panic)
    // the length of arrays is always statically known and it cannot be exceeded
    // uncomment this to see the detailed Rust compiler error
    // let a6 = a[7];

    // The only case in which an array access panics is when the index
    // is not known/cannot be computed statically
    print!("Input element index to lookup:");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(mut i) => {
            println!("Integer input: {}", i);
            // if i>4 { i = 4;  } // comment and input 6
            let _element = a[(i as usize)];
            println!("This will not print without the if");
        }
        ,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    // What are Ok and Err? They're instances of the Result type,
    // which we discuss in few classes


    let mut aa = [(1,2),(1,4)];
    println!("Array1 {:?}",aa);
    aa = [aa[0],(4,5)];
    println!("Array2 {:?}",aa);
    aa[1].0 = 3;
    println!("Array3 {:?}",aa);
    // QUIZ: cosa viene stampato qui?
    // [(1, 2), (1, 4)] || [(1, 2), (4, 5)] || [(1, 2), (3, 5)] || [(3, 2), (4, 5)] || [(1, 3), (4, 5)]
}

/// This function showcases Rust expressions and commands
/// See also:
///     https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
///     https://doc.rust-lang.org/book/ch03-05-control-flow.html
pub fn expressions(){
    // Rust has if-then and if-then-else conditionals
    // Rust has different forms of iteration
    //  - loops
    let mut c = 0;
    loop {
        println!("This will never stop");
        c += 1;
        if c == 4 {
            break; // breaking is the only way to escape an unguarded loop
        }
    }
    //  - while loops
    while c != 0 {
        println!("Cycle with while {}!", c);
        c -= 1;
    }

    //  - for loops, which take iterators as parameters
    //  no i = 0, i< X , i+ notation, the parameter must be iterable, the easiest is using ranges
    for n in 1..51 {   // or 1..=50
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Rust collections can be iterated over.
    // Many datatypes provide .iter or .into_iter methods that allow iteration over the datatype
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Iteration loop: the value is: {}", element);
    }
    // iter - This borrows each element of the collection through each iteration.
    //      Thus leaving the collection untouched and available for reuse after the loop.
    // into_iter - This consumes the collection so that on each iteration the exact data is provided.
    //      Once the collection has been consumed it is no
    //      longer available for reuse as it has been 'moved' within the loop.
    // iter_mut - This mutably borrows each element of the collection,
    //      allowing for the collection to be modified in place.
}

/// This module is used to show Rust's testing infrastructure
// Rust modules can be nested,
// this is a private testing module, as the next line defines
#[cfg(test)]
mod testing {
    use super::testfuns::{crapadd, okadd};

    // all functions marked as #[test] can be run with project testing
    #[test]
    // Ensure you can see the Cargo panel in RR:
    //      View -> Tool Windows -> Cargo --> drag it where you want
    //      Click the 'run cargo command' --> type "cargo test" to run all the tests and only them
    //      see the change in the Run icon on the top-right icons menu
    fn test_crapadd() {
        assert_eq!(crapadd(1,3),2);
    }
    #[test]
    fn test_okadd(){
        assert_eq!(okadd(1, 5), 6);
    }
}
/// This is an example public module used by the testing module above
// this is a public, inner module
pub mod testfuns{
    // the body of this function contains a statement, which needs a ';'
    pub fn crapadd(x: i32,_y: i32) -> i32 {
        return x+x;
    }
    // the body of this function contains an expression,
    // and expressions return the value they compute, so we don't need a return
    pub fn okadd(x: i32, y:i32) -> i32 {
        x+y
    }
}

