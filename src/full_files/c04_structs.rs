/// This module shows some KEY concepts of Rust:
///     structs
///     impl

/// Material for this module:
///      https://doc.rust-lang.org/book/ch05-00-structs.html

// A struct is a custom data type that lets you name and package together multiple related values
//  that make up a meaningful group.
//  Each piece of data and its name is called a field, different fields can have different types

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/// This function showcases Rust structs and how to use them
pub fn struct_usage(){
    // this creates a new instance of the User struct
    // note: initialisation of fields is done with " : " , that may be confusing and source of errors
    // also, fields are separated by " , "
    // and, fields need not be instantiated in the same order in which they are defined
    let _user0 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // " user0 " is an immutable struct
    // _user0.email = String::new();



    // DNC: error[E0594]: cannot assign to `user0.email`, as `user0` is not declared as mutable
    // the one above is immutable too
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // in this case, we can mutate the fields of user1
    //  note, we can mutate **ALL** the fields,
    // we cannot make a single field mutable
    user1.email = String::new();
    user1.username = String::new();

    // we can access the fields of the struct in this module because this code is defined
    // together with the struct
    // QUIZ: if i go into a different file `c04_structshelper`, can i write
    // let x = user1.email;
    // Y / N


    // GOTO != file and try to access the struct

    // shorthands
    // initialising fields from variables with the same name
    let email = String::from("someone@example.com");
    let username = String::from("someusername123");

    let _user2 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };

    let user3 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // The following two commands are same: first one with struct update syntax, second one without
    let _user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user3
    };
    let _user5 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
}


// Oftentimes you want to print out a struct
// the simplest way is to 'derive' a Trait called 'Debug'
// that offers simple pretty-printing facilities
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// alternatively, you have to implement the fmt method as follows:
// this is a way for rust to import the `fmt` module from the `std` crate
// for now, it suffices to say this is an import, we'll look at these things in detail soon
use std::fmt;
/// See
///     https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for Square {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}x{}", self.side,self.side)
    }
}

pub fn struct_printing() {
    let rect = Rectangle{ width: 10, height: 20 };
    let squa = Square{side: 10};
    // println!("Printing a rectangle {}", rect);
    // DNC: error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // so we have to use the {:?}
    println!("Printing a rectangle {:?}", rect);
    println!("Printing a square {}", squa);
}

// We can also use `pub` to designate structs (and enums) as public,
// but there are a few extra details. If we use pub before a struct definition,
// we make the struct public, but the struct’s fields will still be private.
// We can make each field public or not on a case-by-case basis.

// NOW: define these structs that are public,
// and then two functions that return instances of these functions outside
// we need these functions because we can't create instances of these structs outside
// since their fields are private
/// a public struct `Square` that can be initialised in other modules, but whose field is inaccessible
pub struct Square {
    side: u32
}
/// a public struct `Rhombus` with a public `side` field and a private angle
pub struct Rhombus {
    pub side: u32,
    acute_angle: i32,
}
/// Function that returns a `Rhombus`
pub fn new_rhombus() -> Rhombus{
    return Rhombus{ side: 0, acute_angle: 0 };
}
/// Function that returns a `Square`
pub fn _new_square() -> Square{
    return Square{ side: 0 };
}
// GOTO structshelper file
// come back

/* Struct "impl"s.
    Rust supports object-oriented programming (OOP).
     In OOP one defines methods in a class.
     In Rust, this is called methods of structs,
     which are implemented using the `impl` keyword.
     One struct can have multiple `impl` blocks.
 */

impl Rectangle {
    /// a private function for the Rectangle struct
    // we use `&self` instead of rectangle: `&Rectangle`
    // because Rust knows the type of `self` is `Rectangle`
    // due to the "impl Rectangle" above.
    // Note that we still need to use the `&` before `self`,
    // and this checks out with OO languages where objects are passed
    // by reference (&) and not by copy.
    // Methods
    // 1) can take ownership of self,
    //      like the 'take_ownership' method (see its signature),
    // 2) borrow `self` immutably
    //      as we’ve done here,
    // 3) or borrow `self` mutably
    //      as the function 'double' does
    fn area(&self) -> u32 {
        self.width * self.height
    }
    /// a public method
    // methods can be public too
    pub fn perimeter(& self) -> u32 {
        return self.height * 2 + self.width * 2;
    }
    fn double(&mut self) {
        self.width = self.width * 2;
        self.height = self.height * 2;
    }
    fn take_ownership(self) {
    }
    // QUIZ: are these methods or functions:
    // pub fn test1(&self) ...
    // fn test2(&self, arg: int) ...
    // fn test3(arg : int) ...
}

// Sometimes we want to define functions for structs.
// The difference between a *method* and a *function* of a struct
// is that a *function* of a struct doesn't need an instance of the struct to work with.
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    // one often defines constructors this way
    pub fn new() -> Rectangle {
        Rectangle{ width: 10, height: 20 }
    }
    // note that there is no function overloading,
    // so the following is not correct,
    // you have to change the name of the function
    // DNC: error[E0201]: duplicate definitions with name `new`:
    // pub fn new( width : u32, height : u32) -> Rectangle {
    //     Rectangle{ width, height }
    // }
    pub fn new_with_params( width : u32, height : u32) -> Rectangle {
        Rectangle{ width, height }
    }
}

pub fn struct_impl(){
    // this is how functions of a struct are called, i.e., with  ::
    let r = Rectangle::new();
    // this is how methods are called
    // note that the `&self` parameter is written in dot notation
    let a = r.area();
    // they can also be written in infix form, but this is strongly discouraged
    let p = Rectangle::perimeter(&r);
    println!("The Rectangle is {:?}",r);
    println!("Area: {} and Perimeter: {}", a, p);
}


// While you learn where each type is stored, use the cheatsheet below
// but note that it contains a lot more types than we have seen or that we will see:
//  https://cs140e.sergio.bz/notes/lec3/cheat-sheet.pdf

// additions

struct Test {
    pub f: i32,
    pub s: Vec<i32>,
}
pub fn ownstructs() {
    let mut example = Test {
        f: 32,
        s: vec![1],
    };
    let new_f = example.f;
    let new_s = example.s;
    println!("First {}", new_f);        // copied or moved ?
    println!("Second {:?}",new_s);      // copied or moved ?
    // who owns the vector of 1s?
    // println!("vec {}",example.s);
    // return example;
}

pub fn testvec(){
    let mut v = vec![5];
    v.push(6);

    let sixindex = findinv(&v);
    v.push(9);
}
fn findinv(v : &Vec<i32>) -> i32 {
    let mut counter =0;
    for x in v.iter() {
        if *x == 6{
            return counter;
        }
        counter+=1;
    }
    return -1;
}