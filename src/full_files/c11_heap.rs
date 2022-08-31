/// This module shows some KEY concepts of Rust related to
///     heap management
/// The heap is handled through pointers, and pointers are regulated
/// by ownership, so heap-management contents in Rust are quite peculiar.
/// See
///         https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
/// and
///         https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
///         https://ricardomartins.cc/2016/06/08/interior-mutability
///
/// On the heap you store stuff using
///             Smart pointers
/// we'll see these instances of Smart Pointers:
///     Box
///     Rc
///     Arc
///     Ref & RefCell
/// since what you store is accessed by dereferencing a (smart) pointer, we'll also cover
///     The Deref Trait
///     The Drop Trait
///     Implicit Deref Coercions
/// and we'll cover some advanced topics such as
///     Interior Mutability
///     Reference Cycles
/// While you learn where each type is stored, use the cheatsheet below:
///         https://cs140e.sergio.bz/notes/lec3/cheat-sheet.pdf

/* ========== Box ==========
   ========================= */
// Boxes allow you to store data on the heap rather than the stack.
// What remains on the stack is the pointer to the heap data.
// Typical usages:
//  - When you have a type whose size can’t be known at compile time
//      and you want to use a value of that type in a context that requires an exact size
//  - When you have a large amount of data and you want to transfer ownership
//      but ensure the data won’t be copied when you do so
//  - When you want to own a value and you care only that it’s a type
//      that implements a particular trait rather than being of a specific type
//         --> Remember the Trait Objects !

pub fn example_box(){
    // variable `b` is a pointer to a cell in the heap, the content of the cell is 5
    let b = Box::new(5);
    // we dereference using ' * '
    println!("b's value = {}", *b);
    // but this does not work: Rust performs automatic dereferencing for us -- more on that later --
    println!("b's address = {}", b);
    // so if we really need to know the pointer, we need to print it with the pointer formatter:
    println!("b's real address = {:p}", b);
}

// let's look at a bigger example
// we import `mem` for some functions we use later on
use std::mem;

// define 2 simple structs: Point and Rectangle
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
// A Rectangle can be specified by where its top left and bottom right Points
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
/// a function returning the origin point
fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
/// a function returning the origin point, but boxed
fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn example_box_long() {
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection: we can put a box in a box
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    // let's now print a few sizes on the stack, to understand space consumption
    println!("Point occupies {} bytes on the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack", mem::size_of_val(&rectangle));

    // QUIZ:
    // what's going to be printed next?
    // 8 | 16 | 32
    println!("Boxed point occupies {} bytes on the stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack", mem::size_of_val(&box_in_a_box));

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack", mem::size_of_val(&unboxed_point));
}
// Recursive types
//QUIZ:
// which of the following is a recursive type?
// Array | List | Stream | Pair

// Rust is so cool because its design is rooted on Programming Language Theory research
    // do you know we spend ~ 50% of our time doing research, and not teaching,
    // not dealing with you-related stuff ?
    // I happen to do quite a lot of PL research! check out the Semantics of PL course

// The idea of a recursive type comes from functional programming, mostly
// essentially, it's a type that mentions itself.
// The canonical example is Lists:
//     v-----------------------------------------
//  a list of natural numbers is ... ?          |
//  a number followed by ..  ? a list ! --------|
// see the circular definition?

// Recursive types can be an issue in Rust, because Rust needs to know how much space a type takes up.
// QUIZ: can you know how much space will a Recurisve type take *at compile time* ?
// Y | N

// Fortunately, Box es have a fixed size! so you can implement a recursive type with Box es
// Let's implement a Cons list.
// A cons list is a data structure that comes from the Lisp programming language and its dialects.
// In Lisp, the cons function (short for “construct function”) constructs a new pair from its two arguments,
// which usually are a single value and another pair. These pairs containing pairs form a list.

// The cons function concept has made its way into more general functional programming jargon:
// “to cons x onto y” informally means to construct a new container instance by putting
// the element x at the start of this new container, followed by the container y.
// Each item in a cons list contains two elements:
//      the value of the current item
//      and the next item.
//
// The last item in the list contains only a value called Nil without a next item.
// A cons list is produced by recursively calling the cons function.
// The canonical name to denote the base case of the recursion is Nil.
//
// To implement this data structure, we can first define the following enum.
//
enum List {
    // comment, uncomment
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}
// Then we can create our list as shown below.
use crate::c11::List::{Cons,Nil};
pub fn recursivetypes(){
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

}
// DNC: error[E0072]: recursive type `List` has infinite size

