/// This module shows some KEY concepts of Rust:
///     lifetimes
/// all these functions are private: they need not be called, they need to typecheck only
///
/// Material for this module:
///     https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

/* ======= Lifetimes =======
   ====================== */
// Structs can store references to data owned by something else, but to do so requires the use of *lifetimes*.
// Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
// Let’s say you try to store a reference in a struct without specifying lifetimes, like this, which **won’t work**:

// uncomment struct and function
struct User<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}
struct User2 {
    username: &'static str,
    email: &'static str,
    sign_in_count: u64,
    active: bool,
}

struct IntUser<'a> {
    username: &'a i32,
    email: &'a i32,
    sign_in_count: u64,
    active: bool,
}

//
pub fn lifetime_test() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    let user2 = User2 {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    // code
//  s   // DNC: error[E0106]: missing lifetime specifier
//
//     // The lifetime of a variable starts at the time when the variable is defined and ends after the last time that the variable is used.
//     //
//     // The borrow checker is the tool that performs lifetime analysis
//     // Look at this code, which defines 2 new nested scopes.
//     // each scope defines a new lifetime  (lifetimes are specified with a preceding '  "tick"
//     // QUIZ: does this code respect lifetimes?
//     // Y/N
    {
        let r: i32 = 0;         // ---------+-- 'a
                                //          |
        {                       //          |
        let mut x : &i32 = &5;  // -+-- 'b  |
        x = &r;                 //  |       |
            // println!("{}",x);
        }                       // -+       |
                                //          |
        println!("r: {}", r);   //          |
    }                           // ---------+
//     // Y/N


    // {
    //     let r;             // ---------+-- 'a
    //                             //          |
    //     {                       //          |
    //         let x = 5;      // -+-- 'b  |
    //         r = &x;             //  |       |
    //     }                       // -+       |
    //                             //          |
    //     println!("r: {}", r);   //          |
    // }                           // ---------+
}




//     // DNC: error[E0597]: `x` does not live long enough
// }

// start 30 / sept
// project out after class:
// lifetimes and generic type parameters

// Lifetime of a reference is not always explicit. For example:
// uncomment this function
// fn longest(x:&str, y:&str) -> &str {
//     if x.len() > y.len() { x } else { y}
// } // this function doesn't work
// DNC: error[E0106]: missing lifetime specifier
// When running this function, Rust doesn't know the lifetime of `x` and `y`.
// The following situation may happen:

// uncomment this function
pub fn uselongest() {
    let x = String::from("hi");

    {
        let z;
        let y = String::from("there");
        z = correct_longest(&x,&y); //will be &y

        println!("z = {}",z);//yikes!
    } //drop y, and thereby z
}
// To fix it, we need to explicitly specify that  `x` and `y` must have
// the same lifetime, and the returned reference shares it.
// This can be done using the apostrophe `'` followed by a lowercase,
// like generic types. By convention, we use `'a`.
//
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
//
// So, the problem of the previous `longest()` definition is
// that that function may return `x` or `y`, so Rust cannot tell
// what's the exact lifetime of the return value.
// To solve this, we need to tell Rust explicitly that `x` and `y`
// need to have the same lifetime,
// or we will return the one with the shorter lifetime.
// Lifetimes on function or method parameters are called *input lifetimes*,
// and lifetimes on return values are called *output lifetimes*.
//
fn correct_longest<'a>(x:&'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// lifetimes are types, so trying to m
fn another_longest<'a,'b>(x:&'a str, y:&'b str) -> &'b str {
    // QUIZ: does this compile?
    // return
        if x.len() > y.len() { x } else { y };
//  error: lifetime may not live long enough
    return y;
}
// Note:
// - Each reference to a value of type `t` has a `lifetime parameter`.
// - `&t` (and `&mut t`) – lifetime is implicit
// - `&'a t` (and `&'a mut t`) – lifetime `'a` is explicit
// - Where do the lifetime names come from?
// - When left implicit, they are generated by the compiler
// - Global variables have lifetime `'static`, which are encoded in the binary of the program.
//

/* ===== Lifetimes FAQ =====
   ====================== */
