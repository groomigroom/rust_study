fn groomi(x: i32) {
    println!("groomi is cute in: {}", x);
}

fn main() {
    groomi(12);
    println!("Hello, World!");
}

/*
groomi is cute in: 12
Hello, World!
*/


------------------------------------------------------

fn groomi(x: i32, groomi_char: char) {
    println!("groomi is cute in: {}{}", x, groomi_char);
}

fn main() {
    groomi(12, 's');
    println!("Hello, World!");
}

/*
groomi is cute in: 12s
Hello, World!
*/
