// use std::fmt::Display;
// use std::io;
// use std::ops::Mul;
// use std::{env, fmt::Debug};
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let mut datas_string: Vec<&str> = Vec::new();
//     let mut datas: Vec<i64> = Vec::new();
//     let mut input = String::new();
//
//     io::stdin().read_line(&mut input);
//     datas_string = input.trim().split_whitespace().collect();
//     for data in datas_string {
//         datas.push(data.parse().unwrap());
//     }
//
//     println!("Hello, world! {datas:?}");
// }

#[derive(Debug, Clone)]
enum ConsList<T> {
    Cons(T, Box<ConsList<T>>),
    Nil,
}
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use crate::ConsList::{Cons, Nil};

#[derive(Debug)]
struct ConsListCanvas(usize, usize, usize, Option<String>);

impl<T: Into<String>> ConsList<T> {
    fn new() -> Self {
        Nil
    }

    fn push_to_conslist(self, new_item: T) -> ConsList<T> {
        Cons(new_item, Box::new(self))
    }

    fn inspect(self, depth: usize, sender: Sender<ConsListCanvas>) {
        let sub_depth = depth + 1;

        match self {
            Cons(item, sub_conslist) => {
                let item_str: String = item.into();
                let item_len = item_str.len() + 4;

                let _ = sub_conslist.inspect(sub_depth, sender.clone());
                let width = item_len;
                let height = 5;

                let _ = sender.send(ConsListCanvas(depth, width, height, Some(item_str)));
            }
            Nil => {
                let _ = sender.send(ConsListCanvas(depth, 0, 0, None));
            }
        }
    }

    fn start_inspect(self, sender: Sender<ConsListCanvas>) {
        let v: Vec<ConsListCanvas> = Vec::new();
        self.inspect(0, sender);
    }
}

fn test() {
    let (tx_canvas, rx_canvas): (Sender<ConsListCanvas>, Receiver<ConsListCanvas>) = mpsc::channel();

    let handle = thread::spawn(move || {
        let conslist = ConsList::new();
        
        conslist.push_to_conslist("1104")
                .push_to_conslist("Hello")
                .push_to_conslist("world")
                .start_inspect(tx_canvas);
    });

    for received in rx_canvas {
        if received.1 == 0 {
            continue;
        }
        
        println!("Got: {received:?}")
    }
}

fn main() {
    test();
}