// pick a pivot (the closer to the median it is, the more efficient the search)
// break the array into 2, everything smaller than the pivot into one and the other is 
// everything larger. repeat for each array. 

pub fn quick_sort(array: &mut [i32]) {
    let length = array.len();

    if length <= 1 {
        return;
    }

    let pivot_index = partition(array);

    quick_sort(&mut array[0..pivot_index]);
    quick_sort(&mut array[pivot_index + 1..length]);
}

fn partition(array: &mut [i32]) -> usize {
    let length = array.len();

    // choose pivot using median of three
    let first = 0;
    let middle = length / 2;
    let last = length - 1;

    let pivot_index = median_of_three(array, first, middle, last);
    let pivot = array[pivot_index];

    // move pivot to end
    array.swap(pivot_index, last);

    let mut i = 0;

    for j in 0..last {
        if array[j] < pivot {
            let temp = array[i];
            array[i] = array[j];
            array[j] = temp;
            i += 1;
        }
    }

    let temp = array[i];
    array[i] = array[last];
    array[last] = temp;

    i
}

fn median_of_three(array: &mut [i32], a: usize, b: usize, c: usize) -> usize {
    let x = array[a];
    let y = array[b];
    let z = array[c];

    if (x > y) == (x < z) {
        a
    } else if (y > x) == (y < z) {
        b
    } else {
        c
    }
}