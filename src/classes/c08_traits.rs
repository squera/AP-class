/// This module shows some KEY concepts of Rust:
///     generics
///     traits (definition, where clauses, impl, dyn)
///     polymorphism

/* ======= Generics ========
   ====================== */
/* We have seen generics already, they are abstract stand-ins
for concrete types that reduce code duplication,
so instead of having to write
    Option<i32>
        Some(i32)
    Option<usize>
        Some(usize)
one writes
    Option <T>
        Some(T)

Generics have long been studied in logics and formal languages, the
    T
of a generic declaration is called
    Type Parameter
much like in a function there are parameters
    pub fn add(x, y)
in a type declaration (be it a struct / enum / function singnature), there are
    Type Parameters
for example:
*/
// this is a struct with generic fields
struct Point<T, U> {
    x: T,
    y: U,
}
// impl Point<T,U>{
//     // pub fn add(e : T) -> T {
//     //     if T == i32 {
//     //         ...
//     //     }
//     //     if T == String{
//     //         ...
//     //     }
//     // }
// }
// and its impl to get the first
// Note that we have to declare `<T,U>` just after `impl`
// so we can use it to specify that we’re implementing methods on the type `Point<T,U>`.
// By doing so, Rust can identify that the type in the angle brackets
// in `Point` is a generic type rather than a concrete type.
impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }
}
// this instead is an impl that only exists for points i32, i32.
impl Point<i32,i32>{
    fn xx(&self) -> i32 {
        0
    }
}
impl Point<f32,f32>{
    fn xx(&self) -> i32 {
        0
    }
}
impl<T> Point<i32,T>{
    fn xxx(&self) -> i32 {
        0
    }
}
pub fn struct_generic(){
    let both_integer : Point<i32,i32> = Point { x: 5, y: 10 };
    let both_float : Point<f32,f32> = Point { x: 1.0, y: 4.0 };
    let integer_and_float : Point<i32,f32> = Point { x: 5, y: 4.0 };
    let val1:i32 = *both_integer.x();
    let val2:i32 = integer_and_float.xxx();
    // both_integer.xxx(); // QUIZ: can i call this:
    // let val3 = both_float.xx();

    println!("val1 and val2: {} and {}", val1, val2);
}
/*
Much like when calling a function we supply arguments for the parameter
    add(1,2)
we need to supply type arguments to stuff with type parameters.
Many languages do this implicitly for us,
in Rust this happens implicitly most times, but we have syntax to do this explicitly too
 */
pub fn explicit_type(){
    let b = String::new();
    let trimmed = b.trim();
    match trimmed.parse::<u32>() {
        _ => ()
    }
}

/*
Monomorphization of Generics:
Monomorphization is the process of turning generic code into specific code
by filling in the concrete types that are used when compiled.
This, in turn, makes generic code not slow

// This function
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}

// Gets compiled by the rust compiler into
enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}
fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
 */

/* ======== Traits =========
   ========================= */
// A *trait* tells the Rust compiler about functionality
// a particular **type** has and can share with other types.
// - traits define shared behavior in an abstract way.
// - bounds specify that a generic type can be any type that has certain behavior.
// for example:

// kind of like interfaces, but not really. Closer to Haskell ADT
// Kinda existential types: "existentially-quantified type”, is a type that intuitively represents “any type satisfying a given property"
//  https://varkor.github.io/blog/2018/07/03/existential-types-in-rust.html
// the universal quantifier ∀ is scoped over the entire type — everything after the ∀ —
// whereas the existential quantifier ∃ is scoped over just the immediate trait bound

// Imagine we are working on many many different type of numbers,
// and we want to implement a `the_large_one()` function
// that can return the largest value of two input values.
// Without generic, we need to define this function for every single data type:

fn the_large_one_i8(x: i8, y:i8) -> i8 {if x > y {x} else {y}}
fn the_large_one_i16(x: i16, y:i16) -> i16 {if x > y {x} else {y}}

// As a smart coder, we cannot let it happen.
// So, we define a generic type `T`, and tell rust to do the rest for us.

