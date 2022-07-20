/// This module shows some KEY concepts of Rust:
///     structs
///     impl

// TODO : add rustdoc links for impl

// A struct is a custom data type that lets you name and package together multiple related values
//  that make up a meaningful group.
//  Each piece of data and its name is called a field, different fields can have different types
// See
//      https://doc.rust-lang.org/book/ch05-00-structs.html

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
    // the one above is im
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
    // GOTO c02_ownership, line 404 try to access the struct -> not possible

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

pub fn struct_printing() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // struct s that derive Debug can be printed with the " :? " formatter
    println!("rect1 is {:?}", rect1);

    // TODO: derive display and implement .. ?
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
// GOTO TODO
// come back

// TODO: impl -> ### Defining methods for Structs in lecture 3
// TODO: lifetimes -> c5
// TODO: traits -> c6
// TODO: poly -> c7
// TODO: oop -> c8



