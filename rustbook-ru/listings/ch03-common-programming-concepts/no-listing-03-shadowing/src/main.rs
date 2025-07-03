fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The значение of x in the inner scope is: {x}");
    }

    println!("Значение переменной x: {x}");
}
