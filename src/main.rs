use std::cell::RefCell;
use std::convert::TryInto;
use std::io;
use std::rc::Rc;

struct Packet {
    price: isize,
    quantity: isize,
}

impl Packet {
    pub fn new(price: isize, quantity: isize) -> Packet {
        Packet {
            price: price,
            quantity: quantity,
        }
    }

    pub fn from_array(price_list: Vec<isize>) -> Vec<Rc<Packet>> {
        let mut packet_list: Vec<Rc<Packet>> = Vec::new();
        for index in 0..price_list.len() {
            let quantity: isize = (index + 1).try_into().unwrap();
            let price = price_list.get(index).unwrap();
            if price > &0 {
                let new_packet = Rc::new(Packet::new(*price, quantity));
                packet_list.push(new_packet);
            }
        }
        packet_list.sort_by(|a, b| {
            if a.price == b.price {
                return a.quantity.cmp(&b.quantity);
            }
            return a.price.cmp(&b.price);
        });
        return packet_list;
    }
}

struct Possibility {
    packet_list: Vec<Rc<Packet>>,
}

impl Possibility {
    pub fn empty() -> Possibility {
        Possibility {
            packet_list: Vec::new(),
        }
    }

    pub fn new(avaliable_packets: &Vec<Rc<Packet>>, choices: Vec<usize>) -> Possibility {
        let mut new_possibilty = Possibility {
            packet_list: Vec::new(),
        };
        for index in 0..choices.len() {
            let packet_quantity = choices.get(index).unwrap();
            let packet = avaliable_packets.get(index).unwrap();
            for _ in 0..*packet_quantity {
                new_possibilty.packet_list.push(Rc::clone(packet));
            }
        }
        return new_possibilty;
    }

    fn total_cost(&mut self) -> isize {
        let mut sum = 0;
        for index in 0..self.packet_list.len() {
            let packet = self.packet_list.get(index).unwrap();
            sum = sum + packet.price;
        }
        return sum;
    }

    fn total_apples(&mut self) -> isize {
        let mut sum = 0;
        for index in 0..self.packet_list.len() {
            let packet = self.packet_list.get(index).unwrap();
            sum = sum + packet.quantity;
        }
        return sum;
    }

    fn remaining(&mut self, max_packets: usize) -> usize {
        max_packets - self.packet_list.len()
    }

    fn clone(&mut self) -> Possibility {
        let mut clone = Possibility {
            packet_list: Vec::new(),
        };
        for packet in self.packet_list.iter() {
            clone.packet_list.push(Rc::clone(packet));
        }
        return clone;
    }

    fn add(&mut self, new_packet: &Rc<Packet>) {
        self.packet_list.push(Rc::clone(new_packet));
    }
}

struct Problem {
    packet_list: Vec<Rc<Packet>>,
    max_packets: usize,
    required_apples: isize,
}

impl Problem {
    pub fn new(prices: Vec<isize>, max_packets: usize, required_apples: isize) -> Problem {
        let packet_list = Packet::from_array(prices);
        return Problem {
            packet_list: packet_list,
            max_packets: max_packets,
            required_apples: required_apples,
        };
    }

    fn solution(&mut self) -> Option<Possibility> {
        let initial_possibility = RefCell::new(Possibility::empty());
        match self.search(initial_possibility) {
            Some(solution) => return Some(solution.into_inner()),
            None => return None,
        }
    }

    fn search(&mut self, possibility: RefCell<Possibility>) -> Option<RefCell<Possibility>> {
        // for item in possibility.borrow().packet_list.iter() {
        //     print!("({},{}), ", item.quantity, item.price);
        // }
        // println!();

        if possibility.borrow_mut().total_apples() == self.required_apples {
            return Some(possibility);
        }

        if possibility.borrow_mut().remaining(self.max_packets) == 0 {
            return None;
        }

        let new_possibility_list: Vec<RefCell<Possibility>> = self
            .packet_list
            .iter()
            .map(|packet| {
                let new_possibilty = RefCell::new(possibility.borrow_mut().clone());
                new_possibilty.borrow_mut().add(packet);
                return new_possibilty;
            })
            .collect();

        let mut best_solution: Option<RefCell<Possibility>> = None;
        let mut current_cost = -1;
        for new_possibilty in new_possibility_list {
            match self.search(new_possibilty) {
                Some(solution) => {
                    let no_solution_yet = current_cost == -1;
                    if no_solution_yet {
                        current_cost = solution.borrow_mut().total_cost();
                        best_solution = Some(solution);
                        continue;
                    }
                    let possible_better_cost = solution.borrow_mut().total_cost();
                    if possible_better_cost < current_cost {
                        current_cost = solution.borrow_mut().total_cost();
                        best_solution = Some(solution);
                    }
                }
                None => continue,
            }
        }

        return best_solution;
    }
}

