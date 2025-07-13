use dashmap::DashMap;
use salsa::{Database, Storage};

#[salsa::input]
struct Num{
    num :i32
}

#[salsa::db]
trait Db : salsa::Database{
    fn input(&self,num:i32)-> Num;
}

// #[derive(Clone)]
#[salsa::db]
#[derive(Clone)]
struct MyDb {
    storage: Storage<Self>,
    calculated: DashMap<Num, i32>
}
impl Database for MyDb{
    fn salsa_event(&self, event: &dyn Fn() -> salsa::Event) {
        println!("trigger event:{:?}",event())
    }
}

#[salsa::db]
impl Db for MyDb{
    // 按照 reference 所说， num 这种input 是作为 &mut 进入数据库的，而中间结果比如 tracked 是不可变数据
    // 但这里的input 导致整个数据库 做不到memoized ，因为 它会对重复的 i32 在数据库中创建不一样的Num Id ，导致数据库
    // 认为他们不是同一个 Num ，即便他们i32 是相同的，数据库不会去自动比较这个
    fn input(&self, num:i32)-> Num{
        Num::new(self, num)
    }
}

// you should guarantee that all input types are SalsaStructInDb
// #[salsa::tracked] fn fib(db:&dyn Db,i:i32){}
#[salsa::tracked]
fn fib(db:&dyn Db, i:Num)->Num{
    match i.num(db) {
        0 => {Num::new(db, 1)},
        1 => {Num::new(db, 1)},
        x => { 
            Num::new(db, fib(db, Num::new(db, x-1)).num(db) +
            fib(db, Num::new(db, x-1)).num(db) )
        }
    }
}

fn main(){
    let db = MyDb{
        storage: Storage::default(),
        calculated: DashMap::default(),
    };
    let i = db.input(3);
    let num = fib(&db, i);

    let num = fib(&db, i);
    println!("{:?}", num.num(&db))
}