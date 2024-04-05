#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::sync::Mutex;
use tauri::{Manager, PhysicalSize};

const WIDTH: usize = 30;
const HEIGHT: usize = 16;
const BOMBS: usize = 99;
const DELTAS: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

pub struct BoardState(pub Mutex<Board>);
#[derive(Serialize, Default, Clone, Copy, Debug)]
pub struct Board {
    board: [[Tile; WIDTH]; HEIGHT],
    state: CurrentGame,
    score: i32,
    bombs: i32,
}
#[derive(Clone, Copy, Serialize, Default, Debug, PartialEq)]
struct Tile {
    bomb: Bomb,
    revealed: bool,
    flagged: bool,
}
#[derive(Serialize, Clone, Debug, Copy, PartialEq, Eq, Default)]
enum CurrentGame {
    #[default]
    Starting,
    Ongoing,
    Win,
    Lose,
}
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
enum Bomb {
    IsBomb,
    Bombs(u8),
}
#[derive(Serialize, Clone, Copy, PartialEq, Debug)]
struct Point(i32, i32);

impl Default for Bomb {
    fn default() -> Self {
        Bomb::Bombs(0)
    }
}
impl Board {
    fn handle_interaction(&mut self, interaction: String, tile: [i32; 2]) {
        match interaction.as_str() {
            "left" => {
                self.reveal_tile(tile);
            }
            "middle" => {
                self.reveal_surrounding_tiles(tile);
            }
            "right" => {
                self.flag_tile(tile);
            }
            _ => {}
        };
    }

