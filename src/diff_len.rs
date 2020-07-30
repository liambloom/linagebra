WHARNING THIS IS UNCHECKED

use std::sync::Mutex;

pub enum DifferentLength {
    Panic,
    Warn,
    Allow,
}

impl DifferentLength {
    pub fn run(&self, value_name: &str, operation: &str) {
        match self {
            DifferentLength::Panic => panic!("You cannot {} {} of different lengths", operation, value_name),
            DifferentLength::Warn => println!("You shouldn't {} {} of different lengths", operation, value_name),
            DifferentLength::Allow => (),
        }
    }
}

pub trait HasLength {
    const DIFF_LENGTH: Mutex<DifferentLength>;
    fn diff_len<'a>() -> &'a DifferentLength {
        &*Self::DIFF_LENGTH.lock().unwrap()
    }
    fn dif_len_mut<'a>() -> &'a mut DifferentLength {
        Self::DIFF_LENGTH.get_mut().unwrap()
    }
}