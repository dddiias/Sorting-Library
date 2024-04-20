pub mod sorting {

    //Quick Sort
    pub fn quick_sort<T: Ord>(items: &mut [T]) {
        if items.len() <= 1 {
            return;
        }
        let pivot_index = partition(items);
        quick_sort(&mut items[0..pivot_index]);
        quick_sort(&mut items[pivot_index + 1..]);
    }

    fn partition<T: Ord>(items: &mut [T]) -> usize {
        let pivot_index = items.len() - 1;
        let mut i = 0;
        for j in 0..pivot_index {
            if items[j] <= items[pivot_index] {
                items.swap(i, j);
                i += 1;
            }
        }
        items.swap(i, pivot_index);
        i
    }

    //Selection Sort
    pub fn selection_sort<T: Ord>(items: &mut [T]) {
        let len = items.len();
        for i in 0..len {
            let mut min_index = i;
            for j in i + 1..len {
                if items[j] < items[min_index] {
                    min_index = j;
                }
            }
            items.swap(min_index, i);
        }
    }

    //Insertion Sort
    pub fn insertion_sort<T: Ord>(items: &mut [T]) {
        let len = items.len();
        for i in 1..len {
            let mut j = i;
            while j > 0 && items[j - 1] > items[j] {
                items.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    //Merge Sort
    pub fn merge_sort<T: Ord + Clone>(items: &mut [T]) {
        let len = items.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        merge_sort(&mut items[0..mid]);
        merge_sort(&mut items[mid..]);

        let mut result = items.to_vec();
        merge(&items[0..mid], &items[mid..], &mut result);
        items.clone_from_slice(&result);
    }

    fn merge<T: Ord + Clone>(left: &[T], right: &[T], result: &mut [T]) {
        let mut left_idx = 0;
        let mut right_idx = 0;
        let mut result_idx = 0;

        while left_idx < left.len() && right_idx < right.len() {
            if left[left_idx] <= right[right_idx] {
                result[result_idx] = left[left_idx].clone();
                left_idx += 1;
            } else {
                result[result_idx] = right[right_idx].clone();
                right_idx += 1;
            }
            result_idx += 1;
        }

        if left_idx < left.len() {
            result[result_idx..].clone_from_slice(&left[left_idx..]);
        }
        if right_idx < right.len() {
            result[result_idx..].clone_from_slice(&right[right_idx..]);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::sorting::*;

    // Quick Sort tests
    #[test]
    fn quick_sort_empty() {
        let mut vec: Vec<i32> = vec![];
        quick_sort(&mut vec);
        assert_eq!(vec, []);
    }

    #[test]
    fn quick_sort_single_element() {
        let mut vec = vec![1];
        quick_sort(&mut vec);
        assert_eq!(vec, [1]);
    }

    #[test]
    fn quick_sort_multiple_elements() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quick_sort(&mut vec);
        assert_eq!(vec, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    // Selection Sort tests
    #[test]
    fn selection_sort_simple() {
        let mut vec = vec![3, 1, 2];
        selection_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3]);
    }

    #[test]
    fn selection_sort_negatives() {
        let mut vec = vec![-1, -3, 2];
        selection_sort(&mut vec);
        assert_eq!(vec, [-3, -1, 2]);
    }

    // Insertion Sort tests
    #[test]
    fn insertion_sort_reversed() {
        let mut vec = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn insertion_sort_already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    // Merge Sort tests
    #[test]
    fn merge_sort_empty() {
        let mut vec: Vec<i32> = vec![];
        merge_sort(&mut vec);
        assert_eq!(vec, []);
    }

    #[test]
    fn merge_sort_single_element() {
        let mut vec = vec![1];
        merge_sort(&mut vec);
        assert_eq!(vec, [1]);
    }

    #[test]
    fn merge_sort_complex() {
        let mut vec = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
        merge_sort(&mut vec);
        assert_eq!(vec, [2, 3, 5, 6, 7, 9, 10, 11, 12, 14]);
    }
}