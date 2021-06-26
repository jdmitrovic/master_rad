fn main() {
    let mut v: Vec<i32> = vec![4, 77, 10, 923];
    v.push(50);
    v.push(33);

    v.remove(2);

    for elem in &mut v {
        *elem *= 10;
        print!("{} ", elem);
    }
}
