use maze_runner_rs::maze::{Coordinates, Maze, MazeNode};
use maze_runner_rs::search::Searcher;
use maze_runner_rs::search::{a_star, bfs, dfs};
use maze_runner_rs::tilemap::{EmptyTileState, TileMap};
use std::collections::HashMap;
use std::rc::Rc;
use std::{env, fs};

use macroquad::prelude::*;

const STEP_DELAY: f64 = 0.;
const DRAW_DELAY: f64 = 1. / 24.;

// Static mutable variable to store the number of steps taken
static mut STEPS: u64 = 0;

#[macroquad::main("Maze Runner")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Get the file path from the command line arguments
    let Some(filepath) = args.get(1) else {
        eprintln!("A file must be provided");
        return;
    };

    // Get the algorithm from the command line arguments, defaulting to A*
    let algorithm_str = args.get(2).cloned().unwrap_or(String::from("a-star"));

    // Read the file into a string
    let Ok(tilemap_str) = fs::read_to_string(filepath) else {
        eprintln!("File not found: {filepath}");
        return;
    };

    // Parse the string into a TileMap object
    let Ok(tilemap): Result<TileMap, _> = tilemap_str.try_into() else {
        eprintln!("File is not a proper tilemap");
        return;
    };

    // Create a maze from the tilemap
    let maze: Rc<Maze> = Rc::new(tilemap.into());

    let mut done = false; // Whether the search is done
    let mut delta_time: f64 = 0f64; // Time since the last iteration of the loop

    // Timers for the step and draw delays
    let mut step_timer = 0.;
    let mut draw_timer = 0.;

    // Hashmap to store the state of empty tiles, for rendering only (Considering, Visited, Focused)
    let mut empty_tile_states: HashMap<Coordinates, EmptyTileState> = HashMap::new();

    // Define the searcher algorithm, based on the command line argument
    let mut searcher: Box<dyn Searcher> = match algorithm_str.as_str() {
        "dfs" => Box::new(dfs::DepthFirstSearcher::new(&maze)),
        "bfs" => Box::new(bfs::BreadthFirstSearcher::new(&maze)),
        "a-star" => {
            // Define the heuristic function for A* (Manhattan distance)
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

    // Eternal loop, as the program shall render until the user closes the window
    loop {
        let start_time = get_time();

        // Add the delta time to the step and draw timers
        step_timer += delta_time;
        draw_timer += delta_time;

        // Only step if the delay has passed and the search is not done
        if step_timer >= STEP_DELAY && !done {
            step_timer -= STEP_DELAY;
            done = step(&mut searcher, &mut empty_tile_states);
        }

        // Only render if the delay has passed
        // Note: this has no "done" condition, as otherwise the window would crash as soon as a solution was found
        if draw_timer >= DRAW_DELAY {
            draw_timer -= DRAW_DELAY * (draw_timer / DRAW_DELAY).floor();
            draw(&maze, &mut empty_tile_states);
            next_frame().await;
        }

        delta_time = get_time() - start_time;
    }
}


// Advances the search by one step
fn step(
    searcher: &mut Box<dyn Searcher>,
    empty_tile_states: &mut HashMap<Coordinates, EmptyTileState>,
) -> bool {
    // Get the next node to expand, otherwise raise an error message and stop the search
    let Some(path) = searcher.get_current_path().cloned() else {
        eprintln!("No path found");
        return true;
    };

    let Some(node) = searcher.next() else {
        eprintln!("No node left to expand");
        return true;
    };

    // If the selected node is the final node, the search is done
    if node.get_tile() == maze_runner_rs::tilemap::Tile::End {
        #[cfg(debug_assertions)]
        println!("Path found!");
        unsafe {
            println!(
                "Search done.\nNodes considered: {}\nLength of path found: {}",
                STEPS,
                path.iter().len()
            );
        }
        return true;
    }

    unsafe { STEPS += 1 };

    // The following scope is only relevant for rendering purposes, it does not affect the search itself
    {
        // Set all "Focused" nodes to "Visited" (Only relevant for rendering)
        empty_tile_states
            .iter_mut()
            .filter(|(_, state)| **state == EmptyTileState::Focused)
            .for_each(|(_, state)| *state = EmptyTileState::Visited);

        // Set all the nodes being considered to "Considering" (Only relevant for rendering)
        searcher.get_considered_nodes().iter().for_each(|node| {
            empty_tile_states.insert(node.get_coordinates(), EmptyTileState::Considering);
        });

        // Set the current path to "Focused" (Only relevant for rendering)
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
}

fn draw(maze: &Rc<Maze>, empty_tile_states: &mut HashMap<Coordinates, EmptyTileState>) {
    // Define the size of the tiles, based on the screen size and the maze size
    let tile_size = f32::min(
        screen_width() / maze.width() as f32,
        screen_height() / maze.height() as f32,
    );

    // Define the x and y offsets necessary to center the maze on the screen
    let x_offset = (screen_width() - (tile_size * maze.width() as f32)) / 2f32;
    let y_offset = (screen_height() - (tile_size * maze.height() as f32)) / 2f32;

    // Struct to store a streak of tiles with the same color, for faster rendering
    struct TileStreak {
        start_idx: usize,
        length: usize,
        color: Option<Color>,
    }

    for x_idx in 0..maze.width() {
        // Start a streak whenever a new column is rendered
        let mut streak: TileStreak = TileStreak {
            start_idx: 0,
            length: 0,
            color: Some(WHITE),
        };

        // The x-component of the position of the row
        let x_pos = x_offset + (x_idx as f32) * tile_size;
        for y_idx in 0..maze.height() {
            // The tile to render (Empty, Wall, Start or End)
            #[allow(clippy::expect_used)]
            let tile = maze
                .get_node((x_idx, y_idx))
                .expect("Empty node should not be accessible")
                .get_tile();

            // The color of the node, based on the tile and the state of the node
            let node_color: Option<Color> = match tile {
                maze_runner_rs::tilemap::Tile::Start => Some(YELLOW),
                maze_runner_rs::tilemap::Tile::End => Some(GREEN),
                maze_runner_rs::tilemap::Tile::Wall => Some(WHITE),
                maze_runner_rs::tilemap::Tile::Empty => {
                    match empty_tile_states.get(&(x_idx, y_idx)) {
                        None => None,
                        Some(EmptyTileState::Visited) => Some(SKYBLUE),
                        Some(EmptyTileState::Focused) => Some(ORANGE),
                        Some(EmptyTileState::Considering) => Some(RED),
                    }
                }
            };

            // Update the streak based on it's own color and the color of the node
            streak = match (streak.color, node_color) {
                // If the color is "None" (i.e. background color), update it to the new color always
                (None, new_color) => TileStreak {
                    start_idx: y_idx,
                    length: 1,
                    color: new_color,
                },
                // If the color is the same as the streak, increase the length of the streak
                (Some(streak_col), Some(node_col)) if streak_col == node_col => TileStreak {
                    start_idx: streak.start_idx,
                    length: streak.length + 1,
                    color: Some(streak_col),
                },
                // If the color is different from the streak, draw the streak and start a new one
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
                // This case should never happen
                (Some(_), _) => unreachable!("Invalid state"),
            };
        }

        // At the end of the column, draw the last streak
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
}
