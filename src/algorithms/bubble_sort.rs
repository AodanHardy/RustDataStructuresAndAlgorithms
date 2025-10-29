pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();

    for _ in 0..len {
        let mut swapped = false;

        for j in 0..len - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}