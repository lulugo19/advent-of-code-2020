#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeatState {
    Occupied,
    Empty,
    Floor,
}

#[derive(Clone)]
pub struct SeatLayout {
    rows: i32,
    cols: i32,
    seats: Vec<SeatState>,
}

impl SeatLayout {
    fn get(&self, x: i32, y: i32) -> Option<SeatState> {
        if x < 0 || y < 0 {
            None
        } else {
            self.seats.get((y * self.cols + x) as usize).copied()
        }
    }

    fn set(&mut self, x: usize, y: usize, seat_state: SeatState) {
        self.seats[y * self.cols as usize + x] = seat_state;
    }

    fn swap(&mut self, other: &mut SeatLayout) {
        std::mem::swap(self, other)
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> SeatLayout {
    let seats = input
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '#' => SeatState::Occupied,
            'L' => SeatState::Empty,
            '.' => SeatState::Floor,
            _ => panic!("Invalid seat char!"),
        })
        .collect();

    let cols = input.lines().next().unwrap().chars().count() as i32;
    let rows = input.lines().count() as i32;

    SeatLayout { rows, cols, seats }
}

#[aoc(day11, part1)]
pub fn solve_part1(seat_layout: &SeatLayout) -> usize {
    game_of_seats(seat_layout, 4, 1)
}

#[aoc(day11, part2)]
pub fn solve_part2(seat_layout: &SeatLayout) -> usize {
    game_of_seats(seat_layout, 5, std::u32::MAX)
}

fn game_of_seats(seats: &SeatLayout, emptying_rule_count: u8, max_sight: u32) -> usize {
    let mut seats = seats.clone();
    let mut next_seats = seats.clone();
    let mut changed = true;
    let mut occupied_count = 0;
    while changed {
        changed = false;
        occupied_count = 0;
        for y in 0..seats.rows {
            for x in 0..seats.cols {
                let cell = seats.get(x, y).unwrap();
                match cell {
                    SeatState::Empty => continue,
                    SeatState::Occupied => occupied_count += 1,
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
                        while sight < max_sight {
                            if seats.get(nx, ny) != Some(SeatState::Floor) {
                                break;
                            }
                            nx += dx;
                            ny += dy;
                            sight += 1;
                        }

                        if seats.get(nx, ny) == Some(SeatState::Occupied) {
                            neighbours += 1;
                        }
                    }
                }

                next_seats.set(
                    x as usize,
                    y as usize,
                    match cell {
                        SeatState::Empty if neighbours == 0 => SeatState::Occupied,
                        SeatState::Occupied if neighbours >= emptying_rule_count => {
                            SeatState::Empty
                        }
                        _ => cell,
                    },
                );

                changed |= next_seats.get(x, y).unwrap() != cell;
            }
        }
        seats.swap(&mut next_seats);
    }

    occupied_count
}

#[cfg(test)]
mod tests {

    use super::{input_generator, solve_part1, solve_part2};

    const TEST_INPUT: &'static str = "L.LL.LL.LL\n\
    LLLLLLL.LL\n\
    L.L.L..L..\n\
    LLLL.LL.LL\n\
    L.LL.LL.LL\n\
    L.LLLLL.LL\n\
    ..L.L.....\n\
    LLLLLLLLLL\n\
    L.LLLLLL.L\n\
    L.LLLLL.LL";

    #[test]
    fn test_day11_part1() {
        let seat_layout = input_generator(TEST_INPUT);
        assert_eq!(solve_part1(&seat_layout), 37);
    }

    #[test]
    fn test_day11_part2() {
        let seat_layout = input_generator(TEST_INPUT);
        assert_eq!(solve_part2(&seat_layout), 26);
    }
}
