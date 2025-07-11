use std::ptr::addr_of;

use smol_str::SmolStr;
unsafe trait ATrait{
    fn foo(&self);
}
struct S{

}
unsafe impl ATrait for S{
    fn foo(&self){
        unsafe {
            let ptr = addr_of!(self);
            let content = ptr.read();

        }
    }
}


fn main() {
    let s = smol_str::format_smolstr!("Hello, {}!", "world");
    s.matches("H");
    println!("{}",s);
}

