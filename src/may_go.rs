#[macro_use]
extern crate may;

use std::thread;
use std::io::{self, Write};
use may::coroutine;
use std::time::Duration;

fn main() {
    may::config().set_stack_size(0x20);
    
    let max = 2000000;
    let thread_cnt = 1000000;
    let round = 3;
    let mut handlers = vec![];
    let sleep_time = Duration::from_millis(1000);
    
    for i in 0..thread_cnt {
        let handler = go!(move || {
            loop {
                
                for j in 0..max {
                    coroutine::sleep(sleep_time);
                    if j % round == 0 {
                        let stdout = io::stdout();
                        let mut handle = stdout.lock();                        
                        handle.write(&format!("ID = {}; NUM = {}\n", i, j).as_bytes()).expect("Cannot print");
                    }
                }
            }
        });
        handlers.push(handler);
    }
    println!("!!!");
    for handler in handlers {
        handler.join().expect("Cannot join thread");
    }
}
