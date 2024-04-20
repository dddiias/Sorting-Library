use sorting_library::sorting;

fn main() {
    let mut vec = vec![34, 7, 23, 32, 5, 62];
    println!("Original array: {:?}", vec);
    sorting::quick_sort(&mut vec);
    println!("Sorted with Quick Sort: {:?}", vec);

    let mut vec = vec![34, 7, 23, 32, 5, 62];
    sorting::selection_sort(&mut vec);
    println!("Sorted with Selection Sort: {:?}", vec);

    let mut vec = vec![34, 7, 23, 32, 5, 62];
    sorting::insertion_sort(&mut vec);
    println!("Sorted with Insertion Sort: {:?}", vec);

    let mut vec = vec![34, 7, 23, 32, 5, 62];
    sorting::merge_sort(&mut vec);
    println!("Sorted with Merge Sort: {:?}", vec);
}
