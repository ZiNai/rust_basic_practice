pub fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for i in 0..(len - 1) {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                let t = arr[j + 1];
                arr[j + 1] = arr[j];
                arr[j] = t;
            }
        }
    }
}

pub fn genric_bubble_sort<T: PartialOrd>(arr: &mut Vec<T>) {
    let len = arr.len();
    for i in 0..(len - 1) {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[test]
// #[should_panic]
fn test_bubble_sort() {
    let mut test_arr = vec![3, 1, 7, 5, 09, 101, 100];
    println!("Need sort arr : {:?}", test_arr);
    bubble_sort(&mut test_arr);
    println!("Sort result : {:?}", test_arr);
    assert_eq!(test_arr, vec![1, 3, 5, 7, 9, 100, 101]);
    // assert_eq!(test_arr, vec![09, 1, 3, 5, 7, 101, 100]);
}

#[test]
fn test_genric_bubble_sort() {
    let mut test_arr_g = vec!["rust", "java", "javaScirpt", "python", "09", "ruby", "go"];
    println!("Need sort arr : {:?}", test_arr_g);
    genric_bubble_sort(&mut test_arr_g);
    println!("Sort result : {:?}", test_arr_g);
    assert_eq!(
        test_arr_g,
        vec!["09", "go", "java", "javaScirpt", "python", "ruby", "rust"]
    );
}
