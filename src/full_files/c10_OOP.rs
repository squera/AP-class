/// This module shows some KEY concepts of Rust:
///     Object Oriented Programming (in Rust)
/// Some resources:
///     https://doc.rust-lang.org/book/ch17-00-oop.html
///     https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html
/// OOP languages share certain common characteristics, namely
///     1 objects,
///     2 encapsulation, and
///     3 inheritance.

/* ======= Objects =========
   ====================== */
//  > Object-oriented programs are made up of objects.
//  > An object packages both data and the procedures that operate on that data.
//  > The procedures are typically called methods or operations.[name=The Gang of Four book]
//
// Using this definition, Rust is object oriented: structs and enums have data, and `impl` blocks provide methods on structs and enums.
// this is akin to a Rectangle Class in Java
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}

/* ===== Encapsulation =====
   ====================== */
// The option to use `pub` or not for different parts of code enables
//      encapsulation
// of implementation details.
// Remember that by default, everything is
// QUIZ: public / private
//      private.
// If we want to access methods or functions in public API, we need to specify the
//      `pub`
// keyword before our data type.

// let's define a new struct, for a collection of numbers that keeps its average
pub struct AveragedCollection {
    // We leave the `list` and `average` fields private so that
    // there’s no way for external code to add or remove items to the list field directly,
    // otherwise the `average` field might become out of sync when the list changes.
    // The `average` method returns the value in the `average` field,
    // allowing external code to read the average but not modify it.
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    // The public method `add` is the only way to modify an instance of `AveragedCollection`.
    // When an item is added to list using the `add` method
    // the implementation calls the private `update_average` method
    // that takes care of updating the average field as well.
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn get_average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
// Because we’ve encapsulated the implementation details of
// `AveragedCollection`, we can easily change aspects like the data structure in the future.
// For instance, we could use a `HashMap` instead of a `Vec` for the list field.
// As long as the signatures of the `add`, `remove`, and `average` public methods stay the same,
// code using `AveragedCollection` wouldn’t need to change.
// If we made `list` public instead, this wouldn’t necessarily be the case:
// `HashMap` and `Vec` have different methods for adding and removing items,
// so the external code would likely have to change if it was modifying list directly.
//
// If encapsulation is a required aspect for a language to be considered object-oriented,
// then Rust meets that requirement.
// The option to use `pub` or not for different parts of code enables encapsulation of implementation details.

/* ===== Inheritance =======
   ====================== */
/*  Inheritance is a mechanism whereby an object can inherit
    from another object’s definition, thus gaining the parent object’s
    data and behavior without you having to define them again.

    It's the 'is - a' relation you've seen in P2

    In Rust, there is no way to define a struct that inherits
    the parent struct’s fields and method implementations.

    Why does one want Inheritance to begin with, though?
        A) code reuse
        B) subtyping polymorphism: using an object of type A where one of type B is expected, if A extends B

    Rust provides some features for both
        A and B

    A) For A, we have Generics and Traits, as we've already seen
    B) For B, the picture becomes more complex ...
 */

// Subtyping relates to dynamic dispatch.
// The Rust we've seen for now only has static dispatch, it needs to know the type of
// all objects statically.
// For subtyping, we'd need to add some form of Dynamic Dispatch to Rust,
// which is possible with
//      Trait Objects
// which, technically, is an object

// A trait object `&dyn Trait` (for some Trait) points to both
//      an instance of a **pointer type** implementing our specified trait
//      as well as a table used to look up trait methods on that type at runtime.

// Example: using Trait objects in Vectors.
// Recall: the Rust compiler restricts that all the values in a vector **must** have the same type.
// Even if we define a generic type, the generic type can be substituted with **one** concrete type at a time.

fn wrong_Vecs() {
    // Vec<T>
    let v1 = vec![1u32, 1u32];
    let v2 = vec![1u64, 1u64];
    // DNC: error[E0308]: mismatched types
    // let v3 = vec![1u32, 1u64];
}

// A **Trait object** tells rust to pass all types that implement the trait.
// so we can declare a Trait: Show and use it to populate a vector of Show s
trait Show {
    fn show(&self) -> String;
}
// we define this trait for 2 known types,
// note that we're defining Traits for non-struct types!
// even worse, types of the standard library!
// Rust follows this orphan rule: if you want to implement a trait for your data type, at least one of them must be local.
// This means you cannot implement `Deref` trait for `Vec`,
// because both of them are from other crates.
impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}
impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}
pub fn example_oop1() {
    let answer = 42;
    let maybe_pi = 3.14;
    // we need to indicate that the vector is for Trait Objects: thus the `dyn`
    let v: Vec<&dyn Show> = vec![&answer,&maybe_pi];
    for d in v.iter() {
        println!("show {}",d.show());
    }
}

