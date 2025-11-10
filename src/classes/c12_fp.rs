///     https://doc.rust-lang.org/book/ch13-00-functional-features.html

/* ======= Closures ========
   ========================= */
pub mod closures{
    fn function(i: i32) -> i32 { i + 1 }

    pub fn closuresexample() {
        let closure_annotated = |i: i32| -> i32 { i + 1 };
        let closure_inferred = |i| i + 1;
        let i = 1;
        println!("function: {}", function(i));
        println!("closure_annotated: {}", closure_annotated(i));
        println!("closure_inferred: {:?}", closure_inferred(i));

        let one = || -> i32 { 1 };
        println!("closure returning one: {}", one());
        // QUIZ: this makes the language:

        let closure = |x : i32| -> i32 { println!("I'm a closure with {}! ", x);x };

        call_me(closure);
        call_me(function);
    }

    fn call_me<F: Fn(i32) -> i32>(f: F) {
        f(1);
    }

    pub fn capturingexample(){
        use std::mem;

        let color = String::from("green");
        let print2 = |color:String| {println!("{}",color);};
        // < fn print2, [] >
        let print23 = || {
            let color = "wghat".to_string();
            println!("{}",color);
            // return || {return 0;}
        };
        // < fn print23, [] >

        let print = || { println!("`color`: {}", color) };
        // < fn print, [ color -> "green" ] >
        print();

        // QUIZ: can we do this:
        let _reborrow = &color;
        print();
        let _color_moved = color;

        let mut count = 0;
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };
        // let mut inc2 = || {
        //     count +=1;
        // };
        // inc2();
        inc();

        // QUIZ: can we do this, given the inc() below?
        // let _reborrow = &count;
        inc();
        let _count_reborrowed = &mut count;

        let mut movable = Box::new(3);
        let consume = || {
            println!("`movable`: {:?}", &movable);
            *movable = 5;
            mem::drop(movable);
        };
        consume();
        // consume();
        // The error is interesting:
        // 153 |         consume();
        //     |         ^^^^^^^
        //      note: this value implements `FnOnce`, which causes it to be moved when called
    }


    fn apply_FnOnce<F>(f: F) where F: FnOnce() {
        f();
    }
    fn apply_Fn<F>(f: F) where F: Fn() { // Note: The type of F has changed
        f();
    }
    fn apply_FnMut<F>(mut f: F) where F: FnMut() {
        // Note: The type of F has changed
        // and we needed to add the `mut` to F
        f();
    }

    fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
        f(3)
    }
    fn applytest<F>(f:F) -> i32 where F:FnOnce(i32) -> i32 {
        3
    }

    pub fn fntypes(){
        use std::mem;

        let greeting = "hello";
        let mut farewell = "goodbye".to_owned();

        let mut diary = || {
            println!("I said {}.", greeting);
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");
            mem::drop(farewell);
        };
        // QUIZ: which one compiles?
        // apply_FnOnce(diary);
        // apply_FnMut(&mut diary);
        // apply_Fn(&mut diary);
        let _reb = &greeting;

        // QUIZ: what do i need to comment in the code of `diary` to make this work
        // apply_FnMut(diary);

        let double = |x| 2 * x;
        println!("3 doubled: {}", apply_to_3(double));
    }

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        return move || {
            println!("This is a: {}", text);
        };
    }
    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a: {}", text)
    }
    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fn_para<'a,'b : 'a>(text : &'a String, t2 : &'b String) -> Box<dyn Fn() + 'a> {
        return Box::new( move || {
            println!("This is a: {}, {}", & text, & t2);
        });
    }

    pub fn closures_output(){
        let fn_plain = create_fn();
        let mut fn_mut = create_fnmut();
        let fn_once = create_fnonce();

        fn_plain();
        fn_plain();
        fn_mut();
        fn_once();
        // Q: can i?
        // fn_once();
    }

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    pub fn fprules() {
        println!("Find the sum of all the squared odd numbers under 1000");
        let upper = 1000;

        let mut acc = 0;
        for n in 0.. {
            let n_squared = n * n;

            if n_squared >= upper {
                break;
            } else if is_odd(n_squared) {
                acc += n_squared;
            }
        }
        println!("imperative style: {}", acc);

        let sum_of_squared_odd_numbers: u32 =
            (0..).
                map(|n| n * n)
                .take_while(|&n_squared| n_squared < upper)
                .filter(|&n_squared| is_odd(n_squared))
                .fold(0, |acc, n_squared| acc + n_squared);
        println!("functional style: {}", sum_of_squared_odd_numbers);
    }

}

/* ======= Iterators =======
   ========================= */

pub mod iterators{

    pub fn iteratorexample(){
        let mut v1 = vec![1, 2, 3];
        let v1_iter = v1.iter_mut();

        for val in v1_iter {
            println!("Got: {}", val);
        }
        // pub trait Iterator {
        //     type Item;
        //
        //     fn next(&mut self) -> Option<Self::Item>;
        //     // methods with default implementations elided
        // }
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        // QUIZ: what does v1_iter.next() return?

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        // let v2 = v1_iter.next();
        // QUIZ: what will be the first parameter of sum ?
        assert_eq!(total, 6);

        let v1: Vec<i32> = vec![1, 2, 3];
        let v3 = v1.iter().map(|x| { x + 1 });
        let v2: Vec<_> = v1.iter().map(|x| { x + 1 }).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }


    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        let f = |s : &Shoe| { s.size == shoe_size };
        shoes.into_iter().filter(f).collect()
    }

    pub fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker"), },
            Shoe { size: 13, style: String::from("sandal"), },
            Shoe { size: 10, style: String::from("boot"), },
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        println!("filtered vec: {:?}", in_my_size );
    }


    struct Counter {
        count: u32,
        upperbound:u32
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0, upperbound:10 }
        }
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.upperbound {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    pub fn calling_next_directly() {
        let mut counter = Counter::new();
        // QUIZ: what will counter.next be?

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
    pub fn using_other_iterator_trait_methods() {
        // QUIZ: group up- read up
        //      skip , zip , map , filter , sum
        // explain what each thing does to its input and thus their output
        let sum : Vec<u32>  = Counter::new()
            /* zip is an iterator that iterates over 2 other iterators, in this case
                  Counter::new   and  Counter::new().skip
             */
            .zip(Counter::new().skip(1))
            /* multiply each element of the 2 iterators
            0*1 | 1*2 | 2*3 | 3*4 | 4 * 5
            0 | 2 | 6 | 12 | 20
             */
            .map(|(a, b)| a * b)
            /*keep only those numbers in that can be divided by 3
             */
            .filter(|x| x % 3 == 0)
            /*add all the numbers
             */
            .collect()
            ;
        // println!("{}",sum);
    }


    pub fn examplefpiterators() {
        let vector = [1, 2, 3];
        let result = vector.iter().map(|x| { x * 2 }).collect::<Vec<i32>>();
        println!("After mapping: {:?}", result);

        let su: u32 = vec![1, 2, 3, 4, 5, 6].iter().sum();
        let sum: u32 = vec![1, 2, 3, 4, 5, 6].iter().fold(0, |mut summ, &val| {
            summ += val;
            summ
        });
        let a = [1, 2, 3];
        let doubled: Vec<i32> = a.iter()
            .map(|&x| x * 2)
            .collect();

        assert_eq!(vec![2, 4, 6], doubled);
        // A longer list of these functions can be found
        // [here](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
    }

}