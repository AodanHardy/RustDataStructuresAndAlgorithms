// the simplest sort algorithm. find the smallest and move to front, move over 1 index and repeat

pub fn selection_sort(array: &mut [i32]) {
    let len = array.len();

    // go through each element
    for i in 0..len {
        let mut min_index = i;

        // find the smallest element in the rest of the array
        for j in (i + 1)..len {
            if array[j] < array[min_index] {
                min_index = j
            }
        }

        // swap it with the current position
        if min_index != i {
            array.swap(i, min_index)
        }
    }
}