pub fn part_one(input: &str) -> Option<u32> {
    lib03::part_one(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    lib03::part_two(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
mod lib03{

    pub fn part_one(input:&str)->Option<u32>{
        let lines: Vec<_> = input.split("\n").collect();
        let res: u32 = lines.iter().map(|x| get_type(x)).sum::<u32>();
        Some(res)
    }
    pub fn part_two(input:&str)->Option<u32>{
        let mut res2 = 0;
        let mut vec = Vec::new();
        let lines: Vec<_> = input.split("\n").collect();
        for line in lines.iter() {
            vec.push(line);
            if vec.len()==3 {
                if let Some(c) = common_char3(vec.get(0).unwrap(), vec.get(1).unwrap(), vec.get(2).unwrap())
                {
                    res2 += char_to_order(c);
                }
                vec.clear();
            }
        }
        Some(res2)
    }

    fn get_type(items: &str) -> u32 {
        let (i1, i2) = items.split_at(items.len() / 2);
        if let Some(item_type) = common_char2(i1, i2) {
            char_to_order(item_type)
        } else {
            0
        }
    }

    fn common_char2(i1: &str, i2: &str) -> Option<char> {
        for x in i1.chars() {
            if i2.contains(x) {
                return Some(x);
            }
        }
        None
    }

    fn common_char3(i1: &str, i2: &str, i3: &str) -> Option<char> {
        for x in i1.chars() {
            if i2.contains(x) && i3.contains(x) {
                return Some(x);
            }
        }
        None
    }

    fn char_to_order(item_type: char) -> u32 {
        let x = if item_type.is_lowercase() {
            item_type as u8 - b'a' + 1
        } else {
            item_type as u8 - b'A' + 27
        };
        x as u32
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_type_p() {
            let a = "vJrwpWtwJgWrhcsFMMfFFhFp";
            let ai = get_type(a);
            assert_eq!(ai, 16);
        }

        #[test]
        fn test_get_type_V() {
            let a = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
            let ai = get_type(a);
            assert_eq!(ai, 38);
        }

        #[test]
        fn test_get_type_P() {
            let a = "PmmdzqPrVvPwwTWBwg";
            let ai = get_type(a);
            assert_eq!(ai, 42);
        }

        #[test]
        fn test_get_type_v() {
            let a = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
            let ai = get_type(a);
            assert_eq!(ai, 22);
        }

        #[test]
        fn test_first_group() {
            let ai = common_char3("vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg");
            let ac = char_to_order(ai.unwrap());
            assert_eq!(ai.unwrap(), 'r');
            assert_eq!(ac, 18);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
