
// This code is not for this class, it is needed for class c04_structs
//  in the following code, we cannot access the struct name,
//  let alone its fields
// use crate::full_files::c04_structs::Rectangle;
// DNC: Struct `Rectangle` is private [E0603]

// This code is not for this class, it is needed for class c04_structs
use crate::full_files::c04_structs::new_rhombus;
use crate::full_files::c04_structs::Square;
use crate::full_files::c04_structs::Rhombus;
fn _showcase_access () {
    // QUIZ: can i write the following:
    // let rr = Rhombus{ side: 0, acute_angle: 0 };
    // Y/N

    // DNC: error[E0451]: field `acute_angle` of struct `Rhombus` is private

    let rr = new_rhombus();
    // notice that the `side` is still immutable, so it cannot be assigned to
    // rr.side = 10;
    // DNC: error[E0594]: cannot assign to `rr.side`, as `rr` is not declared as mutable
    let _x = rr.side;
    // and the `acute_angle` field is private, so inaccessible
    // let _a = rr.acute_angle;
    // DNC: error[E0616]: field `acute_angle` of struct `Rhombus` is private
}
// GOTO back to c04_structs