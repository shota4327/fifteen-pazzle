use iced::widget::{Button, Column, Container, Row, Text, button};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription, executor, keyboard,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{Duration, Instant};

fn main() -> iced::Result {
    Puzzle::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TileClicked(usize, usize),
    Move(Direction),
    NewGame,
}

enum GameState {
    Playing,
    Solved(Duration),
}

struct Puzzle {
    board: [[i32; BOARD_SIZE]; BOARD_SIZE],
    empty_pos: (usize, usize),
    state: GameState,
    start_time: Instant,
}

impl Application for Puzzle {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let board = make_pazzle();
        let empty_pos = find_zero(&board).expect("Board must have a zero");
        (
            Puzzle {
                board,
                empty_pos,
                state: GameState::Playing,
                start_time: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("15 Puzzle")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        if let GameState::Playing = self.state {
            match message {
                Message::TileClicked(y, x) => {
                    let (ey, ex) = self.empty_pos;
                    let is_adjacent = (y == ey && (x as isize - ex as isize).abs() == 1)
                        || (x == ex && (y as isize - ey as isize).abs() == 1);

                    if is_adjacent {
                        self.swap_with_empty(y, x);
                    }
                }
                Message::Move(direction) => {
                    let (ey, ex) = self.empty_pos;
                    // ユーザーが動かしたいタイルの方向から、対象タイルの座標を計算
                    let (dy, dx) = direction.opposite().delta();
                    let source_y = ey as isize + dy;
                    let source_x = ex as isize + dx;

                    // 盤面の範囲内なら移動を実行
                    if source_y >= 0
                        && source_y < BOARD_SIZE as isize
                        && source_x >= 0
                        && source_x < BOARD_SIZE as isize
                    {
                        self.swap_with_empty(source_y as usize, source_x as usize);
                    }
                }
                Message::NewGame => {
                    *self = Self::new(()).0;
                }
            }
        } else {
            // GameState::Solved
            if let Message::NewGame = message {
                *self = Self::new(()).0;
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        keyboard::on_key_press(|key, _modifiers| {
            // `key`が名前付きのキー（矢印キーなど）の場合に`Direction`に変換する
            if let keyboard::Key::Named(named_key) = key {
                let direction = match named_key {
                    keyboard::key::Named::ArrowUp => Some(Direction::Up),
                    keyboard::key::Named::ArrowDown => Some(Direction::Down),
                    keyboard::key::Named::ArrowLeft => Some(Direction::Left),
                    keyboard::key::Named::ArrowRight => Some(Direction::Right),
                    _ => None,
                };
                return direction.map(Message::Move);
            }
            None
        })
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let board_view = (0..BOARD_SIZE).fold(Column::new().spacing(5), |col, y| {
            let row_view = (0..BOARD_SIZE).fold(Row::new().spacing(5), |row, x| {
                let tile_content = self.board[y][x];

                let button = if tile_content == 0 {
                    // 空白タイルはボタンではないように見せる
                    button(Text::new("")).width(60).height(60)
                } else {
                    button(Text::new(tile_content.to_string()).size(24))
                        .width(60)
                        .height(60)
                        .on_press(Message::TileClicked(y, x))
                };
                row.push(button)
            });
            col.push(row_view)
        });

        let mut content = Column::new()
            .push(board_view)
            .spacing(20)
            .align_items(Alignment::Center);

        if let GameState::Solved(duration) = self.state {
            let seconds = duration.as_secs();
            let millis = duration.subsec_millis();
            let time_str = format!("Clear Time: {:02}:{:02}.{:03}", seconds / 60, seconds % 60, millis);

            content = content.push(Text::new("Congratulations! You solved it!").size(28))
                             .push(Text::new(time_str).size(24));
        }

        content = content.push(
            Button::new(Text::new("New Game").size(20))
                .padding(10)
                .on_press(Message::NewGame),
        );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl Puzzle {
    /// 指定された座標のタイルと空白タイルを入れ替える
    fn swap_with_empty(&mut self, y: usize, x: usize) {
        let (ey, ex) = self.empty_pos;
        self.board[ey][ex] = self.board[y][x];
        self.board[y][x] = 0;
        self.empty_pos = (y, x);

        if self.board == SOLVED_PANEL {
            let clear_time = self.start_time.elapsed();
            self.state = GameState::Solved(clear_time);
        }
    }
}
/// パネル上の移動方向を示す列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// 方向を座標の移動量 (dy, dx) に変換する
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    /// 反対方向を返す
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

const BOARD_SIZE: usize = 4;
const SOLVED_PANEL: [[i32; BOARD_SIZE]; BOARD_SIZE] =
    [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]];

fn make_pazzle() -> [[i32; BOARD_SIZE]; BOARD_SIZE] {
    let mut board = SOLVED_PANEL;

    // 空白パネル(0)の現在位置。最初は右下からスタート
    let mut empty_y: usize = 3;
    let mut empty_x: usize = 3;

    // 直前の動きを記憶し、無駄な往復を防ぐ
    let mut last_move: Option<Direction> = None;
    let mut rng = thread_rng();

    // 1000回シャッフルする
    for _ in 0..1000 {
        // 1. 現在地から物理的に移動可能な方向をすべてリストアップする
        let mut possible_moves = Vec::new();
        if empty_y > 0 {
            possible_moves.push(Direction::Up);
        } // 上に動ける
        if empty_y < BOARD_SIZE - 1 {
            possible_moves.push(Direction::Down);
        } // 下に動ける
        if empty_x > 0 {
            possible_moves.push(Direction::Left);
        } // 左に動ける
        if empty_x < BOARD_SIZE - 1 {
            possible_moves.push(Direction::Right);
        } // 右に動ける

        // 2. 直前の動きと逆方向の動きを候補から除外する
        let opposite_of_last = last_move.map(|d| d.opposite());
        let valid_moves: Vec<_> = possible_moves
            .into_iter()
            .filter(|&dir| Some(dir) != opposite_of_last)
            .collect();

        // 3. 動ける方向がなければ、この回のシャッフルはスキップ
        if valid_moves.is_empty() {
            continue;
        }

        // 4. 有効な動きの中からランダムに1つ選んで実行する
        let chosen_direction = *valid_moves.choose(&mut rng).unwrap();
        let (dy, dx) = chosen_direction.delta();
        let target_y = (empty_y as isize + dy) as usize;
        let target_x = (empty_x as isize + dx) as usize;

        // パネルの数字を入れ替え、空白の位置を更新する
        board[empty_y][empty_x] = board[target_y][target_x];
        board[target_y][target_x] = 0;
        empty_y = target_y;
        empty_x = target_x;
        last_move = Some(chosen_direction);
    }

    board
}

// パネルから空白(0)の位置を探す
fn find_zero(panel: &[[i32; BOARD_SIZE]; BOARD_SIZE]) -> Option<(usize, usize)> {
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if panel[y][x] == 0 {
                return Some((y, x));
            }
        }
    }
    None
}
