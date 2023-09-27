fn main() {
    let mut unsorted: Vec<i32> = vec![43, 68, 25, 48, 69, 420, -10, 0, 0, -1080];
    qsort(unsorted.as_mut_slice());
    println!("{:?}", unsorted);
}

fn qsort(arr: &mut [i32]) {
    if (arr.len() > 1) {
        let pvt_idx = partition(arr);
        // We use +1 on line 19 to ignore the middle pivot value.
        // This guarantees, that there will be no such
        // scenario, that produces infinite loop.
        let arrlen = arr.len();
        qsort(&mut arr[0..(pvt_idx as usize)]);
        qsort(&mut arr[(pvt_idx as usize + 1)..arrlen]);
    }
    return; //return if given slice size is >=0.
}
fn partition(arr: &mut [i32]) -> usize {
    let pvt_val = arr[arr.len() - 1];
    let mut i: usize = 0;
    for j in 0..arr.len() - 1 {
        if (arr[j] <= pvt_val) {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1); // swap pivot with arr[i] (which is always bigger than pivot)
    return i;
}

// ROT13 spoiler
// Guvf pbqr vf nofbyhgryl ergneqrq naq fubhyq abg
// or gnxra frevbhfyl.
