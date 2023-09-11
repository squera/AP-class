/// Material for this module:
///     https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

/// As you write large programs, organizing your code will be important.
/// As a project grows, you can organize code by splitting it into multiple modules and then multiple files.
/// A package can contain multiple binary crates and optionally one library crate.
/// Rust has a number of features that allow you to manage your code’s organization, including which details are exposed,
/// which details are private, and what names are in each scope in your programs.
/// These features, sometimes collectively referred to as the module system, include:
///     Crates
///     Packages
///      Modules
///     Paths
/*
Crates:
    A crate is the smallest amount of code that the Rust compiler considers at a time.
    Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.
    A crate can come in one of two forms:
        a binary crate or
        a library crate.
    Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server.
    Each must have a function called main that defines what happens when the executable runs.
    Library crates don’t have a main function, and they don’t compile to an executable.
    Instead, they define functionality intended to be shared with multiple projects

Packages
    A package is a bundle of one or more crates that provides a set of functionality.
    A package contains a Cargo.toml file that describes how to build those crates.
    Cargo is actually a package that contains the binary crate for the command-line tool you’ve been using to build your code.
    The Cargo package also contains a library crate that the binary crate depends on.
    Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses.
    A package can contain as many binary crates as you like, but at most only one library crate.
    A package must contain at least one crate, whether that’s a library or binary crate.

Modules:
    Modules let us organize code within a crate into groups for readability and easy reuse.
    Modules also control the privacy of items, which is
    whether an item can be used by outside code (public) or
    is an internal implementation detail and not available for outside use (private).

    Using `mod` we can build a tree of definitions, which is often called the module tree. Paths to definitions in the module tree can take two forms:
    - An absolute path starts from a crate root by using a crate name or a literal crate.
    - A relative path starts from the current module and uses self, super, or an identifier in the current module.
 */
//    open other local projects to see the visibility of modules: these projects define modules
//      libtest
//      leaflib


// We now define the path of the functions we import from the `libtest` crate
// the `libtest` crate is defined in `Cargo.toml`
use libtest::toplevel_fun;
// here we are traversing the public module hierarchy to import another function
use libtest::pubmod::pubmodfun;
// we can also use the types declared in the other module and give it local aliases
use libtest::PubEnum as PE;
/*
Paths:
    Using a semicolan after the `mod temp` rather than a block tells rust to load the contents of the module from another file with the same name as the module.
        See the `main.rs` for a usage.
    We can then bring specific definitions into scope with the `use` keyword as above

    Standard crates can be brought into scope with the use keyword.
    Sometimes you might want to use libraries not contained within `std`.
    To do this, we'll need to use `Cargo.toml`.

    The Cargo.toml file for each package is called its manifest.
    It is written in the TOML format.
    Every manifest file consists of many sections, look online for details

    Look at the `Cargo.toml` of this project
    under
        dependencies
    it has 3 kinds of dependencies:
    // QUIZ: what are they?
        a local package
        a crates io package
        a local registry (kellnr, hosted by the uni) package

    We can also depend on a library located in a `git` repository. Your package can depend on any specified version or commit.

    ```
    [dependencies]
    piston_window = { version = "0.120.0", git = "https://github.com/PistonDevelopers/piston" }
    rand = { git = "https://github.com/example/rand", rev = "9f35b8e" }
    ```
 */


pub fn externalcall(){
    // let us now try to use the imported functions
    let s = toplevel_fun();
    let ss = pubmodfun();
    println!("Got {} and {}", s, ss);
    // we can also declare something of the imported Enum
    let _en = PE::P1;
    // println!("Enum {:?}",en);
    // DNC: error[E0277]: `PubEnum` doesn't implement `Debug`
    // we can't really print it though,
    // here we can fix it, only because it is local code,
    // but what if that were a downloaded library?

    // not all crates are local, the most useful repository for crates is 'crates.io'.
    // see Cargo.toml to see the import of crate `rand`

    // rand is the crate imported from crate.io
    // let's use the `random` function
    // see: https://rust-random.github.io/rand/rand/fn.random.html
    // and more of the docs
    use rand::random;
    let x: u8 = random();
    println!("Random u8: {}", x);
}

// in this course you'll use a local repository too, in order to share your code
// see the project `.cargo/config` file for defining the local repo
// then see Cargo.toml for the import of the kellnrtest crate
use kellnrtest::external_function;

pub fn external_registry_call(){
    println!("received true {}", external_function())
}