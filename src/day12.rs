#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavAction {
  North(i32),
  South(i32),
  East(i32),
  West(i32),
  Forward(i32),
  Left,
  Right,
  TurnAround,
}

#[derive(Debug)]
pub struct Ship {
  x: i32,
  y: i32,
  dx: i32,
  dy: i32,
}

impl Ship {
  pub fn new_part1() -> Ship {
    Ship {
      x: 0,
      y: 0,
      dx: 1,
      dy: 0,
    }
  }

  pub fn new_part2() -> Ship {
    Ship {
      x: 0,
      y: 0,
      dx: 10,
      dy: -1,
    }
  }

  pub fn manhattan_distance(&self) -> i32 {
    self.x.abs() + self.y.abs()
  }

  pub fn navigate_action_part1(&mut self, action: &NavAction) {
    match action {
      NavAction::North(value) => self.y -= value,
      NavAction::South(value) => self.y += value,
      NavAction::East(value) => self.x += value,
      NavAction::West(value) => self.x -= value,
      NavAction::Forward(value) => {
        self.x += value * self.dx;
        self.y += value * self.dy;
      }
      NavAction::Left => {
        let dx = self.dy;
        let dy = -self.dx;
        self.dx = dx;
        self.dy = dy;
      }
      NavAction::Right => {
        let dx = -self.dy;
        let dy = self.dx;
        self.dx = dx;
        self.dy = dy;
      }
      NavAction::TurnAround => {
        self.dx = -self.dx;
        self.dy = -self.dy;
      }
    }
  }

  pub fn navigate_action_part2(&mut self, action: &NavAction) {
    match action {
      NavAction::North(value) => self.dy -= value,
      NavAction::South(value) => self.dy += value,
      NavAction::East(value) => self.dx += value,
      NavAction::West(value) => self.dx -= value,
      NavAction::Forward(value) => {
        self.x += value * self.dx;
        self.y += value * self.dy;
      }
      NavAction::Left => {
        let dx = self.dy;
        let dy = -self.dx;
        self.dx = dx;
        self.dy = dy;
      }
      NavAction::Right => {
        let dx = -self.dy;
        let dy = self.dx;
        self.dx = dx;
        self.dy = dy;
      }
      NavAction::TurnAround => {
        self.dx = -self.dx;
        self.dy = -self.dy;
      }
    }
  }

  pub fn navigate_part1(&mut self, actions: &[NavAction]) {
    for action in actions {
      self.navigate_action_part1(action);
    }
  }

  pub fn navigate_part2(&mut self, actions: &[NavAction]) {
    for action in actions {
      self.navigate_action_part2(action);
    }
  }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<NavAction> {
  input
    .lines()
    .map(|line| {
      let value = line[1..].parse::<i32>().unwrap();
      match line.chars().next().unwrap() {
        'N' => NavAction::North(value),
        'S' => NavAction::South(value),
        'E' => NavAction::East(value),
        'W' => NavAction::West(value),
        'L' => match value {
          90 => NavAction::Left,
          180 => NavAction::TurnAround,
          270 => NavAction::Right,
          _ => unreachable!(),
        },
        'R' => match value {
          90 => NavAction::Right,
          180 => NavAction::TurnAround,
          270 => NavAction::Left,
          _ => unreachable!(),
        },
        'F' => NavAction::Forward(value),
        _ => unreachable!(),
      }
    })
    .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(nav_actions: &[NavAction]) -> i32 {
  let mut ship = Ship::new_part1();
  ship.navigate_part1(nav_actions);
  ship.manhattan_distance()
}

#[aoc(day12, part2)]
pub fn solve_part2(nav_actions: &[NavAction]) -> i32 {
  let mut ship = Ship::new_part2();
  ship.navigate_part2(nav_actions);
  ship.manhattan_distance()
}

#[cfg(test)]
mod tests {
  use super::{input_generator, solve_part1, solve_part2, NavAction, Ship};

  #[test]
  fn test_day12_part1() {
    let actions = input_generator("F10\nN3\nF7\nR90\nF11");
    assert_eq!(solve_part1(&actions), 25);
  }

  #[test]
  fn test_day12_part2() {
    let actions = input_generator("F10\nN3\nF7\nR90\nF11");
    assert_eq!(solve_part2(&actions), 286);
  }

  #[test]
  fn test_day12_gen_input() {
    let action_str = "R270\nL270\nL180\nR180\nL90\nR90";
    let actions = vec![
      NavAction::Left,
      NavAction::Right,
      NavAction::TurnAround,
      NavAction::TurnAround,
      NavAction::Left,
      NavAction::Right,
    ];

    assert_eq!(input_generator(action_str), actions);
  }

  #[test]
  fn test_day12_part1_rotate_left() {
    let mut ship = Ship::new_part1();
    assert_eq!((ship.dx, ship.dy), (1, 0));
    ship.navigate_action_part1(&NavAction::Left);
    assert_eq!((ship.dx, ship.dy), (0, -1));
    ship.navigate_action_part1(&NavAction::Left);
    assert_eq!((ship.dx, ship.dy), (-1, 0));
    ship.navigate_action_part1(&NavAction::Left);
    assert_eq!((ship.dx, ship.dy), (0, 1));
    ship.navigate_action_part1(&NavAction::Left);
    assert_eq!((ship.dx, ship.dy), (1, 0));
  }

  #[test]
  fn test_day12_part1_rotate_right() {
    let mut ship = Ship::new_part1();
    assert_eq!((ship.dx, ship.dy), (1, 0));
    ship.navigate_action_part1(&NavAction::Right);
    assert_eq!((ship.dx, ship.dy), (0, 1));
    ship.navigate_action_part1(&NavAction::Right);
    assert_eq!((ship.dx, ship.dy), (-1, 0));
    ship.navigate_action_part1(&NavAction::Right);
    assert_eq!((ship.dx, ship.dy), (0, -1));
    ship.navigate_action_part1(&NavAction::Right);
    assert_eq!((ship.dx, ship.dy), (1, 0));
  }
}
