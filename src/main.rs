use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::mem;

fn main() {
    let panel = make_pazzle();
    play_game(panel);
}

// ゲームをプレイするメインの関数
fn play_game(mut panel: [[i32; BOARD_SIZE]; BOARD_SIZE]) {
    // ゲーム開始時の空白ピースの位置を一度だけ探す
    let (mut empty_y, mut empty_x) = find_zero(&panel).expect("Panel must have a zero");

    loop {
        println!("\n--- 15 Puzzle ---");
        show_panel(panel);

        if is_cleared(panel) {
            println!("\nCongratulations! You solved the puzzle!");
            break;
        }

        print!("\nMove piece (1:Up, 2:Down, 3:Left, 4:Right, 0:Quit): ");
        // プロンプトをすぐ表示するためにflushする
        io::stdout().flush().expect("flush failed!");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("-> Failed to read input.");
            continue;
        }

        // ユーザーの入力を解釈し、移動を試みる
        let move_result = match input.trim().parse::<u32>() {
            // ユーザーの入力は「動かしたいピースの方向」なので、空白の移動方向は逆になる
            Ok(1) => try_move(&mut panel, empty_y, empty_x, Direction::Down), // 上に動かす -> 空白は下に
            Ok(2) => try_move(&mut panel, empty_y, empty_x, Direction::Up),   // 下に動かす -> 空白は上に
            Ok(3) => try_move(&mut panel, empty_y, empty_x, Direction::Right),// 左に動かす -> 空白は右に
            Ok(4) => try_move(&mut panel, empty_y, empty_x, Direction::Left), // 右に動かす -> 空白は左に
            Ok(0) => {
                println!("Quitting the game.");
                break;
            }
            _ => {
                println!("-> Invalid input. Please enter a number from 0 to 4.");
                continue;
            }
        };

        // 移動が成功した場合、空白の位置を更新
        if let Some((next_y, next_x)) = move_result {
            empty_y = next_y;
            empty_x = next_x;
        }
    }
}

/// ピースの移動を試み、成功した場合は新しい空白の位置を返す
fn try_move(panel: &mut [[i32; BOARD_SIZE]; BOARD_SIZE], y: usize, x: usize, dir: Direction) -> Option<(usize, usize)> {
    let (dy, dx) = dir.delta();
    let target_y_isize = y as isize + dy;
    let target_x_isize = x as isize + dx;

    // 境界チェック
    if target_y_isize < 0 || target_y_isize >= BOARD_SIZE as isize || target_x_isize < 0 || target_x_isize >= BOARD_SIZE as isize {
        println!("-> Cannot move in that direction.");
        return None;
    }

    let target_y = target_y_isize as usize;
    let target_x = target_x_isize as usize;

    // イディオマティックなswap
    if y == target_y { // 左右の移動
        panel[y].swap(x, target_x);
    } else { // 上下の移動
        let (y1, y2) = (y.min(target_y), y.max(target_y));
        let (s1, s2) = panel.split_at_mut(y2);
        mem::swap(&mut s1[y1][x], &mut s2[0][x]);
    }

    Some((target_y, target_x))
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
const SOLVED_PANEL: [[i32; BOARD_SIZE]; BOARD_SIZE] = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]];

fn make_pazzle() -> [[i32; BOARD_SIZE]; BOARD_SIZE] {
    let mut panel = SOLVED_PANEL;

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
        if empty_y > 0 { possible_moves.push(Direction::Up); } // 上に動ける
        if empty_y < BOARD_SIZE - 1 { possible_moves.push(Direction::Down); } // 下に動ける
        if empty_x > 0 { possible_moves.push(Direction::Left); } // 左に動ける
        if empty_x < BOARD_SIZE - 1 { possible_moves.push(Direction::Right); } // 右に動ける

        // 2. 直前の動きと逆方向の動きを候補から除外する
        let opposite_of_last = last_move.map(|d| d.opposite());
        let valid_moves: Vec<_> = possible_moves.into_iter()
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
        panel[empty_y][empty_x] = panel[target_y][target_x];
        panel[target_y][target_x] = 0;
        empty_y = target_y;
        empty_x = target_x;
        last_move = Some(chosen_direction);
    }

    panel

}

fn show_panel(panel: [[i32; BOARD_SIZE]; BOARD_SIZE]) {
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if panel[y][x] == 0 {
                print!("   ");
            } else {
                print!("{:>2} ", panel[y][x]);
            }
        }
        println!();
    }
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
    None // 理論上ここには来ない
}

// パズルが完成したかチェックする
fn is_cleared(panel: [[i32; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    panel == SOLVED_PANEL
}