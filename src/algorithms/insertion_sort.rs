// start at 1, move back until you find a smaller number, insert in front of it. this creates a
// sorted section on the left, unsorted on right. move through entire list one and it will be sorted

pub fn insertion_sort(array: &mut [i32]) {
    let length = array.len();

    // start from the second element
    for i in 1..length {
        let current = array[i];
        let mut position = i;

        // move larger elements up one place
        while position > 0 && array[position - 1] > current {
            array[position] = array[position - 1];
            position -= 1;
        }

        // insert the current element into its correct spot
        array[position] = current;
    }
}