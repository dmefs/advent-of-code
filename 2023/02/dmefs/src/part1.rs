pub fn solve(contents: &str) -> u32 {
    let mut res = 0;
    for line in contents.lines() {
        res += match game_possible(line) {
            Some(num) => num,
            None => 0,
        }
    }
    res
}

fn game_possible(line: &str) -> Option<u32> {
    let parts: Vec<_> = line.trim().split(':').collect();
    if game_check(parts[1]) {
        let num_str: Vec<&str> = parts[0].trim().split(' ').collect();
        return Some(num_str[1].parse::<u32>().unwrap());
    }
    None
}
fn game_check(game: &str) -> bool {
    for subset in game.trim().split(';') {
        let (r, g, b) = count_cubes(subset);
        if r > 12 || g > 13 || b > 14 {
            return false;
        }
    }
    true
}

fn count_cubes(subset: &str) -> (u32, u32, u32) {
    let (mut r, mut g, mut b) = (0, 0, 0);
    for cube_info in subset.trim().split(',') {
        let (count, color) = parse_cube_info(cube_info);
        match color {
            "red" => r += count,
            "green" => g += count,
            "blue" => b += count,
            _ => (),
        }
    }
    (r, g, b)
}

fn parse_cube_info(cube_info: &str) -> (u32, &str) {
    let info: Vec<&str> = cube_info.trim().split(' ').collect();
    (info[0].parse::<u32>().unwrap(), info[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split() {
        let l = "Game 1: 12 blue; 2 green, 13 blue, 19 red; 13 red, 3 green, 14 blue";
        let part: Vec<_> = l.split(':').collect();
        assert_eq!("Game 1", part[0]);
    }
    #[test]
    fn test_count_cube() {
        assert_eq!((8, 4, 4), count_cubes("8 red, 4 blue, 4 green"));
    }
    #[test]
    fn test_game_possible() {
        assert_eq!(None, game_possible("Game 100: 8 red, 4 blue, 4 green; 10 blue, 3 red, 4 green; 10 green, 4 red; 18 red, 9 blue, 2 green; 12 red, 4 green, 2 blue"));
    }
}
