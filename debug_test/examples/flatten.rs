fn main(){
    let v = vec![Some("a"),None];
    let flattened = v.iter().flatten().collect::<Vec<_>>();
    assert_eq!(flattened.len() == 1,true);
    println!("{:?}",flattened)
}