// Q: why should this code not compile?
// fn the_large_one_gen<T>(x: T, y: T) -> T {if x > y {x} else {y}}

//
// However, rust will complain because not every data type can be compared.
// DNC: error[E0369]: binary operation `>` cannot be applied to type `T`
// Traits are a way to address this (and other) issues,
// for example, a Trait can tell that > can be applied to all types extending that Trait
// this is called (PartialOrd).
fn the_large_one_gen_correct<T:PartialOrd>(x: T, y: T) -> T {
    if x > y {x} else {y}
}

// This is the same difference between existential and universal types
//

// Let's now see
//      defining a Trait
//      implementing a Trait on a Type

// A type’s behavior consists of the methods we can call on that type.
// Different types share the same behavior if we can call the same methods on all of those types.
// *Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.*

// For example, let’s say we have multiple structs that hold various kinds and amounts of text:
// a `NewsArticle` struct that holds a news story filed in a particular location and
// a `Tweet` that can have at most 280 characters along with metadata that indicates whether it was a new tweet, a retweet, or a reply to another tweet.

// We want to make a media aggregator library that can display summaries o
// f data that might be stored in a `NewsArticle` or `Tweet` instance.
// To do this, we need a summary from each type, and we need to request that summary by calling a `summarize` method on an instance.

// Here, we declare a `trait` using the `trait` keyword and then the trait’s name,
// which is `Summary` in this case.
pub trait Summary {
    //Inside the curly brackets, we declare the method signatures
    // that describe the behaviors of the types that implement this trait,
    // which in this case is `fn summarize(&self) -> String`.
    fn summarize(&self) -> String;
    //We don't give the function body to `summarize`,
    // because we want the `struct`s that implement this trait
    // to implement their own `summarize` function.

    // However, we give a default implementation to function `say_hello()`,
    // so that the concrete type can choose to use this default implementation,
    // or implement their own.
    fn say_hello() where Self: Sized {
        println!("Hello")
    }
}

// If we want to define the desired behavior of a struct or any type of a trait, we need to implement the trait for the type.

// here's the NewsArticle Struct
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// And here is the implementation of the Summary Trait for NewsArticle
// it uses the same keword we've seen for adding behaviour to a struct
// in this case we are specifying what behaviour to add precisely: the behaviour dictated by the Trait
// QUIZ: Will this compile:
// impl Summary for NewsArticle {
//
// }

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
// If you want to call the `Summary` trait from other modules, you need to use `use` keyword as you do for your struct or module.
// You will also need to specify that `Summary` is a public trait before calling from other modules by saying `pub trait Summary ...`.

// the Tweet Struct
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// and its implementation of the Summary Trait for it
// You have the choice to provide a default implementation for the desired behavior, like what we did for the `say_hello()` function.
// Suppose we also give `summarize()` a default implementation,
// then you just need to say `impl Summary for NewsArticle {}` if you want `NewsArticle` to use the default implementation.
// Of course you can replace the default implementation using your custom implementation.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn say_hello() {
        println!("Yello!")
    }
}

pub fn traitexample(){
    let t = Tweet{
        username: "Marco".to_string(),
        content: "no way jose".to_string(),
        reply: false,
        retweet: false
    };
    let n = NewsArticle{
        headline: "Wasps!".to_string(),
        location: "Povo".to_string(),
        author: "Patrignani".to_string(),
        content: "there is a wasp in my attic".to_string()
    };
    t.summarize();
    n.summarize();

    // QUIZ: what do these print?
    // hello / hello | hello / yello | yello / hello | yello / yello |
    NewsArticle::say_hello();
    Tweet::say_hello();
}

// Trait parameters
// We can use traits in function definition as the parameter type
// to tell rust this function can take multiple type.
// For example, if we want to write a function that takes a struct that implement the `Summary` trait as the input, we can say

