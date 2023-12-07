mod part1;
fn main() {
    let input = include_str!("../input.txt");
    let result1 = part1::solve(input);
    println!("part1 = {result1}");
}
