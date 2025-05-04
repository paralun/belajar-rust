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
    
    let (var1, var2) = (24, "Isi var 2");
    println!("Var 1, {0}", var1);
    println!("Var 2, {0}", var2);
}

#[test]
fn number_test() {
    let x: i8 = 10;
    println!("{}", x);

    let y: f32 = 10.5;
    println!("{}", y);

    let z = 70;
    println!("{}", z);
}

// Defaut sebuah variable bersifat Immutable yaitu tidak bisa di ubah
// Variable untuk bisa di ubah harus di jadikan menjadi Mutable dengan keyword `mut`
#[test]
fn mutable_test() {
    let mut message = "Bambang";
    println!("Hallo, {}", message);

    message = "Irwanto";
    println!("Hallo, {}", message);
}