#![allow(non_snake_case)]
// This is the main executable file of our Rust project

// Below are the modules whose functions we can call from this file:
// we'll explain modules in the future
// For now, know modules have the names of their directory, see file mod.rs there
mod full_files;
mod classes;
use crate::full_files::c01_basic as c1;
// use crate::classes::c01_basic: as c1;
use crate::full_files::c02_ownership as c2;
// use crate::classes::c02_ownership as c2;
use crate::full_files::c03_enums as c3;
// use crate::classes::c03_enums as c3;
use crate::full_files::c04_structs as c4;
// use crate::classes::c04_structs as c4;
use crate::full_files::c05_modules as c5;
// use crate::classes::c05_modules as c5;
use crate::full_files::c06_testing as c6;
// use crate::classes::c06_testing as c6;
use crate::full_files::c09_traits as c9;
// use crate::classes::c09_traits as c9;
use crate::full_files::c10_OOP as c10;
// use crate::classes::c10_OOP as c10;

// Below is the main function.
// Notice that CLion already knows we can run it (see the green triangle)
// Functions are written with the `fn` keyword
// and they can be prefixed by a visibility modifier :
//      pub => public, callable by other functions that import this module
//      if one writes no modifier => private, only callable from this file
pub fn main() {

        // from c01_basic
        c1::var_ass_mut();
        c1::vals_types();
        c1::expressions();

        // from c02_ownership
        c2::strings();
        c2::vec();
        c2::hashmap();
        c2::ownership();
        c2::refs_and_borrowing();
        c2::slices();
        c2::ownership_and_compound();

        // from c03_optionerrorpattern
        c3::enum_usage();
        c3::option();
        c3::patternmatching();
        c3::errors();
        c3::collectionerrors();

        // from c04_structs
        c4::struct_usage();
        c4::struct_printing();
        c4::struct_impl();

        // from c05_modules
        c5::externalcall();
        c5::external_registry_call();

        // open c06_testing
        // open c07_project

        // open c08_lifetimes

        // c09_traitspoly
        c9::struct_generic();
        c9::generics_example();
        c9::traitexample();
        c9::example_notify();
        c9::animals_example();
        c9::example_supertraits();

        // c10_oop
        c10::example_oop1();
        c10::example_animals_oop();
        c10::example_multiple_traits();
}