// So now we've been able to use Trait Objects in order to store stuff of
// the same trait in a vec

// let's look at more inheritance --
// this is reminiscent of an interface and two classes implementing it

// This looks like an interface
trait Quack {
    fn quack(&self);
}
// this is the first class
struct Duck ();
// this looks like Duck implements Quack , in Java
impl Quack for Duck {
    fn quack(&self) {
        println!("quack!");
    }
}
// second class
struct RandomBird {
    is_a_parrot: bool
}
// this looks like RandomBird implements Quack , in Java
impl Quack for RandomBird {
    fn quack(&self) {
        if ! self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}

pub fn example_animals_oop() {
    let duck1 = Duck();
    let duck2 = RandomBird { is_a_parrot: false };
    let parrot = RandomBird { is_a_parrot: true };

    let ducks: Vec<&dyn Quack> = vec![&duck1, &duck2, &parrot];

    // QUIZ: what does this print ?
    // qqs | qss | qqq
    for d in &ducks {
        d.quack();
    }

    println!("And now with subtleties:");
    // now some subtleties, these 2 pairs of functions print the same things, but operate differently
    quack_ref(&duck1);
    quack_ref(&parrot);
    quack_trait(&duck1);
    quack_trait(&parrot);
}
// QUIZ: what is the difference between these two?
fn quack_ref (q: &dyn Quack) {
    q.quack();
}
fn quack_trait<Q> (q: &Q)
    where Q: Quack {
    q.quack();
}
// The type parameter Q in quack_trait is any type which implements Quack.
// There's an important difference between quack_trait and the quack_ref.
// The body of quack_trait is compiled for each of the calling types and no virtual method is needed;
// such functions can be completely inlined.
// It uses the trait Quack in a different way, as a constraint on generic types.
// Instead, quack_ref uses Trait Objects, and it'll look up into the paramter Trait object the
// v-table where to do the method dispatch for quack();


/* Problems with Inheritance: how to subclass?

    consider Vehicles - ranging from bicycles to 300t ore trucks.
    There are multiple ways to think about vehicles,
        road-worthiness (all-terrain, city, rail-bound, etc),
        power-source (electric, diesel, diesel-electric, etc),
        cargo-or-people, and so forth.
    Any fixed hierarchy of classes you create based on one aspect ignores all other aspects.
    That is, there are multiple possible classifications of vehicles!

    A similar thing happened to me when coding a monster advancer for dnd 3.5,
    it's easy enough to subtype monsters by type: Aberration, Undead, etc,
    but what happens when you add Archetypes to a monster? you're adding stuff from
    a 'sibling' type into a type, it's a nightmare!
    With traits, this is much more simple to do

    You could, instead, compose these different aspects, with Traits, into the Vehicle struct!
    Generally in Rust there is trait inheritance, which is done by
        :  and  +
    in trait signatures
 */

// recall the trait Show from above, this is a new trait
trait Location {
    fn location(&self) -> String;
}
// Recall the : notation from `trait CompSciStudent: Programmer + Student`.
trait ShowTell: Show + Location {}
// The last trait simply combines our two distinct traits into one, although it could specify other methods.

#[derive(Debug)]
struct Foo {
    name: String,
    location: String
}
// some methods for allocating this struct
impl Foo {
    fn new(name: &str, location: &str) -> Foo {
        Foo{
            name: name.to_string(),
            location: location.to_string()
        }
    }
}
// let's make Foo implement Show
impl Show for Foo {
    fn show(&self) -> String {
        self.name.clone()
    }
}
// and also Location
impl Location for Foo {
    fn location(&self) -> String {
        self.location.clone()
    }
}
impl ShowTell for Foo {}

// let's create 3 simple functions to tell the types
fn transferShow(x: &dyn Show) -> &dyn Show {
    x
}
fn transferLocation(x: &dyn Location) -> &dyn Location {
    x
}
fn transferShowTell(x: &dyn ShowTell) -> &dyn ShowTell {
    x
}

// Now, if I have a value foo of type Foo, then a reference to that value will satisfy
// &Show, &Location or &ShowTell (which implies both.)
pub fn example_multiple_traits(){
    let f = Foo::new("n", "a");
    // let's look at the types of these three variables
    let ff1 = transferShow(&f);
    let ff2 = transferLocation(&f);
    let ff3 = transferShowTell(&f);
    // we can effectively treat, and use `f` as something of all the other Trait types!
    // look at the autocomplete suggestion for each of ff1, ff2, ff3
    println!(" Foo's: {}, Show's: {}, Location's {}, ShowTell's: {}", f.name, ff1.show(), ff2.location(), ff3.show() );
}