use std::convert::TryInto;
use std::io;
use std::rc::Rc;

struct Packet {
    price: usize,
    quantity: usize,
}

impl Packet {
    pub fn new(price: usize, quantity: usize) -> Packet {
        Packet {
            price: price.try_into().expect(""),
            quantity: quantity,
        }
    }

    pub fn from_array(price_list: Vec<isize>) -> Vec<Rc<Packet>> {
        let mut packet_list: Vec<Rc<Packet>> = Vec::new();
        for index in 0..price_list.len() {
            let quantity = index + 1;
            let price = price_list.get(index).unwrap();
            if price > &0 {
                let price: usize = (*price).try_into().expect("");
                let new_packet = Rc::new(Packet::new(price, quantity));
                packet_list.push(new_packet);
            }
        }
        packet_list.sort_by(|a, b| {
            let relative_a = a.price * b.quantity;
            let relative_b = b.price * a.quantity;
            if relative_a == relative_b {
                return b.quantity.cmp(&a.quantity);
            }
            return relative_a.cmp(&relative_b);
        });
        return packet_list;
    }
}

struct Possibility {
    packet_list: Vec<Rc<Packet>>,
}

impl Possibility {
    // pub fn empty() -> Possibility {
    //     Possibility {
    //         packet_list: Vec::new(),
    //     }
    // }

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

    fn total_cost(&mut self) -> usize {
        let mut sum = 0;
        for index in 0..self.packet_list.len() {
            let packet = self.packet_list.get(index).unwrap();
            sum = sum + packet.price;
        }
        return sum;
    }

    fn merge(&mut self, other: &Possibility) {
        for packet in other.packet_list.iter() {
            self.packet_list.push(Rc::clone(packet));
        }
    }
}

struct Problem {
    packet_list: Vec<Rc<Packet>>,
    max_packets: usize,
    required_apples: usize,
}

impl Problem {
    pub fn new(prices: Vec<isize>, max_packets: usize, required_apples: usize) -> Problem {
        let packet_list = Packet::from_array(prices);
        return Problem {
            packet_list: packet_list,
            max_packets: max_packets,
            required_apples: required_apples,
        };
    }

    fn solution(&mut self) -> Option<Possibility> {
        if self.packet_list.len() == 0 {
            return None;
        }
        
        let mut deeper_problem = self.clone();
        let next_packet = deeper_problem.packet_list.remove(0);

        let mut step_length: usize = self.required_apples / next_packet.quantity;
        let remainer = self.required_apples % next_packet.quantity;
        
        if remainer == 0 && step_length <= self.max_packets {
            let possibility = Possibility::new(&self.packet_list, vec![step_length]);
            return Some(possibility);
        }

        if self.max_packets == 0 {
            return None;
        }
        
        if step_length > self.max_packets {
            step_length = self.max_packets;
        }
        
        let mut best_possibility: Option<Possibility> = None;
        let mut best_cost = 0;
        loop {
            let included_apples = step_length * next_packet.quantity;
            
            deeper_problem.max_packets = self.max_packets - step_length;
            deeper_problem.required_apples = self.required_apples - included_apples;
            
            match deeper_problem.solution() {
                Some(mut inner_possibility) => {
                    if best_cost == 0 || inner_possibility.total_cost() < best_cost {
                        best_cost = inner_possibility.total_cost();
                        let mut possibility = Possibility::new(&self.packet_list, vec![step_length]);
                        possibility.merge(&inner_possibility);
                        best_possibility = Some(possibility);
                    }
                },
                None => {},
            }
            if step_length == 0 { break; }
            step_length = step_length - 1;
        }
        
        return best_possibility;
    }
    
    fn clone(&mut self) -> Problem {
        let mut new_problem = Problem {
            packet_list: Vec::new(),
            max_packets: self.max_packets,
            required_apples: self.required_apples,
        };
        for packet in self.packet_list.iter() {
            new_problem.packet_list.push(Rc::clone(packet));
        }
        return new_problem;
    }
}

#[test]
fn test_packet_from_array() {
    /* Trivial constructor */
    let packet_list = Packet::from_array(vec![1, 2, 3]);

    assert!(packet_list.len() == 3);

    assert!(packet_list.get(0).unwrap().price == 3);
    assert!(packet_list.get(1).unwrap().price == 2);
    assert!(packet_list.get(2).unwrap().price == 1);

    assert!(packet_list.get(0).unwrap().quantity == 3);
    assert!(packet_list.get(1).unwrap().quantity == 2);
    assert!(packet_list.get(2).unwrap().quantity == 1);

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

    assert!(Possibility::new(&packet_list, vec![1]).total_cost() == 5);
    assert!(Possibility::new(&packet_list, vec![3]).total_cost() == 15);
    assert!(Possibility::new(&packet_list, vec![0, 0, 3]).total_cost() == 9);
    assert!(Possibility::new(&packet_list, vec![0, 0, 0, 2]).total_cost() == 4);
}

#[test]
fn test_problem_solving() {
    assert_eq!(Problem::new(vec![1,2,3,4,5], 5, 5).solution().unwrap().total_cost(), 5);
    assert_eq!(Problem::new(vec![10,2,3,4], 10, 1).solution().unwrap().total_cost(), 10);
    match Problem::new(vec![-1,-1,4,5,-1], 3, 5).solution() {
        Some(_) => panic!("Not supposed to have a solution"),
        None => assert!(true),
    };
    assert_eq!(Problem::new(vec![10,2,3,4], 4, 7).solution().unwrap().total_cost(), 7);
    assert_eq!(Problem::new(vec![10,2,3,4], 15, 5).solution().unwrap().total_cost(), 5);
}

#[test]
fn test_problem_solving_advanced() {
    match Problem::new(vec![2,1,3,-1,4,10], 3,6).solution() {
        Some(mut solution) => assert_eq!(solution.total_cost(), 3),
        None => panic!("Not supposed to have a solution"),
    };
    
    match Problem::new(vec![2,5,3,2,6], 1,5).solution() {
        Some(mut solution) => assert_eq!(solution.total_cost(), 6),
        None => panic!("Not supposed to have a solution"),
    };
    
    match Problem::new(vec![2,1,3,-1,4], 3,5).solution() {
        Some(mut solution) => assert_eq!(solution.total_cost(), 4),
        None => panic!("Not supposed to have a solution"),
    };
 
    match Problem::new(vec![-1,-1,3,-1,-1,-1,7,-1,-1,-1,-1,-1,13,-1], 2,23).solution() {
        Some(_) => panic!("Not supposed to have a solution"),
        None => assert!(true),
    };
    
    
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

fn read_dup() -> (usize, usize) {
    let input = read_vec();
    return (input[0].try_into().expect(""), input[1].try_into().expect(""));
}

fn main() {
    let total_tests: usize = read_input().parse().expect("Failed to parse interger");

    for _ in 0..total_tests {
        let (max_packets, required_apples) = read_dup();
        let prices = read_vec();
        let mut problem = Problem::new(prices, max_packets, required_apples);
        match problem.solution() {
            Some(mut solution) => println!("{}", solution.total_cost()),
            None => println!("-1"),
        };
    }
}
