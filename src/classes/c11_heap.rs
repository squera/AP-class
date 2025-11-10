/* ========== Box ==========
   ========================= */
pub fn example_box(){
    let b = Box::new(5);
    println!("b's value = {}", *b);
    println!("b's address = {}", b);
    println!("b's real address = {:p}", b);
    println!("b's another address = {:p}", &b);
}

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn example_box_long() {
    let point: Point = origin();    // Stack allocated
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    }); // Heap allocated
    let boxed_point: Box<Point> = Box::new(origin());
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes on the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack", mem::size_of_val(&rectangle));

    // QUIZ:
    // what's going to be printed next?
    // 8 | 16 | 32
    println!("Boxed point occupies {} bytes on the stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack", mem::size_of_val(&box_in_a_box));

    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the heap", mem::size_of_val(&unboxed_point));
}
// Recursive types
//QUIZ:
// which of the following is a recursive type?
// Array | List | Stream | Pair

// uncomment, comment
// enum WishList{
//     WCons(i32, WishList),
//     WNil,
// }
// DNC: error[E0072]: recursive type `List` has infinite size

enum List {
    Cons(i32, Box<List>),
    Nil,
}
use self::List::{Cons,Nil};
pub fn recursivetypes(){
    // let list : WishList = WCons(1, WCons(2, WCons(3, WNil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

/* ========= Deref =========
   ========================= */
use std::ops::Deref;

struct MyBox<T>{
    el : T,
    idx : i32
}
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox{el:x,idx:0}
    }
}

pub fn example_smart1() {
    // first, we test on the canonical Box
    let x = 5;
    let y = Box::new(x);

    println!("I expect 5: {}", x);
    println!("I expect 5: {}", *y);
    // Behind the scenes what is actually run is this.
    println!("I expect 5: {}", *(y.deref()));  // *(y.deref())

    let x = 5;
    let y = MyBox::new(x);

    // QUIZ: what does the second println print?
    println!("I expect 5: {}", x);
    // comment the IMPL below
    println!("I expect! 5: {}", *y);
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    // type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.el
        // &self.idx
    }
}


/* ========== Drop =========
   ========================= */
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
pub fn example_drop() {
    let mut c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    {
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        c = d;
    }
    println!("End of function");
}

/* ========== Rc ===========
   ========================= */
