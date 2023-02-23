fn main() {
    // dereference
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];

    let slice: &[i32] = &a[2..];
    println!("s: {slice:?}"); // :? is for debug.
    sample();
}

fn sample() {
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
}

fn transpose(matrix: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
  let mut result = [[0; 3]; 3];
  for i in 0..result[0].len() {
    for j in 0..result[0].len() {
      result[j][i] = matrix[i][j];
    }
  }
  return result
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
      println!("{row:?}")
    }
    // let array = [10, 20, 30];
    // print!("Iterating over array:");
    // for n in array {
    //     print!(" {n}");
    // }
    // println!();

    // print!("Iterating over range:");
    // for i in 0..array.len() {
    //     print!(" {}", array[i])
    // }
    // println!();
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