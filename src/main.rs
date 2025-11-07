fn main() {
    let p: [i64; 2] = [-79745, 15616];
    let r: [i64; 2] = [0, 0];
    let grid_aperture: i64 = 1;
    let cycles: i64 = 100;

    let mut counter = 0;

    for x in 0..1001 {
        for y in 0..1001 {
            if cycle_check(
                r,
                [p[0] + (grid_aperture * x), p[1] + (grid_aperture * y)],
                cycles,
            ) {
                counter += 1;
            }
        }
    }

    println!("Result is {:?}", counter);
}

fn add(lhs: [i64; 2], rhs: [i64; 2]) -> [i64; 2] {
    [lhs[0] + rhs[0], lhs[1] + rhs[1]]
}

fn multiply(lhs: [i64; 2], rhs: [i64; 2]) -> [i64; 2] {
    [
        (lhs[0] * rhs[0]) - (lhs[1] * rhs[1]),
        (lhs[0] * rhs[1]) + (lhs[1] * rhs[0]),
    ]
}

fn divide(lhs: [i64; 2], rhs: [i64; 2]) -> [i64; 2] {
    [lhs[0] / rhs[0], lhs[1] / rhs[1]]
}

fn cycle_check(mut r: [i64; 2], p: [i64; 2], mut cycles: i64) -> bool {
    loop {
        if cycles < 1 {
            return true;
        }
        cycles -= 1;
        r = multiply(r, r);
        r = divide(r, [100000, 100000]);
        r = add(r, p);

        if r[0] > 1000000 || r[1] > 1000000 {
            return false;
        }
        if r[0] < -1000000 || r[1] < -1000000 {
            return false;
        }
    }
}
