use maze_runner_rs::maze::{Maze, MazeNode};
use maze_runner_rs::search::a_star::AStarSearcher;
use maze_runner_rs::search::bfs::BreadthFirstSearcher;
use maze_runner_rs::search::dfs::DepthFirstSearcher;
use maze_runner_rs::search::path::Path;
use maze_runner_rs::search::Searcher;
use maze_runner_rs::tilemap::TileMap;
use std::collections::HashMap;
use std::{env, fs};

use macroquad::prelude::*;

const STEP_DELAY: f64 = 0f64;
const DRAW_DELAY: f64 = 1f64 / 60f64;

#[derive(PartialEq, Eq)]
enum EmptyTileState {
    Focused,
    Visited,
}

#[macroquad::main("Maze Runnerc")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(filepath) = args.get(1) else {
        eprintln!("A file must be provided");
        return;
    };
    
    let algorithm_str = args.get(2).cloned().unwrap_or(String::from("dfs"));

    let Ok(tilemap_str) = fs::read_to_string(filepath) else {
        eprintln!("File not found: {filepath}");
        return;
    };

    let Ok(tilemap): Result<TileMap, _> = tilemap_str.try_into() else {
        eprintln!("File is not a proper tilemap");
        return;
    };

    let maze: Maze = tilemap.into();

    let mut done = false;
    let mut new_time: f64;
    let mut delta_time: f64 = 1f64 / 60f64;

    let mut step_time = 0f64;
    let mut draw_time = 0f64;

    let mut empty_tile_states: HashMap<(usize, usize), EmptyTileState> = HashMap::new();

    let end_node = maze.get_end().clone();
    #[allow(clippy::expect_used)]
    let mut searcher = {
        let mut path = Path::new();
        path.push(maze.get_start());
        
        #[allow(clippy::expect_used)]
        let algorithm: Box<dyn Searcher> = match algorithm_str.as_str() {
            "dfs" => Box::new(DepthFirstSearcher::new(path)),
            "bfs" => Box::new(BreadthFirstSearcher::new(path)),
            "a-star" => {
                let heuristic: Box<dyn Fn(&MazeNode) -> u64> = Box::new(
                    |node| (usize::abs_diff(
                        node.get_coordinates().0, end_node.get_coordinates().0
                    ) + usize::abs_diff(
                        node.get_coordinates().1, end_node.get_coordinates().1
                    )) as u64
                );
                Box::new(AStarSearcher::new(path, heuristic).expect("Failed to create A* searcher"))
            },
            _ => {
                eprintln!("Invalid algorithm");
                return;
            }
        };
        algorithm
    };

    loop {
        new_time = get_time();

        // Step
        step_time += delta_time;
        if step_time >= STEP_DELAY && !done {
            
            #[cfg(debug_assertions)]
            let step_start_time = get_time();
            
            step_time = 0f64;

            let Some(node) = searcher.next() else {
                panic!("No more nodes to search");
            };

            #[allow(clippy::expect_used)]
            if node.get_tile() != maze_runner_rs::tilemap::Tile::End {
                empty_tile_states.iter_mut().for_each(|(_, state)| {
                    if *state == EmptyTileState::Focused {
                        *state = EmptyTileState::Visited;
                    }
                });
                
                searcher.get_current_path().expect("No more paths").iter().for_each(|node| {
                    empty_tile_states.insert(node.get_coordinates(), EmptyTileState::Focused);
                });
            } else {
                done = true;
                
                #[cfg(debug_assertions)]
                debug!("Path found!");
            }
            
            #[cfg(debug_assertions)]
            debug!("Step time: {}", get_time() - step_start_time);
        }

        // Draw
        draw_time += delta_time;
        if draw_time >= DRAW_DELAY {
            // O(1)
            
            #[cfg(debug_assertions)]
            let render_start_time = get_time();
            
            draw_time = 0f64;
            // clear_background(BLACK);
            let tile_size = f32::min(
                screen_width() / maze.width() as f32,
                screen_height() / maze.height() as f32,
            );

            let x_offset = (screen_width() - (tile_size * maze.width() as f32)) / 2f32;
            let y_offset = (screen_height() - (tile_size * maze.height() as f32)) / 2f32;

            // O(n^2)
            for x_idx in 0..maze.width() {
                for y_idx in 0..maze.height() {
                    let x_pos = x_offset + (x_idx as f32) * tile_size;
                    let y_pos = y_offset + (y_idx as f32) * tile_size;

                    #[allow(clippy::expect_used)]
                    let tile = maze
                        .get_node((x_idx, y_idx))
                        .expect("Empty node should not be accessible")
                        .get_tile();

                    let color = match tile {
                        maze_runner_rs::tilemap::Tile::Empty => {
                            match empty_tile_states.get(&(x_idx, y_idx)) {
                                None => continue,
                                Some(EmptyTileState::Visited) => DARKBLUE,
                                Some(EmptyTileState::Focused) => SKYBLUE,
                            }
                        }
                        maze_runner_rs::tilemap::Tile::Wall => continue,
                        maze_runner_rs::tilemap::Tile::Start => BLUE,
                        maze_runner_rs::tilemap::Tile::End => GREEN,
                    };

                    draw_rectangle(x_pos, y_pos, tile_size, tile_size, color);
                }
            }

            next_frame().await;
            
            #[cfg(debug_assertions)]
            debug!("Render time: {}", get_time() - render_start_time);
        }
        delta_time = get_time() - new_time;
    }
}
