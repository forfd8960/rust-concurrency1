use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub name: String,
}

impl Message {
    pub fn new(id: i64, name: String) -> Self {
        Self { id, name }
    }
}

pub fn write_messages(msg_count: i32, tx: Sender<Message>) {
    let handles: Vec<_> = (0..msg_count)
        .map(|idx| {
            let tx_inner = tx.clone();
            thread::spawn(move || {
                let msg = Message::new(idx as i64, format!("MSG:{}", idx));
                println!("sending msg: {:?}", msg);
                tx_inner.send(msg).unwrap();
            })
        })
        .collect();

    handles.into_iter().for_each(|h| h.join().unwrap());
}

pub fn consume_messages(msg_count: i32, rx: Receiver<Message>) {
    for _ in 0..msg_count {
        let msg = rx.recv().unwrap();
        println!("received msg: {:?}", msg);
    }
}

pub fn mpsc() {
    let msg_count = 10;
    let (tx, rx) = channel();
    let consumer = thread::spawn(move || {
        consume_messages(msg_count, rx);
    });

    let producer = thread::spawn(move || {
        write_messages(msg_count, tx);
    });

    consumer.join().unwrap();
    producer.join().unwrap();
}