fn notify_fn(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// the `impl Trait` syntax is actually syntax sugar for a long form,
// which is called a trait bound.
// Trait bound is a way to set a bound for the types that can be used in functions or methods.

fn notify_bound<T: Summary>(item: &T) {
    println!("More Breaking news! {}", item.summarize());
}

use std::cmp::Ordering;
// let's import 2 commonly used traits first
use std::fmt::{Debug, Display, Formatter};

// The trait bound syntax can express more complexity in other cases.
// For example, we can have two parameters that implement `Summary`.
// Using the `impl Trait` syntax looks like this, where
// we can also specify more than one trait bound using the `+` syntax.

fn notify_fn2(item1: &(impl Summary + Display), item2: &(impl Summary + Display)) {}
// something that 'impl'ements Summary and Display
fn notify_bound2<T: Summary + Display>(item1: &T, item2: &T) {}
// a generic type T that implements traits Summary and Display

pub fn example_notify(){
    let t = Tweet{
        username: "Marco".to_string(),
        content: "no way jose".to_string(),
        reply: false,
        retweet: false
    };
    notify_fn(&t);
    notify_bound(&t);
    // QUIZ: why will this not compile?
    notify_fn2(&t, &t);
    notify_bound2(&t, &t);

    let n = NewsArticle{
        headline: "Wasps!".to_string(),
        location: "Povo".to_string(),
        author: "Patrignani".to_string(),
        content: "there is a wasp in my attic".to_string()
    };
    notify_fn2(&t, &n);
    // notify_bound2(&t, &n);
    //
    // DNC: error[E0277]: `Tweet` doesn't implement `std::fmt::Display`
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("");
        Ok(())
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("");
        Ok(())
    }
}

// If you have really fancy trait bounds for your types,
// your function signature will be very very long. T
// o make our life easier, rust defines a **`where` clause** in which you can put all your trait bounds inside.

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}
fn some_function_where<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug{
    0
}

// What if we want to return something that implements a certain trait?
// uncomment the code, and see the errors
// fn ret_trait_wrong() -> dyn Summary {
//     let t = Tweet{
//         username: "Marco".to_string(),
//         content: "no way jose".to_string(),
//         reply: false,
//         retweet: false
//     };
//     let n = NewsArticle{
//         headline: "Wasps!".to_string(),
//         location: "Povo".to_string(),
//         author: "Patrignani".to_string(),
//         content: "there is a wasp in my attic".to_string()
//     };
//     if rand::random() < 0.5 {
//         t
//     } else {
//         n
//     }
// }

// start here

// DNC: error[E0782]: trait objects must include the `dyn` keyword
// -> add `dyn` :  https://doc.rust-lang.org/std/keyword.dyn.html
// DNC: error[E0038]: the trait `Summary` cannot be made into an object
// the error suggests to read this
//          https://doc.rust-lang.org/reference/items/traits.html#object-safety
//      read it!

// If you want to use `impl Trait` syntax in the return position
// to return a value of some type that implements a trait,
// the function must have a fixed return type
// or ...
// we can do some tricks to achieve this: we need a Trait Object
// Why do we need a Trait Object?
    // The point is that the Rust compiler *needs to know
    // how much space every function's return type requires*.
    // This means all your functions have to return a concrete type.
// What is a Trait Object?
// see later
// So if we want to use a custom trait as the return type of your function,
// you need to use some trick.

// Trait objects:
//      https://doc.rust-lang.org/reference/types/trait-object.html
// A trait object is an opaque value of some type that implements a set of traits.
// The set of traits is made up of an object-safe base trait plus any number of auto traits.
//
// Trait objects implement the base trait, its auto traits, and any supertraits of the base trait.
//
// Trait objects are written as the keyword
//      `dyn`
// followed by a set of trait bounds, but with the following restrictions on the trait bounds.
// All traits except the first trait must be auto traits,
// there may not be more than one lifetime,
// and opt-out bounds (e.g. ?Sized) are not allowed.
// Furthermore, paths to traits may be parenthesized.


// Due to the opaqueness of which concrete type the value is of,
// trait objects are dynamically sized types.
// Trait objects are used behind some type of pointer;
// for example &dyn SomeTrait or
//          Box<dyn SomeTrait>
// Each instance of a pointer to a trait object includes:
// a pointer to an instance of a type T that implements SomeTrait
// a virtual method table, often just called a vtable,
//  which contains, for each method of SomeTrait and its supertraits that T implements,
//  a pointer to T's implementation (i.e. a function pointer).

