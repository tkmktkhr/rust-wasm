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
    // static_constant();
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
    // let first_vec = v[0].0;
    println!("pointer1: {:p}", &v[0].0);
    println!("pointer2: {:p}", &v[0].1);
    println!("pointer3: {:p}", &v[0]);
    println!("pointer3: {:p}", &v);

    // let vv = v.iter().collect::<std::collections::HashSet<&(i32, bool)>>(); // don't need to write this.
    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}")
}

// const DIGEST_SIZE: usize = 3;
// const ZERO: Option<u8> = Some(42);

// fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
//     let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
//     for (idx, &b) in text.as_bytes().iter().enumerate() {
//         digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
//     }
//     digest
// }

// fn static_constant() {
//     let digest = compute_digest("Hello");
//     println!("Digest: {digest:?}");
// }

// const DIGEST_SIZE: usize = 3;
// const ZERO: Option<u8> = Some(42);

// fn compute_digest(test: &str) -> () {

// }
