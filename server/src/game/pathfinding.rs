use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, VecDeque};
use std::usize;

use common::GameCoord;
use common::r#const::{MAP_COLS, MAP_ROWS};

#[derive(Clone)]
struct Node {
    coord: GameCoord,
    prev: Option<GameCoord>,
    g: u16,
    f: u16,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f.cmp(&other.f).then_with(|| self.g.cmp(&other.g))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f && self.g == other.g
    }
}

impl Eq for Node {}

#[allow(dead_code)]
pub fn a_star(
    start: GameCoord,
    end: GameCoord,
    obstacles: &Vec<Vec<bool>>,
) -> Option<VecDeque<GameCoord>> {
    fn chebyshev(p1: GameCoord, p2: GameCoord) -> u16 {
        p1.x.abs_diff(p2.x).max(p1.y.abs_diff(p2.y)) as u16
    }
    fn trace_path(
        end_node: Node,
        closed_list: &mut HashMap<GameCoord, Node>,
    ) -> VecDeque<GameCoord> {
        let mut path = VecDeque::new();
        let mut curr_node = end_node;
        while let Some(prev_coord) = curr_node.prev {
            path.push_front(curr_node.coord);
            curr_node = closed_list.remove(&prev_coord).unwrap();
        }
        path.push_front(curr_node.coord);
        path
    }

    if obstacles[start.y][start.x] || obstacles[end.y][end.x] {
        return None;
    }

    let mut open_ord_list = BinaryHeap::new();
    let mut open_list = HashMap::new();
    let mut closed_list = HashMap::new();
    let start_node = Node {
        coord: start,
        prev: None,
        g: 0,
        f: chebyshev(start, end),
    };

    open_ord_list.push(Reverse(start_node.clone()));
    open_list.insert(start, start_node);

    while open_ord_list.len() != 0 {
        let Reverse(current) = open_ord_list.pop().unwrap();
        let current_coord = current.coord;
        let _ = open_list.remove(&current_coord);

        let current_g = current.g;
        closed_list.insert(current_coord, current);
        for new_x in
            current_coord.x.saturating_sub(1)..=current_coord.x.saturating_add(1).min(MAP_COLS - 1)
        {
            for new_y in current_coord.y.saturating_sub(1)
                ..=current_coord.y.saturating_add(1).min(MAP_ROWS - 1)
            {
                let new_coord = GameCoord { x: new_x, y: new_y };
                if new_coord == current_coord {
                    continue;
                }
                if obstacles[new_coord.y][new_coord.x] {
                    continue;
                }
                let new_node = Node {
                    coord: new_coord,
                    prev: Some(current_coord),
                    g: current_g + 1,
                    f: chebyshev(new_coord, end) + current_g + 1,
                };
                if new_coord == end {
                    return Some(trace_path(new_node, &mut closed_list));
                }
                if let Some(node) = open_list.get(&new_coord) {
                    if *node <= new_node {
                        continue;
                    }
                }
                if let Some(node) = closed_list.get(&new_coord) {
                    if *node <= new_node {
                        continue;
                    }
                }
                open_ord_list.push(Reverse(new_node.clone()));
                open_list.insert(new_coord, new_node);
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn bds<const M: usize, const N: usize>(
    start: GameCoord,
    end: GameCoord,
    obstacles: &Vec<Vec<bool>>,
) -> Option<VecDeque<GameCoord>> {
    let mut forw_visited: Vec<Vec<bool>> = vec![vec![false; N]; M];
    let mut back_visited: Vec<Vec<bool>> = vec![vec![false; N]; M];

    let mut forw_parents: Vec<Vec<Option<GameCoord>>> = vec![vec![None; N]; M];
    let mut back_parents: Vec<Vec<Option<GameCoord>>> = vec![vec![None; N]; M];

    let mut forw_queue: VecDeque<GameCoord> = VecDeque::new();
    let mut back_queue: VecDeque<GameCoord> = VecDeque::new();

    forw_queue.push_back(start);
    forw_visited[start.y][start.x] = true;

    back_queue.push_back(end);
    back_visited[end.y][end.x] = true;

    // Returns false if there's ODOO magic.
    fn process_neightbours<const M: usize, const N: usize>(
        queue: &mut VecDeque<GameCoord>,
        visited: &mut Vec<Vec<bool>>,
        parents: &mut Vec<Vec<Option<GameCoord>>>,
        obstacles: &Vec<Vec<bool>>,
    ) -> bool {
        let current = match queue.pop_back() {
            Some(value) => value,
            None => return false,
        };

        // Adding to the queue the neigthbours
        for row in current.y.saturating_sub(1)..=usize::min(current.y + 1, M - 1) {
            for col in current.x.saturating_sub(1)..=usize::min(current.x + 1, N - 1) {
                if visited[row][col] || obstacles[row][col] {
                    continue;
                }
                visited[row][col] = true;
                parents[row][col] = Some(current);
                queue.push_front(GameCoord { x: col, y: row });
            }
        }
        true
    }

    fn is_intersecting<const M: usize, const N: usize>(
        visited1: &Vec<Vec<bool>>,
        visited2: &Vec<Vec<bool>>,
    ) -> Option<GameCoord> {
        for row in 0..M {
            for col in 0..N {
                if visited1[row][col] && visited2[row][col] {
                    return Some(GameCoord { x: col, y: row });
                }
            }
        }
        None
    }

    let mut path = VecDeque::new();

    while !forw_queue.is_empty() && !back_queue.is_empty() {
        if let Some(intersection) = is_intersecting::<M, N>(&forw_visited, &back_visited) {
            let mut current_opt = Some(intersection);
            while let Some(current_node) = current_opt {
                path.push_front(current_node);
                if current_node == start {
                    break;
                }
                current_opt = forw_parents[current_node.y][current_node.x];
            }

            current_opt = back_parents[intersection.y][intersection.x];
            while let Some(current_node) = current_opt {
                path.push_back(current_node);
                if current_node == end {
                    break;
                }
                current_opt = back_parents[current_node.y][current_node.x];
            }

            return Some(path);
        }

        if !process_neightbours::<M, N>(
            &mut forw_queue,
            &mut forw_visited,
            &mut forw_parents,
            obstacles,
        ) {
            return None;
        };
        if !process_neightbours::<M, N>(
            &mut back_queue,
            &mut back_visited,
            &mut back_parents,
            obstacles,
        ) {
            return None;
        };
    }
    Some(path)
}
