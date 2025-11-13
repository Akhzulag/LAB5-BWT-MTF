#![allow(dead_code)]
use std::cell::RefCell;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
    prev: WeakLink<T>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: PartialEq + Debug + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    pub fn push_front(&mut self, val: T) {
        let new_head = Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(head) => {
                new_head.borrow_mut().next = Some(head.clone());
                head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
                self.head = Some(new_head);
            }

            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn find_remove(&mut self, val: &T) -> Option<usize> {
        let mut index = 0;
        let mut current = self.head.clone();
        while let Some(current_node) = current.as_ref() {
            let next = current_node.borrow().next.clone();
            if current_node.borrow().val == *val {
                let mut current_borrow = current_node.borrow_mut();
                let prev = current_borrow.prev.take();
                let next = current_borrow.next.take();
                match (prev.as_ref().and_then(|w| w.upgrade()), next.as_ref()) {
                    (Some(prev_strong), Some(next_strong)) => {
                        prev_strong.borrow_mut().next = Some(next_strong.clone());
                        next_strong.borrow_mut().prev = prev; // 'prev_link' - це Option<Weak<...>>
                    }

                    (None, Some(next_strong)) => {
                        next_strong.borrow_mut().prev = None;
                        self.head = Some(next_strong.clone());
                    }

                    (Some(prev_strong), None) => {
                        prev_strong.borrow_mut().next = None;
                        self.tail = Some(prev_strong.clone());
                    }

                    (None, None) => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                drop(current_borrow);

                return Some(index);
            }

            current = next;
            index += 1;
        }

        None
    }

    pub fn index_remove(&mut self, index: usize) -> Option<T> {
        let mut i = 0;
        let mut current = self.head.clone();
        while let Some(current_node) = current.as_ref() {
            let next = current_node.borrow().next.clone();
            if index == i {
                let mut current_borrow = current_node.borrow_mut();

                let prev = current_borrow.prev.take();
                let next = current_borrow.next.take();
                match (prev.as_ref().and_then(|w| w.upgrade()), next.as_ref()) {
                    (Some(prev_strong), Some(next_strong)) => {
                        prev_strong.borrow_mut().next = Some(next_strong.clone());
                        next_strong.borrow_mut().prev = prev; // 'prev_link' - це Option<Weak<...>>
                    }

                    (None, Some(next_strong)) => {
                        next_strong.borrow_mut().prev = None;
                        self.head = Some(next_strong.clone());
                    }

                    (Some(prev_strong), None) => {
                        prev_strong.borrow_mut().next = None;
                        self.tail = Some(prev_strong.clone());
                    }

                    (None, None) => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                drop(current_borrow);
                return Some(current_node.borrow().val.clone());
            }

            current = next;
            i += 1;
        }

        None
    }
}

fn init_alphabet() -> LinkedList<u8> {
    let mut alphabet: LinkedList<u8> = LinkedList::new();
    for i in 0..=255 {
        alphabet.push_front(i);
    }
    alphabet
}

pub fn encode(file_read: &str, file_write: &str) -> Result<()> {
    let mut reader = File::open(file_read)?;
    let mut writer = BufWriter::new(File::create(file_write)?);

    let mut alphabet = init_alphabet();
    let mut buf = vec![0; 8 * 1024];
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        buf.iter().take(n).for_each(|x| {
            let index = alphabet.find_remove(x).unwrap();
            alphabet.push_front(*x);
            // println!("{:?}", index);
            writer
                .write_all(&[index as u8])
                .expect("MTF: Encode error writing");
        });
    }

    Ok(())
}

pub fn decode(file_read: &str, file_write: &str) -> Result<()> {
    let mut reader = File::open(file_read)?;
    let mut writer = BufWriter::new(File::create(file_write)?);

    let mut alphabet = init_alphabet();
    let mut index = vec![0; 8 * 1024];
    while let Ok(n) = reader.read(&mut index) {
        if n == 0 {
            break;
        }
        index.iter().take(n).for_each(|x| {
            let byte = alphabet.index_remove(*x as usize).unwrap();
            alphabet.push_front(byte);
            writer
                .write_all(&[byte])
                .expect("MTF: Decode error writing");
        });
    }

    Ok(())
}
