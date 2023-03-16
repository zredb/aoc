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