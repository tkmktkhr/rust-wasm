pub mod array_for_loops {
  #![allow(unused_variables, dead_code)]

  fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..matrix.len() {
      for j in 0..matrix.len() {
        new[i][j] = matrix[j][i]
      }
    }
    new
  }

  fn pretty_print(matrix: &[[i32; 3]; 3]) {
    // unimplemented!()
    for i in matrix.iter() {
      println!("[ {}, {}, {}]", i[0], i[1], i[2]);
    }
  }

  pub fn output() {
    let matrix = [
      [101, 102, 103], // <-- the comment makes rustfmt add a newline
      [201, 202, 203],
      [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
  }
}
