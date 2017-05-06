
fn try_temporary_mutability() {

    struct Thing {
        x: i32,
        y: i32,
    }

    let mut t = Thing{ x: 0, y: 0};

    t.x = 7;

    let t = t;

    println!("thing {}, {}",t.x,t.y);


}

fn try_ref() {
    let x: &i32 = &12;
    println!("{}",x);
}

fn try_borrow() -> i32 {
    let mut x: i32 = 150;
    {
        let y = &mut x;
        *y = *y + 100;
    }
    x
}

fn main() {
    try_ref();
    try_temporary_mutability();
    let x = try_borrow();
    println!("Hello, {}", x);
}
