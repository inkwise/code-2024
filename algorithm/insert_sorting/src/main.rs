fn main() {
    println!("Hello, world!");
    let mut arr=vec![4,8,4,1,4,7,2,6,3];
    insertion_sort(&mut arr);
}
//插入排序
fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
    println!("{:?}",arr);
}
