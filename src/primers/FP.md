FP: https://www.cs.kent.ac.uk/people/staff/dat/miranda/whyfp90.pdf

Programming is about
    ABSTRACTIONS 
-> registers?
-> jumps/gotos?

FP key abstractions:
- immutability: known
  - no assignment (no rebinding)
  - no side-effects
- modularity // divide et impera
  - algebraic data types
  - functions
  - higher-order
- pattern-matching: known

FP benefits:
- small code
  - verifiable
- reasonable in isolation
- reusable

Algebraic data types:  
- Composite types
- defined inductively (base + inductive)  
  - ex: list, pairs


    Tree ::  Node | Tree + Tree

Lists in ML:  

    []              // nil
    | (h :: t )     // cons head tail

So:
  
    (2::4::[]) :: (1::3::5::[]) :: []

Dealing with ADTs: induction --> recursion

FP typically provides datatypes (disjoint unions)
Example (Binary Trees)
    
    datatype ’a bintr = LEAF of ’a
    | NODE of ’a bintr*’a bintr;
  
    datatype ’a bintr = LEAF of ’a
    | NODE of ’a bintr * ’a bintr
  
They can have any number of variants 
    - does the name suggest anything?

Example creating a new tree:
  
    val tree = NODE (NODE(LEAF 1,LEAF 4),LEAF 7);
    val tree = NODE(NODE(LEAF 1,LEAF 4),LEAF 7) : int bintDavid Toman (University of Waterloo) Standard ML 15 / 21
  
Functions on trees: use pattern matching


Functions:
- define them : known
- call them : known
- pass them (future classes in rust)
- compose them:
  - q:  given `f : c → a` and `g : b → c`, what is the type of `f g` ?
- typed, polymorphic,
  - what kinds of polymorphism exist?
- higher-order
  - what does this mean? 

examples:
    
    fun hd []     = raise Empty
    | hd (h::t)   = h;

    fun last []     = raise Empty
    | last [x]      = x
    | last (h::t)   = last t;

    fun length []     = 0
    | length (h::t) = 1 + length t;

Tail Recursion!  
- often achieved via accumulators
- optimisable code


    fun length l = length_acc l 0

    fun lenght_acc [] a     = a
    | length_Acc (h ::t ) a = length_acc t, a+1

    lenght_acc (1 ::2 :: [], 0) =
    lenght_acc ( 2::[], 1) =
    lenght_acc ( [], 2) =  2

    length ( 1::2::[]) =
    1 + length ( 2::[]) 
    1 + 1+ length([])
    1 + 1 + 0

Higher-order (+ polymorphism):

    fun map f []   = []
    | map f (h::t) = (f h)::(map f t);

What is its type?

    val map = fn : (’a -> ’b) -> (’a list -> ’b list)

Example:

    map (fn x=> x+1) 1::2::3::[];

Other examples: foldr

    ∀'a, 'b. 'a list -> b -> (a -> b -> b) -> b