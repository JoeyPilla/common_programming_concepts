fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    another_function3(5, 7);
    conditional();

}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}, the value of y is {}.", x, y);
}

fn five () -> i32 {
    5
}

fn conditional() {
    let number = 3;
    
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}