// The purpose of trait objects is to permit "late binding" of methods.
// Calling a method on a trait object results in virtual dispatch at runtime:
// that is, a function pointer is loaded from the trait object vtable and invoked indirectly.
// The actual implementation for each vtable entry can vary on an object-by-object basis.
// We'll see more of this when we talk about OO features in Rust

// Notice: we had monomorphization of Generics
// and now we have dynamic dispatch for trait objects
//      compare with Java: always dynamically dispatched
/// Also see
///     https://oswalt.dev/2021/06/polymorphism-in-rust/
/// in order to see the memory layout of trait objects
///
/// Generics: larger binaries, faster code
/// Trait objects: smaller binaries, indirection
/// Both: code reuse

// Below we use `Box<dyn Trait>`, a trait object as the return type to solve this problem.
// For now, just know that
//      Box<T>
// is a type of heap-allocated stuff of type T

// see the change in the return type
fn ret_trait() -> Box<dyn Summary> {
    let t = Tweet{
        username: "Marco".to_string(),
        content: "no way jose".to_string(),
        reply: false,
        retweet: false
    };
    let n = NewsArticle{
        headline: "Wasps!".to_string(),
        location: "Povo".to_string(),
        author: "Patrignani".to_string(),
        content: "there is a wasp in my attic".to_string()
    };
    if 0.1 < 0.5 {
        // and now we return stuff allocated in a Box
        Box::new(t)
    } else {
        Box::new(n)
    }
}

// Let's see another example of the usage of Trait Objects

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // uncomment function to see error
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}
// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}
// Returns some struct that implements Animal,
// but we don't know which one at compile time.
// QUIZ: What's the return type?
//
// uncomment all related code
fn random_animal(random_number: f64) ->
//
                                     Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

pub fn animals_example() {
    let random_number = rand::random();
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

/* ==== Conditional Trait Implementation ======
   ====================== */
// We can implement methods conditionally for types that implement a specific trait.
struct Pair<T> {
    x: T,
    y: T,
}

// this function exists for all pairs
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    // this method too
    fn print(&self) {
        println!("Pair")
    }
}
// this function only exists for those pairs of objects that implement both Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {

    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
// compare and contrast: what can we do with something of type T in the first
// impl, and what can we do in the second one with the same somethings?


/* ==== Deriving Traits ====
   ====================== */
// The smart compiler provides basic implementations for some traits via the
//      `#[derive]` [attribute]
//      (https://doc.rust-lang.org/rust-by-example/attribute.html).
// We can plug and use those traits without thinking about the implementation.
//
// *These traits can still be manually implemented if a more complex behavior is required.*

// * Comparison traits: Eq, PartialEq, Ord, PartialOrd.
// * Clone, to create `T` from `&T` via a copy.
// * Copy, to give a type 'copy semantics' instead of 'move semantics'.
// * Hash, to compute a hash from `&T`.
// * Default, to create an empty instance of a data type.
// * Debug, to format a value using the `{:?}` formatter.

// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);
// `Inches`, a tuple struct that can be printed
#[derive(Debug,PartialEq)]
struct Inches(i32);
// we are not implementing any trait here, we're just adding code to Inches
impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}
// `Seconds`, a tuple struct with no additional attributes
struct Seconds(i32);

