use dashmap::DashMap;
use dashmap::parking_lot_core;
use std::hash::RandomState;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

// struct B<T:M>{
//     a:i32,
//     p:PhantomData<T>
// }

// struct ImplTest<T>{
//     A : dyn M<>
// }

// enum K{
//     K1 {i:i32, B:Box<i32>},
//     K2 {i:i32, B:Box<u64>}
// }
fn main() {
    check_deadlock();
    let mut d: DashMap<i32, i32, RandomState> = DashMap::default();
    d.insert(3, 4);
    let m = d.get_mut(&3);
    let m2 = d.get_mut(&3);
}

fn check_deadlock() {
    let a = thread::spawn(|| {
        loop {
            sleep(Duration::new(1, 0));
            println!("checked");
            let mut v = parking_lot_core::deadlock::check_deadlock();
            for i in v {
                for m in i {
                    panic!("m dead")
                }
            }
        }
    });
}
