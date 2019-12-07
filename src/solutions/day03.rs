use std::collections::HashSet;
use std::cmp;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

pub fn a(input: &str) -> String {
  let paths = input.lines().collect::<Vec<&str>>();
  let path_one = parse_path(paths[0]);
  let path_two = parse_path(paths[1]);

  let first_wire_points = find_points(path_one);
  let second_wire_points = find_points(path_two);

  let mut distance = std::i32::MAX;
  for point in first_wire_points {
    if second_wire_points.contains(&point) {
      distance = cmp::min(distance, point.x.abs() + point.y.abs());
    }
  }

  distance.to_string()
}

pub fn b(_input: &str) -> String {
  String::from("")
}

fn parse_path(path: &str) -> Vec<(char, i32)> {
  path
    .split(",")
    .map(|s| (s.chars().next().unwrap(), s[1..].parse::<i32>().unwrap()))
    .collect::<Vec<(char, i32)>>()
}

fn find_points(wire_path: Vec<(char, i32)>) -> HashSet<Point> {
  let mut pos_x = 0;
  let mut pos_y = 0;
  let mut wire_points = HashSet::new();
  for path in wire_path {
    match path.0 {
      'L' => {
        for _ in 0..path.1 {
          pos_x -= 1;
          wire_points.insert(Point { x: pos_x, y: pos_y });
        }
      }
      'R' => {
        for _ in 0..path.1 {
          pos_x += 1;
          wire_points.insert(Point { x: pos_x, y: pos_y });
        }
      }
      'U' => {
        for _ in 0..path.1 {
          pos_y += 1;
          wire_points.insert(Point { x: pos_x, y: pos_y });
        }
      }
      'D' => {
        for _ in 0..path.1 {
          pos_y -= 1;
          wire_points.insert(Point { x: pos_x, y: pos_y });
        }
      }
      _ => unreachable!(),
    };
  }
  wire_points
}

#[test]
pub fn tests_a() {
  assert_eq!(
    a("R75,D30,R83,U83,L12,D49,R71,U7,L72
  U62,R66,U55,R34,D71,R55,D58,R83"),
    ""
  );
}

#[test]
fn tests_b() {
  assert_eq!(b(""), "");
}
