use hashbrown::HashTable;

fn main() {
    let mut table = HashTable::new();

    table.entry(0, eq, hasher)
    table.insert(42, |&x| x); 

    if table.contains(&42, |&x| x) {
        println!("Found 42!");
    }

    if let Some(&value) = table.get(&42, |&x| x) {
        println!("Value: {}", value); 
    }

    table.remove_entry(&42, |&x| x);
}