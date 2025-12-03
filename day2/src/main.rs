use std::fs::read_to_string;

fn main() {
    let lines: String = read_to_string("src/day1.txt").unwrap();
    let part1 = part1(&lines);
    let part2 = part2(&lines);

    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
}

fn part1(lines: &str) -> i32 {
    todo!()
}

fn part2(lines: &str) -> i32 {
    todo!()
}
