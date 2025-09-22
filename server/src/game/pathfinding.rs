use std::collections::VecDeque;
use std::usize;

use common::GameCoord;

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
