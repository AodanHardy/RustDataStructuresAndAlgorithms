// Merge sort is a more efficient algorithm. it divides the array into arrays of 1, like a tree
// then start merging them together in order

pub fn merge_sort(array: &mut [i32]) {
    let length = array.len();

    // if the array has 1 or 0 elements, it's already sorted
    if length <= 1 {
        return;
    }

    // find the middle point
    let middle = length / 2;

    // split the array into two halves
    let mut left = array[..middle].to_vec();
    let mut right = array[middle..].to_vec();

    // sort each half
    merge_sort(&mut left);
    merge_sort(&mut right);

    // merge the two halves
    merge(array, &left, &right);
}

fn merge(array: &mut [i32], left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    // compare elements from each side and pick the smallest
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            array[k] = left[i];
            i += 1;
        } else {
            array[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // copy any leftover elements
    while i < left.len() {
        array[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        array[k] = right[j];
        j += 1;
        k += 1;
    }
}