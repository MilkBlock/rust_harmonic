use tracing::{info, instrument};
use tracing_subscriber;

#[derive(Debug)]
struct A {}

#[instrument]
fn my_function(param1: u32, param2: String, param3: A) {
    info!("Inside my_function");
    println!("bulabula this has been executed");
    b(param1, param2, param3)
    // 函数体
}
#[instrument(skip(param3))]
fn b(param1: u32, param2: String, param3: A) {
    info!("Inside my_function");
    info!("Inside my_function");
    info!("Inside my_function");
    info!("Inside my_function");
    println!("bulabula this has been executed")
    // 函数体
}

fn main() {
    tracing_subscriber::fmt::init();
    my_function(42, "test".to_string(), A {});
}
