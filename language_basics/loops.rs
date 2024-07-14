fn main() {
    let mut x = 0;
    loop {
        if x < 5 {
            println!("x: {x}");
            x += 1;
        } else {
            break;
        }
    }

    let mut y = 5;
    while y > 0 {
        println!("y: {y}");
        y -= 1;
    }

    for i in [1, 2, 3, 4, 5] {
        println!("i: {i}");
    }
}