    fn generate_board(&mut self) {
        self.bombs = BOMBS as i32;
        let populate_bombs = |x: [Point; BOMBS]| {
            let mut new_board = [[Tile {
                bomb: Bomb::Bombs(0),
                revealed: false,
                flagged: false,
            }; 30]; 16];
            for bomb in x.iter() {
                let mut current_tile = new_board[bomb.1 as usize][bomb.0 as usize];
                current_tile.bomb = Bomb::IsBomb;
                new_board[bomb.1 as usize][bomb.0 as usize] = current_tile;
            }
            return new_board;
        };
        let generate_bombs = || {
            let mut bomb_count: i32 = 0;
            let mut current_bombs: [Point; BOMBS] = [Point(0, 0); BOMBS];
            while bomb_count < BOMBS as i32 {
                let current_bomb = bomb_count as usize;
                let random_index = Point(
                    thread_rng().gen_range(0..WIDTH) as i32,
                    thread_rng().gen_range(0..HEIGHT) as i32,
                );
                if current_bombs.contains(&random_index) {
                    continue;
                } else {
                    current_bombs[current_bomb] = random_index;
                    bomb_count = bomb_count + 1;
                }
            }
            return current_bombs;
        };
        let count_bombs = |board: [[Tile; 30]; 16]| {
            let mut final_board = board;

            for (row_index, row) in final_board.iter_mut().enumerate() {
                'tiles: for (col_index, tile) in row.iter_mut().enumerate() {
                    if tile.bomb == Bomb::IsBomb {
                        continue 'tiles;
                    }
                    let mut surrounding_bombs = 0;
                    'inner: for (x, y) in DELTAS {
                        if (row_index as i32 + y) > HEIGHT as i32 - 1
                            || (row_index as i32 + y) < 0
                            || (col_index as i32 + x) > WIDTH as i32 - 1
                            || (col_index as i32 + x) < 0
                        {
                            continue 'inner;
                        }
                        if board[(row_index as i32 + y) as usize][(col_index as i32 + x) as usize]
                            .bomb
                            == Bomb::IsBomb
                        {
                            surrounding_bombs += 1;
                        }
                    }
                    tile.bomb = Bomb::Bombs(surrounding_bombs);
                }
            }
            return final_board;
        };
        let generated_bombs: [Point; BOMBS] = generate_bombs();
        let populated_board = populate_bombs(generated_bombs);
        let complete_board = count_bombs(populated_board);
        self.state = CurrentGame::Ongoing; // Starting and Ongoing can prevent an instant loss when first revealed tile is a bomb
        self.board = complete_board;
    }
    fn reveal_tile(&mut self, tile: [i32; 2]) -> [[Tile; 30]; 16] {
        let mut current_board = self.board;
        let mut current_tile = current_board[tile[1] as usize][tile[0] as usize];

        if current_tile.flagged || current_tile.revealed {
            return self.board;
        }

        if current_tile.bomb == Bomb::IsBomb {
            current_tile.revealed = true;
            current_board[tile[1] as usize][tile[0] as usize] = current_tile;
            self.board = current_board;
            return self.board;
        }

        let mut check_tile_queue = Vec::new();
        check_tile_queue.push(Point(tile[0], tile[1]));

        'outer: while check_tile_queue.len() > 0 {
            let current_point = *check_tile_queue.first().unwrap();
            current_tile = current_board[current_point.1 as usize][current_point.0 as usize];

            if current_tile.bomb == Bomb::Bombs(0) {
                'inner: for (x, y) in DELTAS {
                    if current_point.1 as i32 + y > HEIGHT as i32 - 1
                        || current_point.1 as i32 + y < 0
                        || current_point.0 + x > WIDTH as i32 - 1
                        || current_point.0 + x < 0
                    {
                        continue 'inner;
                    }
                    if current_board[(current_point.1 + y) as usize][(current_point.0 + x) as usize]
                        .revealed
                        || check_tile_queue
                            .contains(&Point(&current_point.0 + x, &current_point.1 + y))
                    {
                        continue 'inner;
                    }
                    check_tile_queue.push(Point(current_point.0 + x, current_point.1 + y));
                }
                if current_tile.flagged {
                    current_tile.flagged = false
                }
            } else {
                if current_tile.flagged {
                    check_tile_queue.remove(0);
                    continue 'outer;
                }
            }
            current_tile.revealed = true;
            current_board[current_point.1 as usize][current_point.0 as usize] = current_tile;
            check_tile_queue.remove(0);
        }
        self.board = current_board;
        return self.board;
    }
    fn flag_tile(&mut self, tile: [i32; 2]) -> [[Tile; 30]; 16] {
        let mut current_board = self.board;
        let mut current_tile = current_board[tile[1] as usize][tile[0] as usize];

        if !current_tile.revealed {
            if current_tile.flagged {
                current_tile.flagged = false;
            } else {
                current_tile.flagged = true;
            }
        }

        current_board[tile[1] as usize][tile[0] as usize] = current_tile;

        self.board = current_board;
        return self.board;
    }
    fn reveal_surrounding_tiles(&mut self, tile: [i32; 2]) -> [[Tile; 30]; 16] {
        let mut current_board = self.board;
        let mut current_tile = current_board[tile[1] as usize][tile[0] as usize];

        if current_tile.flagged || !current_tile.revealed || current_tile.bomb == Bomb::Bombs(0) {
            return self.board;
        }
        let flags_needed = current_tile.bomb;
        let mut surrounding_flagged_tiles = 0;
        'inner_check: for (x, y) in DELTAS {
            if tile[1] + y > HEIGHT as i32 - 1
                || tile[1] + y < 0
                || tile[0] + x > WIDTH as i32 - 1
                || tile[0] + x < 0
            {
                continue 'inner_check;
            }
            current_tile = current_board[(tile[1] + y) as usize][(tile[0] + x) as usize];
            if current_tile.flagged {
                surrounding_flagged_tiles += 1;
            }
        }
        let flagged_tiles_satisfied = Bomb::Bombs(surrounding_flagged_tiles) == flags_needed;
        if flagged_tiles_satisfied {
            'inner_reveal: for (x, y) in DELTAS {
                if tile[1] + y > HEIGHT as i32 - 1
                    || tile[1] + y < 0
                    || tile[0] + x > WIDTH as i32 - 1
                    || tile[0] + x < 0
                {
                    continue 'inner_reveal;
                }
                current_tile = current_board[(tile[1] + y) as usize][(tile[0] + x) as usize];
                if current_tile.flagged {
                    continue 'inner_reveal;
                }
                if current_tile.revealed {
                    continue 'inner_reveal;
                }
                if current_tile.bomb == Bomb::Bombs(0) {
                    self.board = current_board;
                    current_board = self.reveal_tile([tile[0] + x, tile[1] + y]);
                }
                current_tile.revealed = true;
                current_board[(tile[1] + y) as usize][(tile[0] + x) as usize] = current_tile;
            }
        }
        self.board = current_board;
        return self.board;
    }
    fn check_game_state(&mut self) {
        let current_board = self.board;
        let mut current_bombs = BOMBS as i32;
        let mut current_game_state = self.state;

        let mut bomb_revealed = false;
        let mut not_bomb_tile_flagged = false;
        let mut game_won = false;
        let mut not_bomb_tiles_revealed = 0;
        let mut tiles_flagged: i32 = 0;

        current_board.map(|row| {
            row.map(|tile| {
                if tile.flagged {
                    if tile.bomb != Bomb::IsBomb {
                        not_bomb_tile_flagged = true;
                    }
                    tiles_flagged += 1;
                }
                if tile.revealed {
                    if tile.bomb == Bomb::IsBomb {
                        bomb_revealed = true;
                    } else {
                        not_bomb_tiles_revealed += 1;
                    }
                }
            });
        });

        if bomb_revealed {
            current_game_state = CurrentGame::Lose;
        } else {
            if tiles_flagged == BOMBS as i32 && !not_bomb_tile_flagged {
                game_won = true;
            }
            if not_bomb_tiles_revealed - tiles_flagged
                == HEIGHT as i32 * WIDTH as i32 - BOMBS as i32 - tiles_flagged
                && !not_bomb_tile_flagged
            {
                game_won = true;
            }
        }
        if game_won {
            current_game_state = CurrentGame::Win;
        }
        current_bombs -= tiles_flagged;
        self.bombs = current_bombs;
        self.state = current_game_state;
    }

    pub fn get_board(self) -> Board {
        return self;
    }
}

#[tauri::command]
fn generate_board(state: tauri::State<'_, BoardState>) -> Board {
    let mut current_board = state.0.lock().unwrap();
    current_board.generate_board();
    current_board.get_board()
}
#[tauri::command]
async fn tile_interaction(
    state: tauri::State<'_, BoardState>,
    interaction: String,
    tile: [i32; 2],
) -> Result<Board, String> {
    let mut current_board = state.0.lock().unwrap();
    current_board.handle_interaction(interaction, tile);
    current_board.check_game_state();
    Ok(current_board.get_board())
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let min_size = Some(PhysicalSize {
                width: 600,
                height: 400,
            });
            let max_size = Some(PhysicalSize {
                width: 1200,
                height: 800,
            });
            let window = app.get_window("main").unwrap();
            window.set_min_size(min_size).unwrap();
            window.set_max_size(max_size).unwrap();
            Ok(())
        })
        .manage(BoardState(Default::default()))
        .invoke_handler(tauri::generate_handler![generate_board, tile_interaction])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