fn example_derivable() {
    let _one_second = Seconds(1);
    // QUIZ: why do this not compile?
    // println!("One second looks like: {:?}", _one_second);
    // QUIZ: why do this not compile?
    // let _this_is_true = (_one_second == _one_second);

    // DNC: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    // DNC: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
// impl PartialOrd for Point<T,U>{
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         todo!()
//     }
// }

/* === Self (capital S) ====
   ====================== */
//      https://doc.rust-lang.org/reference/paths.html#self-1
// sometimes you may see Self amongst types
// Self, with a capital "S", is used to refer to the implementing
// type within traits and implementations.
// Self can only be used as the first segment, without a preceding ::.

trait T {
    // traits can also define types that their
    //  implementations also need to 'refine'
    type Item;
    const C: i32;
    // `Self` will be whatever type that implements `T`.
    fn new() -> Self;
    // `Self::Item` will be the type alias in the implementation.
    fn f(&self) -> Self::Item;
}
struct ST;
impl T for ST {
    type Item = i32;
    const C: i32 = 9;
    fn new() -> Self {           // `Self` is the type `ST`.
        ST
    }
    fn f(&self) -> Self::Item {  // `Self::Item` is the type `i32`.
        Self::C                  // `Self::C` is the constant value `9`.
    }
}
pub fn test () {
    let st = ST::new();
    let n = st.f();
}

// what is the difference between a generic struct and one
// that defines its type alias?

//
// the control of who instantiates the type: caller/callee


// pub fn testtt(a : Box<dyn T<Item = i32>>) -> ST{
//     return 3;
// }

/* ===== Super Traits ======
   ====================== */
// Rust doesn't have "inheritance", but you can define a
// trait as being a superset of another trait.
//
//  what is the difference between trait inheritance and class inheritance

//
// traits only extend the behaviour
// classes also extend the shape
// For example:
trait Person {
    fn name(&self) -> &String;
}
/*// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
// so a Student 'is a' Person
*/
trait Student: Person {
    fn university(&self) -> String;
}
trait Programmer {
    fn fav_language(&self) -> String;
    fn git_username(&self) -> String;
}
/*
// CompSciStudent (computer science student) is a subtrait
// of both Programmer and Student. Implementing CompSciStudent
// requires you to impl both supertraits
// and their supertraits
*/
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
// why can we have this "multiple inheritance" ?

// This function uses all the methods available as
// something that implements CompSciStudent
fn comp_sci_student_greeting(student: &impl CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        Programmer::git_username(student)
        // student.git_username()
    )
}

struct Entity{
    nm:String,
}
// QUIZ: how many IMPLs do we need now for
// Entity if we say it implements CompSciStudent?
// comment this impl, retype and see the issues
impl CompSciStudent for Entity{
    fn git_username(&self) -> String {
        String::from("squera")
    }
}
impl Programmer for Entity {
    fn fav_language(&self) -> String {
        String::from("Rust!")
    }
    fn git_username(&self) -> String {
        String::from("aswd")
    }
}
impl Student for Entity {
    fn university(&self) -> String {
        String::from("unitn")
    }
}
impl Person for Entity {
    fn name(&self) -> &String {
        &self.nm
    }
}

// let's now test the previous function on some Entity
pub fn example_supertraits(){
    let e = Entity{
        nm : String::from("marco")
    };
    let f = Entity{
        nm : String::from("pigna")
    };
    println!("{}",comp_sci_student_greeting(&e));
    println!("{}",comp_sci_student_greeting(&f));
}




/* ===== Polymorphism ======
   ========================= */
// To many people, polymorphism is synonymous with inheritance.
// They're wrong. ish.
// But it’s actually a more general concept that refers
// to code that can work with data of multiple types.
//
// Polymorphism comes in 3 ways in programming languages
// (https://en.wikipedia.org/wiki/Polymorphism_(computer_science))
//      parametric polymorphism
//          more similar to what we have here
//          Rust has Bounded parametric polymorphism,
//          because each type parameter can also be given Trait bounds
//      ad hoc polymorphism
//         overloading
//      subtyping polymorphism
//          inheritance

// Rust instead uses generics to abstract over different possible types
// and trait bounds to impose constraints on what those types must provide.
// This is sometimes called *bounded parametric polymorphism*.


trait Descrivibile {
    fn descrivi(&self) -> String;
}

fn stampa1<T: Descrivibile>(item: Box<T>) {
    println!("{}", item.descrivi());
}

fn stampa2(item: Box<impl Descrivibile>) {
    println!("{}", item.descrivi());
}

fn stampa3(item: Box<dyn Descrivibile>) {
    println!("{}", item.descrivi());
}