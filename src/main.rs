fn main() {
    let a: [i32; 2] = [148, 53];
    let r: [i32; 2] = [0, 0];
    let cycles: i32 = 3;

    let result: [i32; 2] = cycle(r, a, cycles);

    println!("Result is {:?}", result);
}

fn add(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
    let result: [i32; 2] = [lhs[0] + rhs[0], lhs[1] + rhs[1]];
    result
}

fn multiply(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
    let result: [i32; 2] = [
        (lhs[0] * rhs[0]) - (lhs[1] * rhs[1]),
        (lhs[0] * rhs[1]) + (lhs[1] * rhs[0]),
    ];
    result
}

fn divide(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
    let result: [i32; 2] = [lhs[0] / rhs[0], lhs[1] / rhs[1]];
    result
}

fn cycle(lhs: [i32; 2], rhs: [i32; 2], cycles: i32) -> [i32; 2] {
    let mut r: [i32; 2] = lhs;
    let mut counter: i32 = cycles;

    while counter >= 1 {
        counter -= 1;
        print!("Current Value {:?}", r);
        r = multiply(r, r);
        r = divide(r, [10, 10]);
        r = add(r, rhs);
    }

    r
}
