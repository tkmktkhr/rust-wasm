fn main() {
    // dereference
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];

    let slice: &[i32] = &a[2..4];
    println!("s: {slice:?}");
    sample();
}

fn sample() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}
