use maze_runner_rs::maze::{Coordinates, Maze, MazeNode};
use maze_runner_rs::search::Searcher;
use maze_runner_rs::search::{a_star, bfs, dfs};
use maze_runner_rs::tilemap::TileMap;
use std::collections::HashMap;
use std::rc::Rc;
use std::{env, fs};

use macroquad::prelude::*;

const STEP_DELAY: f64 = 0.;
const DRAW_DELAY: f64 = 1. / 120.;

#[derive(PartialEq, Eq, Clone, Copy)]
enum EmptyTileState {
    Focused,
    Visited,
    Considering,
}

#[macroquad::main("Maze Runner")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(filepath) = args.get(1) else {
        eprintln!("A file must be provided");
        return;
    };

    let algorithm_str = args.get(2).cloned().unwrap_or(String::from("a-star"));

    let Ok(tilemap_str) = fs::read_to_string(filepath) else {
        eprintln!("File not found: {filepath}");
        return;
    };

    let Ok(tilemap): Result<TileMap, _> = tilemap_str.try_into() else {
        eprintln!("File is not a proper tilemap");
        return;
    };

    let maze: Rc<Maze> = Rc::new(tilemap.into());

    let mut done = false;
    let mut new_time: f64;
    let mut delta_time: f64 = STEP_DELAY;

    let mut step_timer = 0.;
    let mut draw_timer = 0.;

    let mut empty_tile_states: HashMap<Coordinates, EmptyTileState> = HashMap::new();

    #[cfg(debug_assertions)]
    let mut steps = 0f64;
    #[cfg(debug_assertions)]
    let mut total_step_time = 0f64;

    let mut searcher: Box<dyn Searcher> = match algorithm_str.as_str() {
        "dfs" => Box::new(dfs::DepthFirstSearcher::new(maze.clone())),
        "bfs" => Box::new(bfs::BreadthFirstSearcher::new(maze.clone())),
        "a-star" => {
            let heuristic = Box::new(|node: &MazeNode, end_node: &MazeNode| {
                (Maze::manhattan_distance(node.get_coordinates(), end_node.get_coordinates()))
                    as u64
            });
            Box::new(a_star::AStarSearcher::new(maze.clone(), heuristic))
        }
        _ => {
            eprintln!(
                "Invalid algorithm: Algorithm must be [\"dfs\" | \"bfs\" | \"a-star\"]. \"{}\" is not a valid algorithm",
                algorithm_str
            );
            return;
        }
    };

    loop {
        new_time = get_time();

        step_timer += delta_time;
        if step_timer >= STEP_DELAY && !done {
            step_timer = 0.;
            done = step(&mut searcher, &mut empty_tile_states);
            #[cfg(debug_assertions)]
            {
                steps += 1f64;
                total_step_time += get_time() - new_time;
                if done {
                    #[cfg(debug_assertions)]
                    println!("Average step time: {:?}", total_step_time / steps)
                }
            }
        }

        draw_timer += delta_time;
        if draw_timer >= DRAW_DELAY {
            draw_timer = 0.;
            draw(&maze, &mut empty_tile_states).await;
        }

        delta_time = get_time() - new_time;
    }
}

fn step(
    searcher: &mut Box<dyn Searcher>,
    empty_tile_states: &mut HashMap<Coordinates, EmptyTileState>,
) -> bool {
    let Some(node) = searcher.next() else {
        eprintln!("No node left to expand");
        return false;
    };

    if node.get_tile() == maze_runner_rs::tilemap::Tile::End {
        #[cfg(debug_assertions)]
        println!("Path found!");

        return true;
    }

    empty_tile_states
        .iter_mut()
        .filter(|(_, state)| **state == EmptyTileState::Focused)
        .for_each(|(_, state)| *state = EmptyTileState::Visited);

    searcher.get_considered_nodes().iter().for_each(|node| {
        empty_tile_states.insert(node.get_coordinates(), EmptyTileState::Considering);
    });

    if let Some(current_path) = searcher.get_current_path() {
        empty_tile_states.extend(
            current_path
                .iter()
                .map(MazeNode::get_coordinates)
                .zip(std::iter::repeat(EmptyTileState::Focused)),
        );
        false
    } else {
        eprintln!("No path found");
        true
    }
}

async fn draw(maze: &Rc<Maze>, empty_tile_states: &mut HashMap<Coordinates, EmptyTileState>) {
    let tile_size = f32::min(
        screen_width() / maze.width() as f32,
        screen_height() / maze.height() as f32,
    );

    let x_offset = (screen_width() - (tile_size * maze.width() as f32)) / 2f32;
    let y_offset = (screen_height() - (tile_size * maze.height() as f32)) / 2f32;

    struct TileStreak {
        start_idx: usize,
        length: usize,
        color: Option<Color>,
    }

    for x_idx in 0..maze.width() {
        let mut streak: TileStreak = TileStreak {
            start_idx: 0,
            length: 1,
            color: None,
        };

        for y_idx in 0..maze.height() {
            let x_pos = x_offset + (x_idx as f32) * tile_size;

            #[allow(clippy::expect_used)]
            let tile = maze
                .get_node((x_idx, y_idx))
                .expect("Empty node should not be accessible")
                .get_tile();

            let node_color: Option<Color> = match tile {
                maze_runner_rs::tilemap::Tile::Empty => {
                    match empty_tile_states.get(&(x_idx, y_idx)) {
                        None => None,
                        Some(EmptyTileState::Visited) => Some(SKYBLUE),
                        Some(EmptyTileState::Focused) => Some(ORANGE),
                        Some(EmptyTileState::Considering) => Some(RED),
                    }
                }
                maze_runner_rs::tilemap::Tile::Wall => Some(WHITE),
                maze_runner_rs::tilemap::Tile::Start => Some(YELLOW),
                maze_runner_rs::tilemap::Tile::End => Some(GREEN),
            };

            streak = match (streak.color, node_color) {
                (None, new_color) => TileStreak {
                    start_idx: y_idx,
                    length: 1,
                    color: new_color,
                },
                (Some(streak_col), Some(node_col)) if streak_col == node_col => TileStreak {
                    start_idx: streak.start_idx,
                    length: streak.length + 1,
                    color: Some(streak_col),
                },
                (Some(streak_col), node_col_opt) if Some(streak_col) != node_col_opt => {
                    let org_y = y_offset + streak.start_idx as f32 * tile_size;
                    draw_rectangle(
                        x_pos,
                        org_y,
                        tile_size,
                        tile_size * streak.length as f32,
                        streak_col,
                    );

                    TileStreak {
                        start_idx: y_idx,
                        length: 1,
                        color: node_col_opt,
                    }
                }
                (Some(_), _) => unreachable!("Invalid state"),
            };
        }

        if let Some(color) = streak.color {
            let org_y = y_offset + streak.start_idx as f32 * tile_size;
            let height = tile_size * streak.length as f32;
            draw_rectangle(
                x_offset + (x_idx as f32) * tile_size,
                org_y,
                tile_size,
                height,
                color,
            );
        }
    }

    next_frame().await;
}