// - How does the Rust compiler figure out *lifetimes*?
// - The first rule is that each parameter that is a reference gets its lifetime parameter.
//      In other words, a function with one parameter gets one lifetime parameter:
//          `fn foo<'a>(x: &'a i32)`;
//      a function with two parameters gets two separate lifetime parameters:
//          `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`;
//      and so on.
// - The second rule is if there is exactly one input lifetime parameter,
//      that lifetime is assigned to all output lifetime parameters:
//          `fn foo<'a>(x: &'a i32) -> &'a i32`.
// - The third rule is if there are multiple input lifetime parameters,
//      but one of them is `&self` or `&mut self` because this is a method,
//      the lifetime of `self` is assigned to all output lifetime parameters.
//      This third rule makes methods much nicer to read and write because fewer symbols are necessary.
// - When do we use explicit lifetimes?
// - When more than one var/type needs the same lifetime (like the `longest` function)
//
// - How do I tell the compiler exactly which lines of code lifetime `'a` covers?
// - You can't. The compiler will (always) figure it out
//
// - How does lifetime subsumption work?
// - If lifetime `'a` is longer than `'b`, we can use
// `'a` where `'b` is expected; can require this with `'b: 'a`.
// - Permits us to call `longest(&x,&y)` when `x` and `y` have different lifetimes, but one outlives the other.

fn what<'a, 'b, 'c> (x : &'a str, y : &'b str) -> &'c str{

    return "Asd";
}

fn outliving_longest<'b, 'a: 'b>(x:&'a str, y:&'b str) -> &'b str {
    if x.len() > y.len() { x } else { y}
}
// why is this ok?
// because the return type says how the returned value will be used.
// it will be used for some lifetime b, so it's ok to return something that may live longer.

// the converse would not be correct though!


// We can also use lifetimes in data definitions, and we see this later when we define structs, enums, etc.
//
//

/* = Non-Lexical Lifetimes =
   ====================== */
//      http://blog.pnkfx.org/blog/2019/06/26/breaking-news-non-lexical-lifetimes-arrives-for-everyone
// Rust has been updated to support NLL --
//  lifetimes that end before the surrounding scope:
fn nll() {                                           // SCOPE TREE
    let mut names =                         // +- `names` scope start
        ["abe", "beth", "cory", "diane"];           // |
                                                    // |
    let alias = &mut names[0];            // | +- `alias` scope start
                                                    // | |
    *alias = "alex";  // <------------------------ write to `*alias`
                                                    // | |
    println!("{}", names[0]);  // <--------------- read of `names[0]`
                                                    // | |
                                                    // | +- `alias` scope end
                                                    // +- `name` scope end
}
fn nll_example() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    let r3 ;
    println!("{} and {}", r1, r2);
    r3 = &mut s;
    // r1 and r2 are no longer used after this point
    // QUIZ: can i do this ?
    println!("{}", r3);

}

pub fn testintuser(){
    let i = 5;
    let j = 10;

    let user = IntUser {
        email: &i,
        username: &i,
        active: true,
        sign_in_count: 1,
    };
    {
        let k = 20;

        let user2 = IntUser {
            email: &i,
            username: &k,
            active: true,
            sign_in_count: 1,
        };
    }
}



// So how do we use references in struct definition?
// we need lifetime annotations in structs
struct Good_User<'a, 'b> {
    username: &'a str,
    email: &'b str,
    sign_in_count: u64,
    active: bool,
}
fn use_lifetimes() {
    let user1 = Good_User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

// this struct defines a lifetime parameter,
// we can only instantiate it with a str that is already valid
struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // this creates an instance of the ImportantExcerpt struct that holds a reference
    // to the first sentence of the String owned by novel. The data in novel exists before
    // the ImportantExcerpt instance is created. In addition, novel doesn’t go out of
    // scope until after the ImportantExcerpt goes out of scope, so the reference
    // in the ImportantExcerpt instance is valid.
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // in this example,
    // error[E0597]: `another` does not live long enough
    // let mut second : &str = "asd";
    // {
    //     let another = String::from("Call me Ishmael. Some years ago...");
    //     second = another.split('a').next().expect("what");
    // }
    // let ii = ImportantExcerpt{
    //     part : second
    // };

    // uncomment after quizzes in impl below
    // let x = i.announce_and_return_part("asd");
    // println!("{}A", x);
}

impl<'a> ImportantExcerpt<'a> {
    // QUIZ: do i need the lifetime annotation here on &self?
    // fn level(&self) -> i32 {
    //     3
    // }
    // QUIZ: do i need the lifetime annotation here ?
    // fn announce_and_return_part(&self, announcement: &str) -> &str {
    //     println!("Attention please: {}", announcement);
    //     self.part
    // }
}
