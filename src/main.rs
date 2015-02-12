#![feature(rustc_private)]

extern crate syntax;

mod pretty;

#[derive(Debug)]
struct Blah {
    x: Vec<i32>
}

fn main() {
    use pretty::Pretty;

    let _ = Blah {
        x: vec![1, 2, 3, 4, 5]
    };
    let pretty = Pretty::new(vec![1, 2, 3, 4, 5]);
    println!("{:?}", pretty);
}
