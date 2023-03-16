pub fn part_one(input: &str) -> Option<u32> {
    lib02::part_one(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    lib02::part_two(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

mod lib02{

    pub fn part_one(input:&str)->Option<u32>{
        let rounds: Vec<&str> = input.trim_end().split("\n").collect();
        let sum1: u32 = rounds.iter().map(|x| calc_round_score1(x)).sum();
        Some(sum1)
    }
    pub fn part_two(input:&str)->Option<u32>{
        let rounds: Vec<&str> = input.trim_end().split("\n").collect();
        let sum2: u32 = rounds.iter().map(|x| calc_round_score2(x)).sum();
        Some(sum2)
    }

    fn get_round_input(round: &str) -> (char, char) {
        let round: Vec<_> = round.chars().collect();
        if round.is_empty(){
            println!("empty");
        }
        let opponent = round.first().unwrap();
        let you = round.last().unwrap();
        (*opponent, *you)
    }

    fn calc_round_score1(round: &str) -> u32 {
        let (opponent, you) = get_round_input(round);
        let opponent=Options::from(opponent);
        let you=Options::from(you);
        you as u32 + judge(opponent, you) as u32
    }

    fn calc_round_score2(round: &str) -> u32 {
        let (opponent, you) = get_round_input(round);
        let your_choose = you_should_select(opponent, you);
        let opponent=Options::from(opponent);
        your_choose as u32 + judge(opponent, your_choose) as u32
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    enum Options {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    /// A for Rock, B for Paper, and C for Scissors.
    /// X for Rock, Y forPaper, Z for Scissors.
    impl From<char> for Options {
        fn from(value: char) -> Self {
            match value {
                'A' | 'X' => Options::Rock,
                'B' | 'Y' => Options::Paper,
                'C' | 'Z' => Options::Scissors,
                _ => unreachable!(),
            }
        }
    }

    #[repr(C)]
    enum States {
        Loss = 0,
        Draw = 3,
        Win = 6,
    }

    /// X means you need to lose,
    /// Y means you need to draw
    /// Z means you need to win.
    impl From<char> for States {
        fn from(value: char) -> Self {
            match value {
                'X' => States::Loss,
                'Y' => States::Draw,
                'Z' => States::Win,
                _ => unreachable!(),
            }
        }
    }

    fn judge(opponent: Options, you: Options) -> States {
        match opponent {
            Options::Rock => {
                match you {
                    Options::Rock => States::Draw,
                    Options::Paper => States::Win,
                    Options::Scissors => States::Loss,
                }
            }
            Options::Paper => {
                match you {
                    Options::Rock => States::Loss,
                    Options::Paper => States::Draw,
                    Options::Scissors => States::Win,
                }
            }
            Options::Scissors => {
                match you {
                    Options::Rock => States::Win,
                    Options::Paper => States::Loss,
                    Options::Scissors => States::Draw,
                }
            }
        }
    }

    fn you_should_select(opponent: char, strategy: char) -> Options {
        let opponent = Options::from(opponent);
        let state = States::from(strategy);

        match opponent {
            Options::Rock => {
                match state {
                    States::Loss => Options::Scissors,
                    States::Draw => Options::Rock,
                    States::Win => Options::Paper,
                }
            }
            Options::Paper => {
                match state {
                    States::Loss => Options::Rock,
                    States::Draw => Options::Paper,
                    States::Win => Options::Scissors,
                }
            }
            Options::Scissors => {
                match state {
                    States::Loss => Options::Paper,
                    States::Draw => Options::Scissors,
                    States::Win => Options::Rock,
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