pub fn example_rc(){
    // uncomment these 3 lines
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(RcCons(5,
                           Rc::new(RcCons(10,
                                          Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // QUIZ: what is the sequence of printed numbers?
    // 1, 2, 3, 4 | 1, 2, 3, 2 | 1, 2, 3 ,3

    let t = Box::new(10);
    let tt = Rc::new(t);
    let ttt = (tt.clone());
    let tttt = (tt.clone());
    let y = Rc::new(tt);
}

use std::rc::Rc;
use self::RcList::{RcCons,RcNil};

enum RcList {
    RcCons(i32, Rc<RcList>),
    // Cons(i32, Box<List>)
    RcNil,
}
/* ========== Arc ==========
   ========================= */
pub fn arc() {
    use std::sync::Arc;
    use std::thread;
    use std::thread::sleep;

    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        // QUIZ: can i replace Arc with Rc?

        let tjh = thread::spawn(move || {
            println!("{:?}", apple);
            println!("count after creating apple in a thread: {}", Arc::strong_count(&apple));
        });
        // tjh.join();
    }

}

struct NaiveRc<T> {
    reference_count: usize,
    inner_value: T,
}

impl<T: Copy> Clone for NaiveRc<T> {
    fn clone(&self) -> Self {
        // QUIZ: why does this code not compile?
        // self.reference_count += 1;
        return NaiveRc{
            reference_count : self.reference_count,
            inner_value : self.inner_value.clone()
        }
    }
}
impl<T: Copy> NaiveRc<T> {  // ughh
    fn clone_mut(&mut self) -> Self{
        self.reference_count += 1;
        return NaiveRc{
            reference_count : self.reference_count,
            inner_value : self.inner_value.clone(),
        }
    }
}

/* ==== Implicit Deref =====
   ========================= */
/*
- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`
- NO `&T` to `&mut U` when `T: DerefMut<Target=U>`
 */
fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn implitictderef() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // hello(&(*m)[..]);
}


/* ======== RefCell ========
   ========================= */
// Interior mutability is a design pattern in Rust
// that allows you to mutate data even when there are immutable references to that data;
// QUIZ: what rust principles dictate this?

use std::cell::RefCell;

pub fn refcell_usage() {
    let refcell = RefCell::new(10);
    println!("refcell {:?}", refcell, );
    let mut mptr = refcell.borrow_mut();
    println!("refcell {:?} + content {}", refcell, *mptr);
    *mptr = 11;
    println!("refcell {:?} + content {}", refcell, *mptr);

    mem::drop(mptr); // comment for panic.
    let ptr1 = refcell.borrow();
    let ptr2 = refcell.borrow();
    let ptr3 = refcell.borrow();
    println!("refcell {:?} + content {}",refcell,*ptr1 );

    // QUIZ: why can i call borrow without panicking?
}

pub fn refcell_usage_2(){
    let refcell = RefCell::new(10);
    println!("refcell {:?}",refcell,);
    let mut ptr1 = refcell.borrow();
    let mut ptr2 = refcell.borrow();
    let mut ptr3 = refcell.borrow();
    println!("refcell {:?} + content {}",refcell,*ptr1 );
    println!("refcell {:?} + content {}",refcell,*ptr2 );
    // can we use these read-only pointers though for writing?
    // uncomment to find out
    // *ptr3 = 11;
    // println!("refcell {:?} + content {}",refcell,*ptr3 );
}

/* == Interior Mutability ==
   ========================= */
pub trait Messenger {
    fn send(&self, msg: &str);
}
// we need to specify both the lifetime and the type of the messenger field
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>  where T: Messenger, {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub mod tests {
    // let's use all that is defined outside this module tests
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // QUIZ: What is the problem with this code?
            // self.sent_messages
            //     .push(String::from(message));
        }
    }

    // added but not used
    pub fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.len(), 1);
        //[QUIZ]: what trait implementations are needed to  ...
    }
}

pub mod workingtests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // compare to the previous definition!
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        //In the new function, we create a new `RefCell<Vec<String>>` instance around the empty vector.
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // compare to the previous def!
        fn send(&self, message: &str) {
            self.sent_messages
                .borrow_mut()
                .push(String::from(message));
        }
    }

    pub fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

/* ====== Rc + RefCell =====
   ========================= */
pub mod rc_plus_refcell {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use std::cell::RefCell;
    use std::rc::Rc;
    use self::List::{Cons,Nil};

    pub fn examplepcrefcell() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        // Graph:
        /*
            { Nil } <----------------|
                                     |
            {  5  } <------------|   |
                                 |   |
  |---------------> |-> a ---> [ | , | ]
  |                 |
  |   b ----> [ | , |]
  |             |----------------------------> { 3 }
  |
  |-----------------|
                |---+----------> { 4 }
      c ----> [ | , |]

         */
        // Q: what Rust feature is used here ?
        let x = *value.borrow_mut() += 10;
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

/* === Reference cycles ====
   ========================= */
pub mod overflow {
    use std::cell::RefCell;
    use std::rc::Rc;
    use self::List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
        fn head(&self) -> Option<&i32> {
            match self {
                Nil => None,
                Cons( e, _) => Some(e)
            }
        }
    }

    pub fn exampleoverflow() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        // QUIZ: What will happen?
        // println!("a next item = {:?}", a.head());
        // and here?
        println!("a next item = {:?}", a.tail());
    }
}


/* ======== Graphs =========
   ========================= */
