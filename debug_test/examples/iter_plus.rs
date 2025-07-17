fn main() {
    for i in 0..1000000 {
        ["Alice", "Mike", "Michael", "Dave"]
            .iter()
            .map(|x| x.to_string())
            .filter(|x| x.starts_with('M'))
            .map(|x| x.replacen("M", "F", 1))
            .for_each(|x| println!("{}", x));
    }
}
