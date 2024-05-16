fn main() {
    //   /* Task 1
    reverse_range();
    // */ Task 2
    take_input();
    // Task 3
    arithmetic_ops();
}

// reverse range operator
fn reverse_range() {
    for i in (1..51).rev() {
        println!("{}", i);
    }
}
// Task 2

fn take_input() {
    println!("Enter a integer: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_int: i32 = input.trim().parse().unwrap();
    for i in 0..input_int {
        println!("{}", i);
    }
}

// Task 3


fn arithmetic_ops () {
    println!("Enter a integer: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_int: i32 = input.trim().parse().unwrap();
    println!("Enter another integer: ");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).unwrap();
    let input_int2: i32 = input2.trim().parse().unwrap();
    println!("Addition: {}", input_int + input_int2);
    println!("Subtraction: {}", input_int - input_int2);
    println!("Multiplication: {}", input_int * input_int2);
    println!("Division: {}", input_int / input_int2);
}