use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

/*
pushing 2 to my_vec
pushing 0 to my_vec
pushing 1 to my_vec
pushing 6 to my_vec
pushing 3 to my_vec
pushing 5 to my_vec
pushing 4 to my_vec
pushing 7 to my_vec
pushing 8 to my_vec
pushing 9 to my_vec
[2, 0, 6, 1, 3, 5, 4, 7, 8, 9]
*/
pub fn concurrent_vec(size: i32) {
    let my_vec = Arc::new(Mutex::new(Vec::with_capacity(size as usize)));

    let handles: Vec<_> = (0..size)
        .map(|idx| {
            let inner = my_vec.clone();
            thread::spawn(move || {
                println!("pushing {} to my_vec", idx);
                inner.lock().unwrap().push(idx);
            })
        })
        .collect();

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{:?}", my_vec.lock().unwrap());
}

/*
pushing 0:value-0 to my_hash
pushing 2:value-2 to my_hash
pushing 5:value-5 to my_hash
pushing 1:value-1 to my_hash
pushing 7:value-7 to my_hash
pushing 8:value-8 to my_hash
pushing 9:value-9 to my_hash
pushing 4:value-4 to my_hash
pushing 3:value-3 to my_hash
pushing 6:value-6 to my_hash
concurrent_hashmap: {5: "value-5", 4: "value-4", 1: "value-1", 6: "value-6", 9: "value-9", 0: "value-0", 2: "value-2", 7: "value-7", 3: "value-3", 8: "value-8"}
*/
pub fn concurrent_hashmap(size: i32) {
    let my_hash = Arc::new(Mutex::new(HashMap::with_capacity(size as usize)));

    let handles: Vec<_> = (0..size)
        .map(|idx| {
            let inner = my_hash.clone();
            thread::spawn(move || {
                let val = format!("value-{}", idx);
                println!("pushing {}:{} to my_hash", idx, val);
                inner.lock().unwrap().insert(idx, val);
            })
        })
        .collect();

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("concurrent_hashmap: {:?}", my_hash.lock().unwrap());
}