// Boxes provide only the indirection and heap allocation;
// they don’t have any other special capabilities,
// like those we’ll see with the other smart pointer types.
// They also don’t have any performance overhead that these special capabilities incur,
// so they can be useful in cases like the cons list where the indirection is the only feature we need.
//
// Those references that do extra things are called smart pointers
// Smart pointers are data structures that not only act like a pointer
// but also have additional metadata and capabilities.
// The concept of smart pointers isn’t unique to Rust:
//  smart pointers originated in C++ and exist in other languages as well.
// In Rust, the different smart pointers defined in the standard library
// provide functionality beyond that provided by references.
// One example that we’ll explore in this chapter is the reference counting smart pointer type.
// This pointer enables you to have multiple owners of data by keeping track of the number of owners and, when no owners remain, cleaning up the data.



/* ========= Deref =========
   ========================= */
// Implementing the Deref trait allows you to customize the behavior of the dereference operator,
//  * (as opposed to the multiplication).
// By implementing Deref in such a way that a smart pointer can be treated like a regular reference,
// you can write code that operates on references and use that code with smart pointers too.

// let's bring the trait in scope for later
use std::ops::Deref;

// define our own box, for arbitrary stuff of type T,
// we don't give this struct's field a name, we'll access it with   .0
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

pub fn example_smart1() {
    // first, we test on the canonical Box
    let x = 5;
    let y = Box::new(x);

    println!("I expect 5: {}", x);
    println!("I expect 5: {}", *y);
    // Behind the scenes in this example, what is actually run is this.
    //     (x.deref())
    //     *(y.deref())

    let x = 5;
    let y = MyBox::new(x);

    // QUIZ: what does the second println print?
    println!("I expect 5: {}", x);
    // comment the IMPL below
    // DNC: error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
    println!("I expect 5: {}", *y);
    // let's implemen t Deref for Mybox then
    // uncomment the IMPL below, now it work
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
//Without the Deref trait, the compiler can only dereference & references.
// The deref method gives the compiler the ability to take a value
// of any type that implements Deref and call the deref method
// to get a & reference that it knows how to dereference.
// The reason the deref method returns a reference to a value,
// and that the plain dereference outside the parentheses in \*(y.deref()) is still necessary,
// is the ownership system. If the deref method returned the value directly
// instead of a reference to the value, the value would be moved out of self.


/* ========== Drop =========
   ========================= */
//The second trait important to the smart pointer pattern is Drop,
// which lets you customize what happens when a value is about to go out of scope.
// You can provide an implementation for the Drop trait on any type,
// and the code you specify can be used to release resources like files or network connections.
// We’re introducing Drop in the context of smart pointers
// because the functionality of the Drop trait is almost always used when implementing a smart pointer.
// For example, when a Box is dropped it will deallocate the space on the heap that the box points to.
//
// Now we implement a custom smart pointer, which will print when the instance goes out of scope.
//
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
pub fn example_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("End of function");
}
// When running this example, Rust automatically called drop for us
// when our instances went out of scope, calling the code we specified.
// Variables are dropped in the reverse order of their creation,
//      so d was dropped before c.
// This example gives you a visual guide to how the drop method works;
// usually you would specify the cleanup code that your type needs to run rather than a print message.


/* ========== Rc ===========
   ========================= */
// Rc is the reference counted smart pointer.
// In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
// However, there are cases when a single value might have multiple owners.
// For example, in graph data structures, multiple edges might point to the same node,
// and that node is conceptually owned by all of the edges that point to it.
// A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it.
//
// To enable multiple ownership, Rust has a type called Rc,
// which is an abbreviation for reference counting.
// The Rc type keeps track of the number of references to a value
// to determine whether or not the value is still in use.
// If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