struct Graph<T> {
    nodes: Vec<Node<T>>,
}
struct Node<T>(NodeRef<T>);
// The private representation of a node.
struct _Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}
type NodeRef<T> =
Rc<RefCell<_Node<T>>>;
impl<T> Node<T> {
    // Creates a new node with no edges.
    fn new(inner: T) -> Node<T> {
        let node = _Node {
            inner_value: inner,
            adjacent: vec![]
        };
        Node(Rc::new(RefCell::new(node)))
    }

    // Adds a directed edge from this node to other node.
    fn add_adjacent(&self, other: &Node<T>) {
        (self.0.borrow_mut()).adjacent.push(other.0.clone());
    }
}

impl<T> Graph<T> {
    fn with_nodes(nodes: Vec<Node<T>>) -> Self {
        Graph { nodes: nodes }
    }
}

pub fn graphexample() {
    // Create some nodes
    let node_1 = Node::new(1);
    let node_2 = Node::new(2);
    let node_3 = Node::new(3);

    // Connect some of the nodes (with directed edges)
    node_1.add_adjacent(&node_2);
    node_1.add_adjacent(&node_3);
    node_2.add_adjacent(&node_1);
    node_3.add_adjacent(&node_1);

    // Add nodes to graph
    let graph = Graph::with_nodes(vec![node_1, node_2, node_3]);

    // Show every node in the graph and list their neighbors
    for node in graph.nodes.iter().map(|n| n.0.borrow()) {
        let value = node.inner_value;
        let neighbours = node.adjacent.iter()
            .map(|n| n.borrow().inner_value)
            .collect::<Vec<_>>();
        println!("node ({}) is connected to: {:?}", value, neighbours);
    }
}

/* ===== Mutex / RWLock =======
   ========================= */
pub mod par{
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex, RwLock};
    use std::thread;

    pub fn arcmutex() {
        // let counter = Arc::new(RefCell::new(0));
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                println!("I am thread number {}",num);
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }
    pub fn arcrwlock() {
        let counter = Arc::new(RwLock::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.write().unwrap();
                println!("I am thread number {}",num);
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.read().unwrap());
    }
}


/* ========= Cell ==========
   ========================= */
use std::cell::Cell;
use std::time::Duration;

fn foo(cell: &Cell<u32>) {
    let value = cell.get();
    cell.set(value * 2);
}
pub fn cellexamplee() {
    let cell = Cell::new(1);
    let new_value = cell.get() + 10;
    println!("cell value : {}", cell.get());
    foo(&cell);
    println!("cell value : {}", cell.get());
    cell.set(new_value);
    // QUIZ: what will this print?
    println!("cell value : {}", cell.get());
}
struct NaiveRcWithCell<T> {
    inner_value: T,
    references: Cell<usize>,
}

// implement its creation
impl<T> NaiveRcWithCell<T> {
    fn new(inner: T) -> Self {
        NaiveRcWithCell {
            inner_value: inner,
            references: Cell::new(1),
        }
    }
    fn references(&self) -> usize {
        self.references.get()
    }
}
impl<T: Clone> Clone for NaiveRcWithCell<T> {
    fn clone(&self) -> Self {
        self.references.set(self.references.get() + 1);
        NaiveRcWithCell {
            inner_value: self.inner_value.clone(),
            references: self.references.clone(),
        }
    }
}
pub fn rcwithcellexample() {
    let wrapped = NaiveRcWithCell::new("Hello!");
    println!("references before cloning: {:?}", wrapped.references());
    let wrapped_clone = wrapped.clone();
    println!("references after cloning: {:?}", wrapped.references());
    println!("clone references: {:?}", wrapped_clone.references());
    let wrapped_clone2 = wrapped.clone();
    println!("references after cloning2: {:?}", wrapped.references());
    println!("references after cloning2: {:?}", wrapped_clone.references());
    println!("references after cloning2: {:?}", wrapped_clone2.references());
}

