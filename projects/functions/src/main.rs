fn main() {
    another_function(10);

    print_labeled_meaurement(5, 'h');

    expression_function();

    retung_function();

    call_function();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_meaurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn expression_function() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn retung_function() {
    let x = five();

    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn call_function() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}