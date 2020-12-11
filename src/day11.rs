#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(seats: &Vec<Vec<char>>) -> usize {
    game_of_seats(seats, 4, 1)
}

#[aoc(day11, part2)]
pub fn solve_part2(seats: &Vec<Vec<char>>) -> usize {
    game_of_seats(seats, 5, std::u32::MAX)
}

fn game_of_seats(seats: &Vec<Vec<char>>, emptying_rule_count: u8, max_sight: u32) -> usize {
    let mut seats = seats.clone();
    let height = seats.len() as i32;
    let width = seats[0].len() as i32;
    let mut changed = true;
    let mut occupied_count = 0;
    while changed {
        let mut next_seats = seats.clone();
        changed = false;
        occupied_count = 0;
        for y in 0..height {
            for x in 0..width {
                let cell = seats[y as usize][x as usize];
                match cell {
                    '.' => continue,
                    '#' => occupied_count += 1,
                    _ => (),
                }
                let mut neighbours: u8 = 0;
                for dy in -1..=1 as i32 {
                    for dx in -1..=1 as i32 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let mut nx = x + dx;
                        let mut ny = y + dy;
                        let mut sight = 1;
                        while sight < max_sight
                            && nx >= 0
                            && ny >= 0
                            && nx < width
                            && ny < height
                            && seats[ny as usize][nx as usize] == '.'
                        {
                            nx += dx;
                            ny += dy;
                            sight += 1;
                        }

                        if nx >= 0
                            && ny >= 0
                            && nx < width
                            && ny < height
                            && seats[ny as usize][nx as usize] == '#'
                        {
                            neighbours += 1;
                        }
                    }
                }

                next_seats[y as usize][x as usize] = match cell {
                    'L' if neighbours == 0 => '#',
                    '#' if neighbours >= emptying_rule_count => 'L',
                    _ => cell,
                };

                changed |= next_seats[y as usize][x as usize] != cell;
            }
        }
        seats = next_seats;
    }

    occupied_count
}

#[cfg(test)]
mod tests {

    use super::{solve_part1, solve_part2};

    #[test]
    fn test_day11_part1() {
        let seat_layout: Vec<Vec<char>> = "L.LL.LL.LL\n\
        LLLLLLL.LL\n\
        L.L.L..L..\n\
        LLLL.LL.LL\n\
        L.LL.LL.LL\n\
        L.LLLLL.LL\n\
        ..L.L.....\n\
        LLLLLLLLLL\n\
        L.LLLLLL.L\n\
        L.LLLLL.LL"
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        assert_eq!(solve_part1(&seat_layout), 37);
    }

    #[test]
    fn test_day11_part2() {
        let seat_layout: Vec<Vec<char>> = "L.LL.LL.LL\n\
        LLLLLLL.LL\n\
        L.L.L..L..\n\
        LLLL.LL.LL\n\
        L.LL.LL.LL\n\
        L.LLLLL.LL\n\
        ..L.L.....\n\
        LLLLLLLLLL\n\
        L.LLLLLL.L\n\
        L.LLLLL.LL"
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        assert_eq!(solve_part2(&seat_layout), 26);
    }
}
