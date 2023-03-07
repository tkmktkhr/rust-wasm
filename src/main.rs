use std::convert::AsRef;
use std::fmt::Debug;

fn main() {
    // dereference
    let mut x: _ = 10; // _ i> i32
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];

    let slice: &[_] = &a[2..]; // _ i> i32
    println!("s: {slice:?}"); // :? is for debug.
    sample();
    type_inference();
    static_constant();
    banner();
    shadowing();
    stack_memory();
    copy_clone();
    life_time_1();
    life_time_2()
}

// Array, Vec, Slice
fn sample() {
    // matrix practice
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(&matrix);
    println!("transposed:");
    pretty_print(&transposed);

    // advanced matrix practice
    pretty_print(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    pretty_print([["a", "b"], ["c", "d"]]);
    pretty_print(vec![vec![1, 2], vec![3, 4]])
}

fn transpose(matrix: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..result[0].len() {
        for j in 0..result[0].len() {
            result[j][i] = matrix[i][j];
        }
    }
    return result;
}

fn pretty_print<T, Line, Matrix>(matrix: Matrix)
where
    T: Debug,
    // A line references a slice of items
    Line: AsRef<[T]>,
    // A matrix references a slice of lines
    Matrix: AsRef<[Line]>,
{
    for row in matrix.as_ref() {
        println!("{:?}", row.as_ref())
    }
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(&matrix);

    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    )
}

fn type_inference() {
    let mut v = Vec::new();
    let abc = "abc";
    let def = "def";
    v.push((String::from(abc), false));
    v.push((String::from(def), true));
    println!(" v: {v:?}");

    // pointer
    println!("pointer1: {:p}", &v[0].0);
    println!("pointer2: {:p}", &v[0].1);
    println!("pointer3: {:p}", &v[0]);
    println!("pointer3: {:p}", &v);

    // let vv = v.iter().collect::<std::collections::HashSet<&(i32, bool)>>(); // don't need to write this.
    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}")
}

const DIGEST_SIZE: usize = 3; // usize is u32 or u64. // inlined upon use.
const ZERO: Option<u8> = Some(42);

fn compute_digest(test_str: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in test_str.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn static_constant() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}

static BANNER: &str = "Welcome to RustOS 3.14"; // not inlined upon use and have an actual associated memory location.

fn banner() {
    println!("{BANNER}");
}

fn shadowing() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}

fn stack_memory() {
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("world");
    // DON'T DO THIS AT HOME! For educational purposes only.
    // String provides no guarantees about its layout, so this could lead to
    // undefined behavior.
    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
    }
    let a = String::from("Hello");
    let b = a.clone();
    println!("{:?}, {:p}", a, &a);
    println!("{:?}, {:p}", b, &b);
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn copy_clone() {
    let x = 42;
    let y = x; // certain types have copy traits.
    println!("x: {x}");
    println!("y: {y}");

    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}, {:p}", &p1);
    println!("p2: {p2:?}, {:p}", &p2);
}

// #[derive(Debug)]
// struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 > p2.0 {
        p1
    } else {
        p2
    }
}

fn life_time_1() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p3: &Point = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);
    println!("pointer: {:p},{:p},{:p}", &p1, &p2, &p3);
}

#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn life_time_2() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // erase(text); // not ok
    println!("{fox:?}");
    println!("{dog:?}");
    erase(text) // ok
}
