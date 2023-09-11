// https://www.theregister.com/2022/11/11/nsa_urges_orgs_to_use/


/// rust's management of Crates:
/// - mod component:
///     - in the main (for specifying at the crate-level what modules to be considered)
///     - in a mod.rs (pub mod ... ) for telling cargo that the folder contains these modules
///        with these names and these visibility modifiers
///     - in a file: to create a hierarchy of modules and namespaces
/// - what is a path : use crate:: // super::


/// lifetimes

pub struct Inner {
    value : i32
}
impl Inner {
    pub fn new()-> Inner {
        Inner {value:0}
    }
}
/// let's define a struct Inner, which we can't just call Goods for obv reasons
pub struct Container<'a>{
    content: &'a Inner,
    data: i32
}
/// let's define a struct Container that takes a pointer to an Inner
/// the Inner could be owned by sth else, and it makes sense to pass a pointer
/// we need to specify the lifetime to ensure the Inner lives long enough,
/// so the pointer inside Container is valid for the lifetime of Container
/// So the lifetime of the Inner must be at least as long as the lifetime of the Container
impl<'a> Container<'a> {
    pub fn new(a:&Inner) -> Container {
        Container { content:a,data:0}
    }
    // let's define a methhod for changing the content of a container
    pub fn addinner(&mut self, a:&'a Inner) {
        self.content = a;
    }
}

// now let's simulate other external functions that invoke the addinner
// this is your other code using the Container API

/// the function below is incorrect: why?
// pub fn modder(c: &mut Container){       // ----|
//     let a = Inner::new();         //
//     c.addinner(&a);                     //
// }                                       // ----|

//
/// The lifetime of a is shorter than the Container's! it's been allocated after!

/// To fix this, we need to pass the lifetime of Inner and make sure it matches what Container wants
pub fn alsogoodmodder<'a>(c:&mut Container<'a>, a:&'a Inner){
    c.addinner(a);
}
pub fn goodmodder<'a, 'b>(c:&'b mut Container<'a>, a:&'a Inner){
    c.addinner(a);
}

pub mod test{
    use crate::basedir::c99_QA::*;
    // use crate::lifetimes::lt::{*};

    pub fn main(){
        let mut a = Inner::new();
        let mut b = Container::new(&a);
        b.addinner(&a);
        // modder(&mut b);

        goodmodder(&mut b, &a);
    }
}



///
pub mod traitqa{
    use std::ops::{Add, Deref, DerefMut};

    pub struct S1{
        f1:i32
    }
    pub struct S2{
        f2:bool
    }
    pub trait Addable{
        fn get_i32(&self)-> i32;
        fn add(&mut self, o:&dyn Addable);
    }
    impl Addable for S1{
        fn get_i32(&self) -> i32 {
            self.f1
        }
        fn add(&mut self, o: &dyn Addable) {
            self.f1+=o.get_i32();
        }
    }
    impl Addable for S2{
        fn get_i32(&self) -> i32 {
            if self.f2 {0} else {1}
        }
        fn add(&mut self, o: &dyn Addable) {
            let mut tmp = false;
            if o.get_i32() == 0 {tmp = true;};
            self.f2 && tmp;
        }
    }

    pub fn testit(){
        let mut s1 = S1{f1:0};
        let mut s2 = S2{f2:false};
        let mut s3 = S2{f2:true};
        let mut s4 = S1{f1:10};

        let mut s5 = S1{f1:100};

        let mut v1 : Vec<&dyn Addable> = vec![(&s1),(&s2),(&s3),(&s4)];
        let mut v2 : Vec<&dyn Addable> = vec![(&s2),(&s3),(&s4),(&s1)];
        for el1 in v1.iter() {
            println!("i32 {}", el1.get_i32());
        }
        s1.add(&s2);

    }
}