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
// use crate::classes::c03_structenum as c4;

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

        // from c04_structenum
        c4::struct_usage();
        c4::struct_printing();
}