pub fn example_rc(){
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // DNC: error[E0382]: use of moved value: `a`
    // let c = Cons(4, Box::new(a));

    // To fix this example, we can change the definition of List to uce Rc in place of Box.
    // take a look at the RcList definition below
    // first, notice that we're calling Rc::new, not just RcCons
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // When we create b, instead of taking ownership of a, we’ll clone the Rc that a is holding,
    // thereby increasing the number of references from one to two and letting
    // a and b share ownership of the data in that Rc.
    // Also, we'll take the address of a
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    // We’ll also clone a when creating c, increasing the number of references from two to three.
    // Every time we call Rc::clone, the reference count to the data within the Rc will increase,
    // and the data won’t be cleaned up unless there are zero references to it.
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // QUIZ: what is the sequence of printed numbers?
    // 1, 2, 3, 4 | 1, 2, 3, 2 | 1, 2, 3 ,3

    // The value gets decremented in the implementation of the Drop trait!
}

use std::rc::Rc;
use crate::c11::RcList::{RcCons,RcNil};

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}


/* ==== Implicit Deref =====
   ========================= */

// Deref coercion is a convenience that Rust performs on arguments to functions and methods.
// Deref coercion works only on types that implement the Deref trait.
// Deref coercion converts such a type into a reference to another type.
//
// For example, deref coercion can convert &String to &str because String implements
// the Deref trait such that it returns &str.
//
//
// Deref coercion happens automatically when we pass a reference
// to a particular type’s value as an argument to a function or method
// that doesn’t match the parameter type in the function or method definition.
// A sequence of calls to the deref method converts the type we provided into the type the parameter needs.
//
// Deref coercion was added to Rust so that programmers writing function
// and method calls don’t need to add as many explicit references and dereferences with & and *.
// The deref coercion feature also lets us write more code that can work for either references or smart pointers.
//
// To see an example, we can use our MyBox type.

// this is an auxiliary function that takes an  &str  argument
fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn implitictderef() {
    let m = MyBox::new(String::from("Rust"));
    // Here we’re calling the hello function with the argument &m,
    // which is a reference to a MyBox value.
    // Because we implemented the Deref trait on MyBox,
    // Rust can turn &MyBox into &String by calling deref.
    // The standard library provides an implementation of Deref on String that returns a string slice,
    // and this is in the API documentation for Deref.
    // Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
    hello(&m);
}

// If Rust didn't implement deref coercion, we would have to write the following code.// ```rust
// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello(&(*m)[..]);
// }

// Similar to how you use the Deref trait to override the * operator on immutable references,
// you can use the DerefMut trait to override the * operator on mutable references.
// Rust does deref coercion when it finds types and trait implementations in three cases:
//  - From `&T` to `&U` when `T: Deref<Target=U>`
//  - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
//  - From `&mut T` to `&U` when `T: Deref<Target=U>`
//
// The first two cases are the same except for mutability.
// The first case states that if you have a &T, and T implements Deref to some type U,
// you can get a &U transparently.
//
// The second case states that the same deref coercion happens for mutable references.
//
// The third case is trickier:
// Rust will also coerce a mutable reference to an immutable one.
// But the reverse is not possible: immutable references will never coerce to mutable references. B
// ecause of the borrowing rules, if you have a mutable reference,
// that mutable reference must be the only reference to that data
// (otherwise, the program wouldn’t compile).

/* ========== Arc ==========
   ========================= */
// Similar to Rc, Arc (atomic reference counted) can be used when sharing data across
//      threads
// This struct, via the Clone implementation can create a reference pointer
// for the location of a value in the memory heap while increasing the reference counter.
// As it shares ownership between threads,
// when the last reference pointer to a value is out of scope, the variable is dropped.

pub fn arc() {
    use std::sync::Arc;
    use std::thread;

    // This variable declaration is where its value is specified.
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a reference in the memory heap.
        let apple = Arc::clone(&apple);

        let tjh = thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
            println!("count after creating apple in a thread: {}", Arc::strong_count(&apple));
            // What's going on? See:
            //      https://doc.rust-lang.org/std/sync/struct.Arc.html#method.strong_count
        });

        tjh.join();
    }
}

/*
    Let's try to understand what is going on with Rc (and Arc)
    After all, they're implemented
        in Rust
    so we should be able to replicate them:
 */
struct NaiveRc<T> {
    reference_count: usize,
    inner_value: T,
}

