#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
mod classes;

use crate::classes as basedir;
use classes::c01_basic as c1;
use classes::c01_std;
use classes::c02_ownership as c2;
use classes::c03_enums as c3;
use classes::c04_structs as c4;
use classes::c04_structshelper as c4b;
use classes::c07_generics as c7;
use classes::c08_lifetimes as c8;
use classes::c09_traits as c9;
use classes::c10_OOP as c10;
use classes::c11_heap as c11;
use classes::c12_fp as c12;
pub fn main() {
    // // // from c01_basic
    // c1::var_ass_mut();
    // c1::vals_types();
    // c1::expressions();
    //
    // // from c01_std
    // c01_std::strings();
    // c01_std::vec();
    // c01_std::hashmap()

    // c2::ownership();
    // c2::ownership_for_functions();
    // c2::refs_and_borrowing();
    // c2::slices();
    // c2::ownership_and_compound();
    //s
    // // from c03_enums
    // c3::enum_usage();
    // c3::option();
    // c3::patternmatching();
    // c3::errors();
    // c3::testqm();
    //
    // // from c04_structs
    // c4::struct_usage();
    // c4::struct_printing();
    // c4b::_showcase_access();
    // c4::struct_impl();
    // c4::ownstructs();

    // c3::testqm();
    // let r = c3::readfilecontent();
    // if r.is_err(){
    //         println!("Error {:?}", r.unwrap_err());
    // }
    // c4::ownstructs();

    // // from c05_modules
    // c5::externalcall();
    // c5::external_registry_call();
    //
    // // open c06_testing

    // c7::struct_generic();
    // c7::generics_example();
    // c7::explicit_type();

    //
    // c8::lifetime_test();
    // c8::uselongest();
    // c8::nll_example();
    // c8::main();
    //
    // // c09_traitspoly
    // c9::traitexample();
    // c9::example_notify();
    // c9::animals_example();
    // c9::example_supertraits();
    //
    // // c10_oop
    // c10::example_oop1();
    // c10::example_animals_oop();
    // c10::example_multiple_traits();
    //
    // // c11_heap
    // c11::example_box();
    // c11::example_box_long();
    // c11::recursivetypes();
    // c11::example_smart1();
    // c11::example_drop();
    // c11::example_rc();
    // c11::implitictderef();
    // c11::arc();
    // c11::refcell_usage();
    // c11::refcell_usage_2();
    // c11::tests::it_sends_an_over_75_percent_warning_message();
    // c11::workingtests::it_sends_an_over_75_percent_warning_message();
    // c11::rc_plus_refcell::examplepcrefcell();
    // c11::overflow::exampleoverflow();
    // c11::graphexample();
    // c11::par::arcmutex();
    // c11::par::arcrwlock();
    // c11::cellexamplee();
    // c11::rcwithcellexample();

    // // c12_fp
    // c12::closures::closuresexample();
    c12::closures::capturingexample();
    // c12::closures::fntypes();
    // c12::closures::closures_output();
    // c12::closures::fprules();
    // c12::iterators::iteratorexample();
    // c12::iterators::filters_by_size();
    // c12::iterators::examplefpiterators();
    // c12::iterators::calling_next_directly();
    // c12::iterators::using_other_iterator_trait_methods();

    // cqa::traitqa::testit();

    // c14::thread_spawning();
    // c14::thread_test();
    // c14::channel_example();
    // c14::mutex_example();
    // c14::arc_mutex();

}

// use std::marker::PointeeSized;

#[derive(Debug)]
enum C{
    C1(i32,i32),
    C2(bool,String)
}
#[derive(Debug)]
struct D{
    c : C,
    s : String
}
impl D{
    pub fn new() -> D{
        D{ c: C::C1(0,0), s: "".to_string() }
    }
    pub fn new_with_C(cc : C) -> D{
        let content = match &cc {
            C::C1(_, _) => String::from("not there"),
            C::C2(_, s) => s.clone()
        };
        D{
            c : cc,
            s: content
        }
    }
    pub fn larger(&self) -> usize {
        let con = match &self.c {
            C::C1(_,_) => 0,
            C::C2(_,ss) => ss.len(),
        };
        let l = self.s.len();
        return if l > con {l} else {con};
    }
}

// impl fmt::Display for D {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "D: {} with {:?}", self.s, self.c)
//     }
// }
// pub fn main () {
//     let c1 = C::C1(0,1);
//     let c2 = C::C2(true, String::from("no way jose"));
//     println!("gotten {:?}",D::new());
//     let d1 = D::new_with_C(c1);
//     println!("dbg {:?}",d1);
//     println!("fmt {}",d1);
//     let d2 = D::new_with_C(c2);
//     println!("dbg {:?}",d2);
//     println!("fmt {}",d2);
//     println!("larger {}",d2.larger());
//     println!("larger {}",d1.larger());
// }
