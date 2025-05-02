fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, test!")
}

#[test]
fn variable_test() {
    let name = "Bambang Irwanto";
    println!("Hello, {}", name);
}

#[test]
fn number_test() {
    let x: i8 = 10;
    println!("{}", x);

    let y: f32 = 10.5;
    println!("{}", y)
}