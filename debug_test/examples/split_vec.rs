fn main() {
    let a = vec![1, 2, 3];
    // let b = vec![1,2,3];
    let b = vec!["a", "b", "c"];
    let (a, b): (Vec<i32>, Vec<_>) = a.iter().zip(b).map(|(x, y)| (x, y)).collect();
}
