pub fn largest_prime_divisor(num: i32) -> i32 {
    if num == 0 || num == 1 || num == 2 {return num}
    let mut cpy_num: i32 = num;
    let mut store_negativity: i32 = 1;
    if cpy_num < 0 {
        cpy_num = cpy_num * -1;
        store_negativity = -1;
    }
    for x in (2..cpy_num).rev() {
        match div(cpy_num, x).1 {
            0 => match is_prime(x) {
                true => return x*store_negativity,
                false => (),
            }
            _ => (),
        }
    }
    return num;
}

// Divides a by b, returns Result<(quotent, remainder), Err>
#[allow(dead_code)]
fn safe_div(a: i32, b: i32) -> Result<(i32, i32), String> {
    match b {
        0 => Err("Division by zero".to_string()),
        _ => Ok(div(a, b)),
    }
}
#[warn(dead_code)]

fn div(val1: i32, val2: i32) -> (i32, i32) {
    let mut a: i32 = val1;
    let mut b: i32 = val2;
    if a < 0 {a = a * -1}
    if b < 0 {b = b * -1}
    let mut remainder: i32 = 0;
    let mut result: i32 = 0;
    while a >= b {
        remainder = a - b;
        result += 1;
        a = a - b;
    }
    return (result, remainder);
}

fn is_prime(num: i32) -> bool {
    if num <= 3 {return true}
    // No need for safe_div, because we know the divisor
    for i in 2..num {
//        println!("div({}, {}) = ({}, {})", num, i, div(num, i).0, div(num, i).1);
        if div(num, i).1 == 0 {
            return false;
        }
    }
    return true;
}
