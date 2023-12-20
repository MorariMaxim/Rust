use std::collections::HashMap; 
use std::{cell::RefCell, rc::Rc,io,io::Write};

fn main() {
    let mut cache = Cache::new(10);

    loop {
        print!("Enter a number (or 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
 
        if input.trim() == "exit" {
            break;
        }
 
        match input.trim().parse::<u64>() {
            Ok(num) => { 
                let is_prime = cache.check(num);

                if is_prime {
                    println!("{} is prime", num);
                } else {
                    println!("{} is not prime", num);
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}
struct Cache {
    ordered_nodes: LinkedList,
    quick_access: HashMap<u64, (Rc<RefCell<Node>>, bool)>,
    size: u64,
    capacity: u64,
}

impl Cache {
    fn new(mut _capacity: usize) -> Self {
        if _capacity < 10 {
            _capacity = 10
        };
        Self {
            ordered_nodes: LinkedList::new(_capacity),
            quick_access: HashMap::new(),
            size: 0,
            capacity: _capacity as u64,
        }
    }

    fn check(&mut self, number: u64) -> bool {
        if let Some(val) = self.quick_access.get(&number) {
            val.1
        } else {
            let r = Cache::is_prime_number(number);

            if self.size == self.capacity {
                let temp = self.ordered_nodes.delete_oldest();

                if let Some(val) = temp {
                    self.quick_access.remove(&(val.borrow_mut()).value);
                }
            }

            let temp = self.ordered_nodes.add(number);

            self.quick_access.insert(number, (temp, r));

            r
        }
    }

    fn is_prime_number(num: u64) -> bool {
        if num < 2 {
            return false;
        }
        for i in 2..=(num as f64).sqrt() as u64 {
            if num % i == 0 {
                return false;
            }
        }
        true
    }
}

pub struct Node {
    pub next: Option<Rc<RefCell<Node>>>,
    pub value: u64,
}

pub struct LinkedList {
    pub oldest: Rc<RefCell<Node>>,
    pub newest: Rc<RefCell<Node>>,
}

impl LinkedList {
    pub fn new(_capacity: usize) -> Self {
        let start_node = Rc::new(RefCell::new(Node {
            next: None,
            value: 0,
        }));
        Self {
            oldest: start_node.clone(),
            newest: start_node.clone(),
        }
    }
    pub fn add(&mut self, value: u64) -> Rc<RefCell<Node>> {
        let new_node = Rc::new(RefCell::new(Node { next: None, value }));
        (*self.newest.borrow_mut()).next = Some(new_node.clone());
        self.newest = new_node;
        self.newest.clone()
    }
    pub fn sum_all(&self) -> u64 {
        let mut sum = 0;
        let mut current = self.oldest.clone();
        loop {
            let next_node;
            {
                let tmp = RefCell::borrow(&current);
                sum += (*tmp).value;
                next_node = if let Some(next) = (*tmp).next.as_ref() {
                    Some(next.clone())
                } else {
                    None
                };
            }
            if next_node.is_none() {
                break;
            }
            current = next_node.unwrap();
        }
        sum
    }

    pub fn delete_oldest(&mut self) -> Option<Rc<RefCell<Node>>> {
        let next = self.oldest.borrow_mut().next.clone();

        if let Some(next) = next {
            let next = next.borrow_mut().next.clone();
            let temp = next.clone();

            if next.is_none() {
                self.newest = self.oldest.clone();
            }

            self.oldest.borrow_mut().next = next;

            return temp;
        }
        None
    }
}