impl<T: Copy> Clone for NaiveRc<T> {
    fn clone(&self) -> Self {
        // QUIZ: does this code compile?
        // Y | N
        // self.reference_count += 1;
        // DNC: error[E0594]: cannot assign to `self.reference_count`, which is behind a `&` reference
        // The problem is: clone takes an immutable reference to self, so the reference count can’t be mutated!
        return NaiveRc{
            reference_count : self.reference_count,
            inner_value : self.inner_value.clone()
        }
    }
}
impl<T: Copy> NaiveRc<T> {
    //We could implement a special, differently-named cloning function that takes &mut self,
    // but that is awful for usability
    // (because it defies the convention of simply checking if a type implements Clone),
    // and forces the user of our API to always declare mutable instances of that type
    fn clone_mut(&mut self) -> Self{
        self.reference_count += 1;
        return NaiveRc{
            reference_count : self.reference_count,
            inner_value : self.inner_value.clone(),
        }
    }
}
// We also know that the reference counted wrappers in the standard library
// (std::rc::Rc and std::sync::Arc) don’t rely on that solution,
// which suggests there’s another way.
//      Interior mutability
// which is achieved through the RefCell and Cell types

/* ======== RefCell ========
   ========================= */
//Interior mutability is a design pattern in Rust
// that allows you to mutate data even when there are immutable references to that data;
// normally, this action is disallowed by the borrowing rules.
//
// To mutate data, the pattern uses unsafe code inside a data structure
// to bend Rust’s usual rules that govern mutation and borrowing.
//      NOTE: while we're going to use these library types that
//          rely on unsafe Rust, we are not ever going to write unsafe
//          Rust ourselves. For a good reason.
//
// We will explore this concept using the `RefCell<T>` type, which follows the interior mutability pattern.
// Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds.
// What makes `RefCell<T>` different from a type like `Box<T>`?
//
// QUIZ: recall the borrowing rules we learned in previous lectures:
//  - At any given time, you can have either one mutable reference or any number of immutable references.
//  - References must always be valid.
//
// When using `Box<T>`, the borrowing rules’ invariants are enforced at compile time.
// With `RefCell<T>`, these invariants are enforced at
//      runtime
// With Box references, if you break these rules, you’ll get a
//      compiler error
// With `RefCell<T>`, if you break these rules, your program will
//      panic and exit.
//
// Checking borrowing rules at compile time has the advantage of
// catching errors sooner in the development process,
// and there is no impact on runtime performance because all the analysis is completed beforehand.
// For those reasons, checking the borrowing rules at compile time
// is the best choice in the majority of cases.
//
// However, there are other scenarios where one might want
// to take advantage of the additional flexibility afforded by checking
// the borrowing rules at runtime. Because some analyses are impossible,
// if the Rust compiler can’t be sure the code complies with the ownership rules,
// it might reject a correct program; in this way, it is conservative.
// The `RefCell<T>` type is useful when you’re sure your code follows
// the borrowing rules but the compiler is unable to understand and guarantee that.

//When to use each type?
//
//  - `Rc<T>` enables multiple owners of the same data; `
//          Box<T>` and `RefCell<T>` have single owners.
//  - `Box<T>` allows immutable or mutable borrows checked at compile time;
//          `Rc<T>` allows only immutable borrows checked at compile time;
//          `RefCell<T>` allows immutable or mutable borrows checked at runtime.
//  - Because `RefCell<T>` allows mutable borrows checked at runtime,
//          you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

use std::cell::RefCell;

/* == Interior Mutability ==
   ========================= */
//TODO

/* === Reference cycles ====
   ========================= */
//TODO


/* ======== Graphs =========
   ========================= */
// For example, when connecting nodes in a graph.
// You may think to wrap the nodes in Rc or Arc and call it a day.
// That a perfectly reasonable line of though, and it would work
//      if you never, ever needed to mutate nodes.
// Once you try building the graph by incrementally adding and connecting nodes,
// the compiler will give you grief.
// You could call get_mut to receive an Option<&mut T>,
// but that would work only once: get_mut only returns a mutable reference
// as if there is only one “strong” reference to the value... Foiled again!
//
// Fortunately, you can use interior mutability here:
//  use Rc<Cell<T>> or Rc<RefCell<T>>.
// That way you can clone the reference-counted wrapper as much as you want
// and still modify the innermost value wrapped by Cell or RefCell.


