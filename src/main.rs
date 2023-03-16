fn main() {
    let mut test_arr = vec![3, 1, 7, 5, 09, 101, 100];
    println!("need sort array : {:?}", test_arr);
    bubble_sort(&mut test_arr);
    println!("bubble sort result : {:?}", test_arr);

    let mut test_arr_g = vec!["rust", "java", "javaScirpt", "python", "09", "ruby", "go"];
    println!("need genric sort array : {:?}", test_arr_g);
    genric_bubble_sort(&mut test_arr_g);
    println!("genric bubble sort result : {:?}", test_arr_g);
}

fn bubble_sort(arr: &mut Vec<i32>) {
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

fn genric_bubble_sort<T: PartialOrd>(arr: &mut Vec<T>) {
    let len = arr.len();
    for i in 0..(len - 1) {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
