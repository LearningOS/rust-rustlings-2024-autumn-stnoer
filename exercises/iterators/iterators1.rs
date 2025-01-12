// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // 获取迭代器

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));   // Step 1
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));  // Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));  // Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));  // Step 4
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));  // Step 5
    assert_eq!(my_iterable_fav_fruits.next(), None);  // 迭代完成，没有更多元素
}