#[test]
fn test_packet_from_array() {
    /* Trivial constructor */
    let packet_list = Packet::from_array(vec![1, 2, 3]);

    assert!(packet_list.len() == 3);

    assert!(packet_list.get(0).unwrap().price == 1);
    assert!(packet_list.get(1).unwrap().price == 2);
    assert!(packet_list.get(2).unwrap().price == 3);

    assert!(packet_list.get(0).unwrap().quantity == 1);
    assert!(packet_list.get(1).unwrap().quantity == 2);
    assert!(packet_list.get(2).unwrap().quantity == 3);

    /* Non-trivial constructor */
    let packet_list = Packet::from_array(vec![9, -1, 7]);

    /* -1 index will omit second packet */
    assert!(packet_list.len() == 2);

    /* packets are sorted by price, so first packet costs 7, second costs 9 */
    assert!(packet_list.get(0).unwrap().price == 7);
    assert!(packet_list.get(1).unwrap().price == 9);

    /* packets are sorted by price, so first packet contains 3, second contains 1 */
    assert!(packet_list.get(0).unwrap().quantity == 3);
    assert!(packet_list.get(1).unwrap().quantity == 1);
}

#[test]
fn test_possibility_total_cost() {
    let packet_list = Packet::from_array(vec![1, 2, 3, 4, 5]);

    assert!(Possibility::new(&packet_list, vec![1]).total_cost() == 1);
    assert!(Possibility::new(&packet_list, vec![3]).total_cost() == 3);
    assert!(Possibility::new(&packet_list, vec![0, 0, 3]).total_cost() == 9);
    assert!(Possibility::new(&packet_list, vec![0, 0, 0, 2]).total_cost() == 8);
}

#[test]
fn test_possibility_total_quantity() {
    let packet_list = Packet::from_array(vec![1, 2, 3, 4, 5]);

    assert!(Possibility::new(&packet_list, vec![1]).total_apples() == 1);
    assert!(Possibility::new(&packet_list, vec![3]).total_apples() == 3);
    assert!(Possibility::new(&packet_list, vec![0, 0, 3]).total_apples() == 9);
    assert!(Possibility::new(&packet_list, vec![0, 0, 0, 2]).total_apples() == 8);
}

#[test]
fn test_possibility_remaining() {
    let packet_list = Packet::from_array(vec![1, 2, 3, 4, 5]);

    assert!(Possibility::new(&packet_list, vec![0, 0, 3]).remaining(5) == 2);
    assert!(Possibility::new(&packet_list, vec![0, 2, 3]).remaining(5) == 0);
}

#[test]
fn test_possibility_clone() {
    let packet_list = Packet::from_array(vec![1, 2, 3, 4, 5]);

    let initial = RefCell::new(Possibility::new(&packet_list, vec![0, 0, 3]));
    let clone = RefCell::new(initial.borrow_mut().clone());

    /* clone is equal to original */
    assert!(initial.borrow_mut().total_cost() == 9);
    assert!(initial.borrow_mut().total_apples() == 9);
    assert!(initial.borrow_mut().remaining(5) == 2);

    assert!(clone.borrow_mut().total_cost() == 9);
    assert!(clone.borrow_mut().total_apples() == 9);
    assert!(clone.borrow_mut().remaining(5) == 2);

    clone.borrow_mut().add(packet_list.get(4).unwrap());

    /* clone modification won't change original item */
    assert!(initial.borrow_mut().total_cost() == 9);
    assert!(initial.borrow_mut().total_apples() == 9);
    assert!(initial.borrow_mut().remaining(5) == 2);

    /* clone is correct */
    assert!(clone.borrow_mut().total_cost() == 14);
    assert!(clone.borrow_mut().total_apples() == 14);
    assert!(clone.borrow_mut().remaining(5) == 1);
}

#[test]
fn test_problem_solving() {
    assert!(Problem::new(vec![1, 2, 3, 4, 5], 5, 5).solution().unwrap().total_cost() == 5);
    assert!(Problem::new(vec![-1, -1, 4, 5, -1], 3, 5).solution().is_none());
    assert!(Problem::new(vec![10, 2, 3, 4], 4, 7).solution().unwrap().total_cost() == 7);
    assert!(Problem::new(vec![10,2,3,4], 10, 5).solution().unwrap().total_cost() == 5);
    // assert!(Problem::new(vec![10,2,3,4], 10, 1).solution().unwrap().total_cost() == 10);
}

fn read_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed at read_line");

    return input.trim().parse().expect("Failed to parse");
}

fn read_vec() -> Vec<isize> {
    let input: Vec<isize> = read_input()
        .split(" ")
        .map(|x| x.parse().expect("Failed to parse interger"))
        .collect();

    return input;
}

fn read_dup() -> (isize, isize) {
    let input = read_vec();
    return (input[0], input[1]);
}

fn main() {
    let total_tests: usize = read_input().parse().expect("Failed to parse interger");

    for _ in 0..total_tests {
        let (max_packets, required_apples) = read_dup();
        let prices = read_vec();
        let problem = Problem::new(prices, max_packets.try_into().expect(""), required_apples);
    }
}
