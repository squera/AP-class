/* TODO: text+ import from lecture 04
    See
        https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
    Rust Project kinds:
        Binary (Application)
        Library

    Rust Project building blocks:
        Modules
        Paths
        Crates
        Packages

    Rust pubs
 */



//    open other local projects:
//      libtest
//      leaflib

//TODO explain
use libtest::toplevel_fun;
use libtest::pubmod::pubmodfun;
// we can also use the types declared in the other module and give it local aliases
use libtest::PubEnum as PE;


pub fn externalcall(){
    let s = toplevel_fun();
    println!("Got {}",s);
    let _en = PE::P1;

    // rand is the crate imported from crate.io
    // let's use the `random` function
    // see: https://rust-random.github.io/rand/rand/fn.random.html
    // and more of the docs
    use rand::random;
    let x: u8 = random();
    println!("Random u8: {}", x);

}
