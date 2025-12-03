use std::fs::read_to_string;

fn part1(lines: &str) -> i32 {
    let mut start = 50;
    let mut count = 0;
    let instructions: Vec<(&str, i32)> = lines
        .lines()
        .map(|l| {
            let (direction, num) = l.split_at(1);
            (direction, num.parse::<i32>().unwrap())
        })
        .collect();

    for instruction in instructions {
        if instruction.0 == "L" {
            if (start - instruction.1) % 100 == 0 {
                count += 1;
                start = 0;
            } else if start - instruction.1 < 0 {
                start = 100 - (start - instruction.1).abs();
            } else {
                start -= instruction.1;
            }
        } else if (start + instruction.1) % 100 == 0 {
            count += 1;
            start = 0;
        } else if start + instruction.1 > 100 {
            start = start + instruction.1 - 100;
        } else {
            start += instruction.1;
        }
    }
    count
}

fn part2(lines: &str) -> i32 {
    let mut start = 50;
    let mut count = 0;
    let instructions: Vec<(&str, i32)> = lines
        .lines()
        .map(|l| {
            let (direction, num) = l.split_at(1);
            (direction, num.parse::<i32>().unwrap())
        })
        .collect();

    for instruction in instructions {
        let mut num = instruction.1;
        if instruction.0 == "L" {
            num = -num;
        }
        if start == 0 && num < 0 {
            count -= 1;
        }
        start += num;
        count += start.div_euclid(100).abs();
        start = start.rem_euclid(100);
        if start == 0 && num < 0 {
            count += 1
        }
    }
    count
}

fn main() {
    let lines: String = read_to_string("src/day1.txt").unwrap();
    let part1 = part1(&lines);
    let part2 = part2(&lines);

    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
}
