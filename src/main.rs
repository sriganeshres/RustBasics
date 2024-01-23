#![allow(unused)]

use core::panic;
use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::{BufRead, BufWriter, ErrorKind, Write};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // let mut input: String = String::new();
    // io::stdin().read_line(&mut input).expect("failed to read");
    // println!("Entered Sentence is {}", input);
    // let x = 2;
    // let y = x;
    // println!("{}", x);
    // let random_num: i32 = rand::thread_rng().gen_range(1..101);
    // println!("Entered Sentence is {}", random_num);

    // let mut hell = String::from("value");
    // println!("Entered Sentence is {}", hell);
    // for val in hell.as_bytes() {
    //     print!("{} ", val);
    // }
    // println!();

    // let path = "lines.txt";
    // let output = File::create(path);
    // let mut output = match output {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem Craeting file {:?}", error);
    //     }
    // };

    // write!(output, "Just\nsome\ntext").expect("Failed to write to file");

    // let inp = File::open(path).unwrap();
    // let buf = BufReader::new(inp);
    // for line in buf.lines() {
    //     println!("{}", line.unwrap());
    // }
    // let arr = [1, 2, 3, 4];
    // let mut iter_tr = arr.iter();
    // println!("{:?}", iter_tr.next());
    // let can_vote = |age: i32| -> bool { age >= 18 };

    // println!("{}", can_vote(10));
    // println!("{}", can_vote(20));
    // let m = Box::new(5);
    // println!("{}", &m);
    // struct TreeNode<T> {
    //     pub left: Option<Box<TreeNode<T>>>,
    //     pub right: Option<Box<TreeNode<T>>>,
    //     pub key: T,
    // }

    // impl<T> TreeNode<T> {
    //     pub fn new(key: T) -> Self {
    //         TreeNode {
    //             left: None,
    //             right: None,
    //             key,
    //         }
    //     }

    //     pub fn left(mut self, node: TreeNode<T>) -> Self {
    //         self.left = Some(Box::new(node));
    //         self
    //     }
    //     pub fn right(mut self, node: TreeNode<T>) -> Self {
    //         self.right = Some(Box::new(node));
    //         self
    //     }
    // }

    // let node1 = TreeNode::new(1)
    //     .left(TreeNode::new(2))
    //     .right(TreeNode::new(5));
    // let thread1 = thread::spawn(|| {
    // for i in 1..25 {
    //   println!("Spawned thread: {}", i);
    // thread::sleep(Duration::from_millis(1));
    //}
    // });
    // for i in 1..20 {
    //  println!("Main Thread: {}", i);
    // thread::sleep(Duration::from_millis(1));
    // }

    // thread1.join().unwrap();
    //
    pub struct Bank {
        balance: f32,
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amnt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("take small amnt as balance is {}", bank_ref.balance);
        } else {
            bank_ref.balance -= amnt;
            println!("withdrew {} balance is {}", amnt, bank_ref.balance);
        }
    }
    fn cust(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.0 }));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            cust(bank_ref);
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);
}
