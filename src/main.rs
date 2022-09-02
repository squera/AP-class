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
use crate::full_files::c11_heap as c11;
// use crate::classes::c11_heap as c11;
use crate::full_files::c12_fp as c12;
// use crate::classes::c12_fp as c12;

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

        // from c03_enums
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

        // c11_heap
        c11::example_box();
        c11::example_box_long();
        c11::recursivetypes();
        c11::example_smart1();
        c11::example_drop();
        c11::example_rc();
        c11::implitictderef();
        c11::arc();
        c11::tests::it_sends_an_over_75_percent_warning_message();
        c11::workingtests::it_sends_an_over_75_percent_warning_message();
        c11::rc_plus_refcell::examplepcrefcell();
        c11::overflow::exampleoverflow();
        c11::graphexample();
        c11::cellexamplee();
        c11::rcwithcellexample();

        // c12_fp
        c12::closures::closuresexample();
        c12::closures::capturingexample();
        c12::closures::fntypes();
        c12::closures::closures_output();
        c12::closures::fprules();
        c12::iterators::iteratorexample();
        c12::iterators::filters_by_size();
        c12::iterators::examplefpiterators();
        c12::iterators::calling_next_directly();
        c12::iterators::using_other_iterator_trait_methods();

}
