use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

pub fn perfectly_spherical_houses_in_a_vacuum(xs: &str) -> usize {
    let mut pos = Point::new(0, 0);
    let mut set = HashSet::new();
    set.insert(pos);
    for c in xs.chars() {
        match c {
            '>' => pos.x += 1,
            '<' => pos.x -= 1,
            '^' => pos.y += 1,
            'v' => pos.y -= 1,
            _ => unreachable!(),
        }
        set.insert(pos);
    }
    set.len()
}

pub fn perfectly_spherical_houses_in_a_vacuum_part_two(xs: &str) -> usize {
    let mut pos_s = Point::new(0, 0);
    let mut pos_r = Point::new(0, 0);
    let mut set = HashSet::new();
    set.insert(pos_s);
    for (i, c) in xs.chars().enumerate() {
        let pos = if i % 2 == 0 { &mut pos_s } else { &mut pos_r };
        match c {
            '>' => pos.x += 1,
            '<' => pos.x -= 1,
            '^' => pos.y += 1,
            'v' => pos.y -= 1,
            _ => unreachable!(),
        }
        set.insert(*pos);
    }
    set.len()
}
