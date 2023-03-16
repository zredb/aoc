
pub fn part_one(input: &str) -> Option<u32> {
    lib01::max_calories(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    lib01::total_calories(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

mod lib01{
    use std::collections::BinaryHeap;

    pub fn max_calories(input: &str) -> Option<u32> {
        let groups: Vec<&str> = input.split("\n\n").collect();
        let calories: Vec<u32> = groups.iter().map(|x| sum(x)).collect::<Vec<_>>();
        let res = calories.iter().max().unwrap();
        Some(*res)
    }

    pub fn total_calories(input: &str) -> Option<u32> {
        let groups: Vec<&str> = input.split("\n\n").collect();
        let calories: Vec<u32> = groups.iter().map(|x| sum(x)).collect::<Vec<_>>();
        let mut bh = BinaryHeap::new();
        for c in calories{
            bh.push(c);
        }
        let total= bh.pop().unwrap()+bh.pop().unwrap()+bh.pop().unwrap();

        Some(total)
    }

    fn sum(g: &str) -> u32 {
        let values: Vec<&str> = g.split("\n").collect();
        let mut sum: u32 = 0;
        for v in values {
            if v.len()>0{
                let v = v.parse::<u32>().unwrap();
                sum += v;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
