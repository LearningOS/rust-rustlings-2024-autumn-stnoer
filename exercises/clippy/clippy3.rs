// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 直接使用 is_some() 进行检查
    if my_option.is_some() {
        // 不再调用 unwrap()，因为我们知道它是 None
    }

    let my_arr = &[
        -1, -2, -3, // 添加缺失的逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 使用 clear() 方法来清空 Vec
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 std::mem::swap 来交换值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
