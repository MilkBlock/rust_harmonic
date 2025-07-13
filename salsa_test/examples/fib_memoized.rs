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
    calculated: DashMap<i32, Num>
}
impl Database for MyDb{
    fn salsa_event(&self, event: &dyn Fn() -> salsa::Event) {
        println!("trigger event:{:?}",event())
    }
}

#[salsa::db]
impl Db for MyDb{
    // 按照 reference 所说， num 这种input 是作为 &mut 进入数据库的，而中间结果比如 tracked 是不可变数据
    fn input(&self, num:i32)-> Num{
        match self.calculated.entry(num){
            dashmap::Entry::Occupied(occupied_entry) => {
                occupied_entry.get().clone()
            },
            dashmap::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(Num::new(self, num)).clone()
            },
        }
    }
}

// you should guarantee that all input types are SalsaStructInDb
// #[salsa::tracked] fn fib(db:&dyn Db,i:i32){} lead to an error
#[salsa::tracked]
fn fib(db:&dyn Db, i:Num)->Num{
    match i.num(db) {
        0 => {Num::new(db, 1)},
        1 => {Num::new(db, 1)},
        x => { 
            db.input(fib(db, db.input(x-1)).num(db) +
            fib(db, db.input( x-1)).num(db) )
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
    let num = fib(&db, i);
    let num = fib(&db, i);
    let num = fib(&db, i);
    let num = fib(&db, i);
    println!("{:?}", num.num(&db))
}