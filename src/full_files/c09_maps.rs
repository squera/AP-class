/// Material for this module:
///
///     https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html/
///     https://www.newline.co/@kkostov/the-rust-map-function-a-gateway-to-iterators--f991b22b

// do you recall map from functional programming?
// type of map:  list a -> (a->b) -> list b
//
// in Rust, map works on iterators
//      because they're a general concept and all collections implement them
//      so it's easy to lift the notion from lists to hashmaps etc
pub fn singlemap(){
    let numbers = vec![3, 6, 9, 12];
    let result: Vec<i32> = numbers
        .iter()
        .map(|n| n * 10)
        .collect();
    println!("{:?}",result);
}
// what does map return then? another iterator!
// because we can chain maps!
pub fn twomaps(){
    let numbers = vec![3, 6, 9, 12];
    let result: Vec<i32> = numbers
        .iter()
        .map(|n| n * 10)
        .map(|n| n / 3)
        .collect();
    println!("{:?}",result);
}
// notice the `.collect()`
// it's there to convert the returned iterator into a Vec


// map is LAZY
pub fn lazymap_collect(){
    let numbers = vec![3, 6, 9, 12];
    let mut number_of_times = 0;
    let result: Vec<i32> = numbers
        .iter()
        .map(|n| {
            number_of_times += 1;
            return n * 10
        })
        .collect();
    println!("{:?}",result);
    println!("{}",number_of_times);
}
pub fn lazymap_nocollect(){
    let numbers = vec![3, 6, 9, 12];
    let mut number_of_times = 0;
    // notice the change in type here
    let result = numbers
        .iter()
        .map(|n| {
            number_of_times += 1;
            return n * 10
        });
    println!("{:?}",result);
    println!("{}",number_of_times);
}
// QUIZ: what will this print?

pub fn string_tolower(){
    let words: Vec<&str> = vec!["Hello", "from", "Rust", "!"];
    println!("Words before map: {:?}", words);
    let lowercased_words: Vec<String> = words // using String as the return type of `to_lowercase`
        .iter()
        .map(|word| word.to_lowercase())
        .collect();
    println!("Words after map: {:?}", lowercased_words);
}

pub fn maps_options(){
    let str_numbers: Vec<&str> = vec!["1", "2", "3", "I am not a number"];
    let numbers: Vec<u32> = str_numbers
        .iter()
        // does not compile
        // .map(|str_number| str_number.parse::<u32>())
        // must use `flat_map`
        //  https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
        .flat_map(|str_number| str_number.parse::<u32>())
        .collect();
    println!("{:?}",numbers);
}

/// https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html/
pub fn mapsownership(){
    let s1 = String::from("asd1");
    let s2 = String::from("asd2");
    let s3 = String::from("asd3");
    let s4 = String::from("asd4");
    let mut v : Vec<String> = Vec::new();
    v.push(s1); v.push(s2); v.push(s3); v.push(s4);
    let newv : Vec<String> = v
        .iter()
        .map(|x| x.replace('a', "aa"))
        .collect();
    println!("v: {:?}",v);
    println!("v: {:?}",newv);
    // using 'replace' returns a new string! no need for ownership transfer
    // so many similar examples are not informative

    let n1 = Node::new(10);
    let n2 = Node::new(20);
    let n23 = Node::new(23);
    let n4 = Node::new(40);
    let mut vv = vec![n1,n2,n4];
    vv.push(n23);
    // pushing moves ownership: vv now owns the nodes
    let mut v2 : Vec<Node> = Vec::new();
    println!("vv {:?}",vv);
    let r : Vec<()> = vv
        .iter_mut()
        .map(|mut el| el.inc_content())
        .collect();
    println!("vv {:?}",vv);
    println!("r {:?}",r);
    println!("v2 {:?}",v2);
    // but iter and iter_mut are still borrowing data, not owning it
    // in fact, here we can still print  vv

    let rr : Vec<()> = vv
        // .iter()  //cannot move
        .into_iter()   // acquires ownership
        .map(|el| v2.push(el))
        .collect();
    // println!("vv {:?}",vv);     // passed ownership of vv away
    println!("v2 {:?}",v2);
    println!("rr {:?}",rr);
    // to change ownership of data, use into_iter
    // see that we cannot print vv afterwards, vv is not the owner!
}

#[derive(Debug)]
pub struct Node {
    content: i32,
}

impl Node {
    pub fn new(content: i32) -> Node {
        Node { content }
    }
    fn eq_content(&self, o: &Node) -> bool {
        self.content == o.content
    }
    pub fn eq(&self, o: &Node) -> bool {
        self.eq_content(o)
    }
    pub fn inc_content(&mut self){self.content+=1;}
}