pub fn qsort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let pvt_idx = partition(arr);
        let arrlen = arr.len();
        qsort(&mut arr[0..(pvt_idx as usize)]);

        // +1 to ignore the middle pivot value.
        // This guarantees, that there will be no such
        // scenario, that produces infinite loop.
        qsort(&mut arr[(pvt_idx as usize + 1)..arrlen]);
    }
    return; //return if given slice size is <= 1.
}
fn partition(arr: &mut [i32]) -> usize {
    let pvt_val = arr[arr.len() - 1];
    let mut i: usize = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= pvt_val {
            arr.swap(i, j);
            i += 1;
        }
    }
    // swap pivot with arr[i] (which is always bigger than pivot)
    arr.swap(i, arr.len() - 1);
    return i;
}
