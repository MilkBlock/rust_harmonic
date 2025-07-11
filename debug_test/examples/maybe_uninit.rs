use std::mem::MaybeUninit;

#[derive(Debug)]
struct Task(u32);

fn main() {
    // 分配未初始化内存
    let mut buffer: Box<[MaybeUninit<Task>]> = Box::new_uninit_slice(3);
    
    // 安全初始化
    unsafe {
        for i in 0..3 {
            buffer[i].as_mut_ptr().write(Task(i as u32));
        }
        let initialized: Box<[Task]> = buffer.assume_init();
        // MaybeUnint<X> -> X  
        println!("{:?}", initialized); // [Task(0), Task(1), Task(2)]
    }
    let mut values = Box::<[u32]>::new_uninit_slice(3);

    let values = unsafe {
        // Deferred initialization:
        values[0].as_mut_ptr().write(1);
        values[1].as_mut_ptr().write(2);
        values[2].as_mut_ptr().write(3);

        values.assume_init()
    };

    assert_eq!(*values, [1, 2, 3])
}
