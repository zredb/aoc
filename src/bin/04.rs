pub fn part_one(input: &str) -> Option<u32> {
    lib04::contains(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    lib04::overlaps(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

mod lib04 {
    pub fn contains(input: &str) -> Option<u32> {
        let lines: Vec<_> = input.trim_end().split("\n").collect();
        let mut contains = 0;
        for line in lines.iter() {
            let (p1, p2) = parse_line(line);
            if p1.contains(&p2) || p2.contains(&p1) {
                contains += 1;
            }
        }
        Some(contains)
    }

    pub fn overlaps(input: &str) ->Option<u32>{
        let lines: Vec<_> = input.trim_end().split("\n").collect();
        let mut overlaps = 0;
        for line in lines.iter() {
            let (p1, p2) = parse_line(line);
            if p1.overlaps(&p2) {
                overlaps += 1;
            }
        }
        Some(overlaps)
    }

    struct Pair {
        min: u32,
        max: u32,
    }

    impl Pair {
        fn new(min: u32, max: u32) -> Self {
            Pair { min, max }
        }
        fn contains(&self, other: &Self) -> bool {
            self.min <= other.min && self.max >= other.max
        }
        //不重叠有两种情况：A在B前面，A在B后面
        fn overlaps(&self, other: &Self) -> bool {
            !(self.max < other.min || self.min > other.max)
            // max(self.min, other.min)<=min(self.max,other.max) //法2
        }
    }

    fn parse_line(line: &str) -> (Pair, Pair) {
        let x = line.split(',').collect::<Vec<_>>();
        let p1 = parse_pair(x.get(0).unwrap());
        let p2 = parse_pair(x.get(1).unwrap());
        (p1, p2)
    }

    fn parse_pair(pair: &str) -> Pair {
        let x = pair.split('-').collect::<Vec<_>>();
        let min: u32 = x.get(0).unwrap().parse().unwrap();
        let max: u32 = x.get(1).unwrap().parse().unwrap();

        Pair::new(min, max)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_pair() {
            let a = "7-24";
            let p = parse_pair(a);
            assert_eq!(p.min, 7);
            assert_eq!(p.max, 24);
        }

        #[test]
        fn test_parse_Line() {
            let a = "7-24,8-8";
            let (p1, p2) = parse_line(a);
            assert_eq!(p1.min, 7);
            assert_eq!(p1.max, 24);

            assert_eq!(p2.min, 8);
            assert_eq!(p2.max, 8);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
