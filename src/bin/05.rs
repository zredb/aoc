pub fn part_one(input: &str) -> Option<String> {
    lib05::crate_move_9000(input)
}

pub fn part_two(input: &str) -> Option<String> {
    lib05::crate_move_9001(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

mod lib05 {
    use std::fs::File;
    use std::io::Read;

    pub fn crate_move_9000(input: &str) -> Option<String> {
        let stacks = init_stack();
        let instructions = init_instructions_from_str(input.trim_end());
        let mut crates = Crates::new(stacks, instructions);
        crates.moves(false);
        let res = crates.print_tops();
        Some(res)
    }

    pub fn crate_move_9001(input: &str) -> Option<String> {
        let stacks = init_stack();
        let instructions = init_instructions_from_str(input.trim_end());
        let mut crates = Crates::new(stacks, instructions);
        //println!("first: {}", crates.print_tops());
        crates.moves(true);
        let res = crates.print_tops();
        Some(res)
    }


    #[derive(Clone)]
    struct Crates {
        stacks: Vec<Vec<char>>,
        instructions: Vec<Instruction>,
    }

    impl Crates {
        fn new(stacks: Vec<Vec<char>>, instructions: Vec<Instruction>) -> Self {
            Crates { stacks, instructions }
        }
        fn moves(&mut self, reverse: bool) {
            for i in &self.instructions {
                // self.print_state();
                // dbg!(" {i:?} -> ");
                let mut changes = Vec::new();
                let from = self.stacks.get_mut(i.from - 1).unwrap();
                for _ in 0..i.count {
                    let v = from.pop().unwrap();
                    changes.push(v);
                }
                if reverse {
                    changes.reverse();
                }
                let to = self.stacks.get_mut(i.to - 1).unwrap();
                for i in changes {
                    to.push(i);
                }
                // self.print_state();
                // dbg!("<---------------------------------------------> ");
            }
        }

        fn print_tops(&self) -> String {
            let mut res = String::new();
            for x in self.stacks.iter() {
                let c = *x.last().unwrap();
                res.push(c);
            }
            res
        }

        /// Returns the print state of this [`Crates`].
        fn print_state(&self) {
            for (idx, s) in self.stacks.iter().enumerate() {
                println!("{}: {s:?}", idx + 1);
            }
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    struct Instruction {
        count: u32,
        from: usize,
        to: usize,
    }

    impl From<&str> for Instruction {
        //move 2 from 8 to 2
        fn from(value: &str) -> Self {
            let x = value.split(' ').collect::<Vec<_>>();
            let count: u32 = x.get(1).unwrap().parse().unwrap();
            let from: usize = x.get(3).unwrap().parse().unwrap();
            let to: usize = x.get(5).unwrap().parse().unwrap();
            Self { count, from, to }
        }
    }

    fn init_stack() -> Vec<Vec<char>> {
        let mut res = Vec::new();
        let stack1 = vec!['W', 'T', 'H', 'P', 'J', 'C', 'F'].into_iter().rev().collect();
        let stack2 = vec!['H', 'B', 'J', 'Z', 'F', 'V', 'R', 'G'].into_iter().rev().collect();
        let stack3 = vec!['R', 'T', 'P', 'H'].into_iter().rev().collect();
        let stack4 = vec!['T', 'H', 'P', 'N', 'S', 'Z'].into_iter().rev().collect();
        let stack5 = vec!['D', 'C', 'J', 'H', 'Z', 'F', 'V', 'N'].into_iter().rev().collect();
        let stack6 = vec!['Z', 'D', 'W', 'F', 'G', 'M', 'P'].into_iter().rev().collect();
        let stack7 = vec!['P', 'D', 'J', 'S', 'W', 'Z', 'V', 'M'].into_iter().rev().collect();
        let stack8 = vec!['S', 'D', 'N'].into_iter().rev().collect();
        let stack9 = vec!['M', 'F', 'S', 'Z', 'D'].into_iter().rev().collect();

        res.push(stack1);
        res.push(stack2);
        res.push(stack3);
        res.push(stack4);
        res.push(stack5);
        res.push(stack6);
        res.push(stack7);
        res.push(stack8);
        res.push(stack9);
        res
    }

    fn init_instructions(path: &str) -> Vec<Instruction> {
        let mut res = Vec::new();
        let mut input = String::new();
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut input).unwrap();
        let lines: Vec<_> = input.split("\n").collect();
        for line in lines.iter().skip(10) {
            let i = Instruction::from(*line);
            res.push(i);
        }
        res
    }

    fn init_instructions_from_str(input: &str) -> Vec<Instruction> {
        let mut res = Vec::new();
        let lines: Vec<_> = input.split("\n").collect();
        for line in lines.iter().skip(10) {
            let i = Instruction::from(*line);
            res.push(i);
        }
        res
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_init_stacks() {
            let a = init_stack();
            assert_eq!(*a.first().unwrap(), vec!['W', 'T', 'H', 'P', 'J', 'C', 'F'].into_iter().rev().collect::<Vec<_>>());
            assert_eq!(*a.last().unwrap(), vec!['M', 'F', 'S', 'Z', 'D'].into_iter().rev().collect::<Vec<_>>());
        }

        #[test]
        fn test_init_instructions() {
            let a = init_instructions("../day5/input.txt");
            let first = a.first().unwrap();
            let last = a.last().unwrap();
            let exp_first = &Instruction::from("move 2 from 8 to 2");
            let exp_last = &Instruction::from("move 1 from 2 to 7");
            assert_eq!(first, exp_first);
            assert_eq!(last, exp_last);
        }

        #[test]
        fn test_push_pop() {
            let mut vec = vec![1, 2, 3];
            vec.push(4);
            eprintln!("{:?}", &vec);
            vec.pop();
            eprintln!("{:?}", &vec);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