// A graph can be represented in several ways. For the sake of illustrating how
// interior mutability works in practice, let's go with the simplest
// representation: a list of nodes.
struct Graph<T> {
    nodes: Vec<Node<T>>,
}
// Each node has an inner value and a list of adjacent nodes it is connected to
// (through a directed edge).
//TODO: what's with NODE and _NODE ?
struct Node<T>(NodeRef<T>);
// The private representation of a node.
struct _Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}
// That list of adjacent nodes cannot be the exclusive owner of those nodes, or
// else each node would have at most one edge to another node and the graph
// couldn't also own these nodes.
// We need to wrap Node with a reference-counted box, such as Rc or Arc. We'll
// go with Rc, because this is a toy example.
type NodeRef<T> = Rc<RefCell<_Node<T>>>;
// However, Rc<T> and Arc<T> enforce memory safety by only giving out shared
// (i.e., immutable) references to the wrapped object, and we need mutability to
// be able to connect nodes together.
// The solution for this problem is wrapping Node in either Cell or RefCell, to
// restore mutability. We're going to use RefCell because Node<T> doesn't
// implement Copy (we don't want to have independent copies of nodes!).

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
// If you ignore the loop that prints out the graph’s information,
// now the user doesn’t know how a Node is implemented.
// This version’s usability can still be improved by implementing
// the std::fmt::Debug trait for Node and Graph, for instance.

// You can play with this example in the Rust Playground:
//          https://play.rust-lang.org/?gist=9ccf40fae2347519fcae7dd42ddf5ed6
// Try changing some things yourself! I find breaking things helps me consolidate new knowledge:
//      Replacing RefCell with Cell
//      Removing RefCell and using Rc<Node<T>>
//      Removing Rc and using RefCell<Node<T>>

/* ========= Cell ==========
   ========================= */
// Finally, let's mention Cell too, which can be also used in place of RefCell for interior mutability
// The most obvious difference between Cell and RefCell is that
//      RefCell makes run-time borrow checks, while Cell does not.
// Cell is quite simple to use:
//      you can read and write a Cell’s inner value by calling get or set on it.
// Since there are no compile-time or run-time checks,
// you do have to be careful to avoid some bugs the borrow checker would stop you from writing,
// such as accidentally overwriting the wrapped value:
use std::cell::Cell;

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
    // oops, we clobbered the work done by foo, now it'll contain 11!
    // TODO explain better what's what
    println!("cell value : {}", cell.get());
}
// In contrast, a RefCell requires you to call borrow or borrow_mut
// (immutable and mutable borrows) before using it, yielding a pointer to the value.
// Its borrow semantics are identical to externally mutable variables:
// you can have either a mutable borrow on the inner value or several immutable borrows,
// so the kind of bug I mentioned earlier is detected in run-time.

// we define our Rc with Cell
struct NaiveRcWithCell<T> {
    inner_value: T,
    references: Cell<usize>,
}
//TODO quiz here?

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
// and implement its cloning, in a way that allows us to increment the references
impl<T: Clone> Clone for NaiveRcWithCell<T> {
    fn clone(&self) -> Self {
        self.references.set(self.references.get() + 1);
        NaiveRcWithCell {
            inner_value: self.inner_value.clone(),
            references: self.references.clone(),
        }
    }
}
// now we use our Cell Rc, and the counts are all correct!
pub fn rcwithcellexample() {
    let wrapped = NaiveRcWithCell::new("Hello!");
    println!("references before cloning: {:?}", wrapped.references());
    let wrapped_clone = wrapped.clone();
    println!("references after cloning: {:?}", wrapped.references());
    println!("clone references: {:?}", wrapped_clone.references());
}
// Calling borrow or borrow_mut on a mutably borrowed RefCell will cause a panic,
// as will calling borrow_mut on a immutably borrowed value.
// This aspect makes RefCell unsuitable to be used in a parallel scenario;
// you should use a thread-safe type (like a Mutex or a RwLock, for example) instead.
//
// A RefCell will stay “locked” until the pointer you received falls out of scope,
// so you might want to declare a new block scope (ie., { ... }) while working with the borrowed value,
// or even explicitly drop the borrowed value when you’re done with it, to avoid unpleasant surprises.
//
// Another significant difference between Cell and RefCell is that
// Cell<T> requires that the inner value T implements Copy,
// while RefCell<T> has no such restriction.
// Often, you won’t want copy semantics on your wrapped types, so you’ll have to use RefCell.
//
// Put succinctly,
//      Cell has Copy semantics and provides values
//      RefCell has move semantics and provides references.