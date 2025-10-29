mod data_structures;
mod algorithms;
use std::time::Instant;
use data_structures::linked_list::LinkedList;
use data_structures::queue::Queue;
use data_structures::stack::Stack;
use data_structures::binary_tree::BinaryTree;
use data_structures::heap::Heap;

use crate::algorithms::selection_sort::selection_sort;

use rand::Rng;
use crate::algorithms::bubble_sort::bubble_sort;
use crate::algorithms::insertion_sort::insertion_sort;
use crate::algorithms::merge_sort::merge_sort;
use crate::algorithms::quick_sort::quick_sort;

fn main() {
    println!();
    println!("****************************************");;
    println!("**********DATA STRUCTURES***************");
    println!("****************************************");
    println!();

    // Linked List
    println!("********Linked List*********");
    let mut list = LinkedList::new();

    list.append("hello");
    list.append("World");
    list.append("3");
    list.append("4");

    list.insert(2, "Orange");

    list.print();


    // stack
    println!("********Stack*********");
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    stack.print();

    println!("Top: {:?}", stack.peek());

    stack.pop();
    stack.print();


    // queue
    println!("********Queue*********");
    let mut q = Queue::new();

    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    q.print();

    q.dequeue();
    q.print();

    q.enqueue(4);
    q.print();


    // heap
    println!("********HEAP*********");
    let mut h = Heap::new();
    h.insert(5);
    h.insert(3);
    h.insert(8);
    h.insert(1);

    h.print();
    println!("min: {:?}", h.peek());

    h.remove_min();
    h.print();


    println!("********Binary Tree*********");
    let mut tree = BinaryTree::new();
    tree.insert(5);
    tree.insert(2);
    tree.insert(8);
    tree.insert(1);
    tree.insert(3);

    tree.print();
    println!("{}", tree.contains(&3));
    println!("{}", tree.contains(&7));


    println!("****************************************");;
    println!("**********SORT ALGORITHMS***************");
    println!("****************************************");

    const ARRAY_LENGTH: i32 = 10;
    const ARRAY_MIN: i32 = 0;
    const ARRAY_MAX: i32 = 100;


    // selection sort
    println!();
    println!("********Selection Sort*********");
    let mut nums = random_array(ARRAY_LENGTH, ARRAY_MIN, ARRAY_MAX);
    println!("Before: {:?}", nums);
    
    selection_sort(&mut nums);

    println!("After:  {:?}", nums);


    // bubble sort
    println!();
    println!("********Bubble Sort*********");
    let mut nums = random_array(ARRAY_LENGTH, ARRAY_MIN, ARRAY_MAX);
    println!("Before: {:?}", nums);
    
    bubble_sort(&mut nums);

    println!("After:  {:?}", nums);


    // insertion sort
    println!();
    println!("********Insertion Sort*********");
    let mut nums = random_array(ARRAY_LENGTH, ARRAY_MIN, ARRAY_MAX);
    println!("Before: {:?}", nums);

    insertion_sort(&mut nums);
    
    println!("After:  {:?}", nums);


    // Merge sort
    println!();
    println!("********Merge Sort*********");
    let mut nums = random_array(ARRAY_LENGTH, ARRAY_MIN, ARRAY_MAX);
    println!("Before: {:?}", nums);
    
    merge_sort(&mut nums);
    
    println!("After:  {:?}", nums);

    // Quick sort
    println!();
    println!("********Quick Sort*********");
    let mut nums = random_array(ARRAY_LENGTH, ARRAY_MIN, ARRAY_MAX);
    println!("Before: {:?}", nums);

    quick_sort(&mut nums);

    println!("After:  {:?}", nums);



    // Larger Lists to record time
    const LARGE_ARRAY_LENGTH: i32 = 10_000;
    const LARGE_ARRAY_MIN: i32 = 0;
    const LARGE_ARRAY_MAX: i32 = 30_000;

    let mut nums = random_array(LARGE_ARRAY_LENGTH,LARGE_ARRAY_MIN,LARGE_ARRAY_MAX);
    let start = Instant::now();
    bubble_sort(&mut nums);
    let bubble_duration = start.elapsed();

    let mut nums = random_array(LARGE_ARRAY_LENGTH,LARGE_ARRAY_MIN,LARGE_ARRAY_MAX);
    let start = Instant::now();
    selection_sort(&mut nums);
    let selection_duration = start.elapsed();

    let mut nums = random_array(LARGE_ARRAY_LENGTH,LARGE_ARRAY_MIN,LARGE_ARRAY_MAX);
    let start = Instant::now();
    insertion_sort(&mut nums);
    let insertion_duration = start.elapsed();

    let mut nums = random_array(LARGE_ARRAY_LENGTH,LARGE_ARRAY_MIN,LARGE_ARRAY_MAX);
    let start = Instant::now();
    merge_sort(&mut nums);
    let merge_duration = start.elapsed();

    let mut nums = random_array(LARGE_ARRAY_LENGTH,LARGE_ARRAY_MIN,LARGE_ARRAY_MAX);
    let start = Instant::now();
    quick_sort(&mut nums);
    let quick_duration = start.elapsed();

    println!();
    println!("*****************************************");;
    println!("********SORT ALGORITHMS TIMES************");
    println!("*****************************************");
    println!("BUBBLE:               {:?}", bubble_duration);
    println!("SELECTION:            {:?}", selection_duration);
    println!("INSERTION:            {:?}", insertion_duration);
    println!("MERGE:                {:?}", merge_duration);
    println!("QUICK:                {:?}", quick_duration);
    println!("*****************************************");
}

pub fn random_array(len: i32, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut numbers = Vec::new();

    // fill the vector with random numbers
    for _ in 0..len {
        let n = rng.gen_range(min..=max);
        numbers.push(n)
    }

    numbers
}