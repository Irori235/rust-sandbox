use proconio::{input, source::once::OnceSource};

fn main() {
    let source = OnceSource::from("3\n1 2 3");

    input! {
        from source,
        n: u8,
        a: [i32;n]
    }

    println!("n: {}, a: {:?}", n, a)
}
