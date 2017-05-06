fn try_borrow() -> i32 {
    let mut x: i32 = 150;
    {
        let y = &mut x;
        *y = *y + 100;
    }
    x
}

fn main() {
    let x = try_borrow();
    println!("Hello, {}", x);
}
