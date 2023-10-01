mod qsort;
mod lpdiv;
use qsort::qsort;
use lpdiv::largest_prime_divisor;

fn main() {
    let mut unsorted: Vec<i32> = vec![43, 68, 25, 48, 69, 420, -10, 0, 0, -1080];
    qsort(unsorted.as_mut_slice());
    println!("{:?}", unsorted);
    for item in unsorted.iter() {
        // largest for numbers above 0, smallest for numbers below
        println!("for {} {} prime divisor is {}", item, (match *item >= 0 {true => "largest", false => "smallest"}), largest_prime_divisor(*item));
    }
}


// ROT13 spoiler
// Guvf pbqr vf nofbyhgryl ergneqrq naq fubhyq abg
// or gnxra frevbhfyl.
