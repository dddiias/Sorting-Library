# Sorting Library

This Rust library provides various sorting algorithms implemented to sort arrays of any type that implements the `Ord` trait.

## Usage

Include this library in your Cargo project by adding:

```toml
[dependencies]
sorting_library = { git = "https://github.com/dddiias/sorting_library" }
```

## Screenshots

![Creating](https://github.com/dddiias/sorting_library/tree/main/img/creating.png)
![Compilation](https://github.com/dddiias/sorting_library/tree/main/img/compilation.png)
![Tests](https://github.com/dddiias/sorting_library/tree/main/img/tests.png)
![Main_example](https://github.com/dddiias/sorting_library/tree/main/img/main_example.png)

## Examples

Below are examples demonstrating how to use the sorting algorithms included in this library.

### Quick Sort

```rust
let mut vec = vec![34, 7, 23, 32, 5, 62];
sorting_library::sorting::quick_sort(&mut vec);
assert_eq!(vec, [5, 7, 23, 32, 34, 62]);
```

### Selection Sort

```rust
let mut items = vec!["orange", "apple", "banana"];
sorting_library::sorting::selection_sort(&mut items);
assert_eq!(items, ["apple", "banana", "orange"]);
```

### Insertion Sort

```rust
let mut values = vec![3, 1, 4, 1, 5, 9, 2];
sorting_library::sorting::insertion_sort(&mut values);
assert_eq!(values, [1, 1, 2, 3, 4, 5, 9]);
```

### Merge Sort

```rust
let mut numbers = vec![38, 27, 43, 3, 9, 82, 10];
sorting_library::sorting::merge_sort(&mut numbers);
assert_eq!(numbers, [3, 9, 10, 27, 38, 43, 82]);
```

## License

Sorting Library is released under the MIT License.
