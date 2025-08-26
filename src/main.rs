use std::io::{self, Write};

fn main() {
    let panel = make_pazzle();
    play_game(panel);
}

// ゲームをプレイするメインの関数
fn play_game(mut panel: [[i32; 4]; 4]) {
    loop {
        println!("\n--- 15 Puzzle ---");
        show_panel(panel);

        if is_cleared(panel) {
            println!("\nCongratulations! You solved the puzzle!");
            break;
        }

        // 空白ピース(0)の現在位置を探す
        let (y, x) = match find_zero(&panel) {
            Some(pos) => pos,
            None => {
                println!("Error: Could not find the empty space. Exiting.");
                break;
            }
        };

        print!("\nMove piece (1:Up, 2:Down, 3:Left, 4:Right, 0:Quit): ");
        // プロンプトをすぐ表示するためにflushする
        io::stdout().flush().expect("flush failed!");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(1) => { // 上へ: 空白ピースの下のピースを動かす
                if y < 3 {
                    let tmp = panel[y][x];
                    panel[y][x] = panel[y + 1][x];
                    panel[y + 1][x] = tmp;
                } else {
                    println!("-> Cannot move up.");
                }
            }
            Ok(2) => { // 下へ: 空白ピースの上のピースを動かす
                if y > 0 {
                    let tmp = panel[y][x];
                    panel[y][x] = panel[y - 1][x];
                    panel[y - 1][x] = tmp;
                } else {
                    println!("-> Cannot move down.");
                }
            }
            Ok(3) => { // 左へ: 空白ピースの右のピースを動かす
                if x < 3 {
                    let tmp = panel[y][x];
                    panel[y][x] = panel[y][x + 1];
                    panel[y][x + 1] = tmp;
                } else {
                    println!("-> Cannot move left.");
                }
            }
            Ok(4) => { // 右へ: 空白ピースの左のピースを動かす
                if x > 0 {
                    let tmp = panel[y][x];
                    panel[y][x] = panel[y][x - 1];
                    panel[y][x - 1] = tmp;
                } else {
                    println!("-> Cannot move right.");
                }
            }
            Ok(0) => {
                println!("Quitting the game.");
                break;
            }
            _ => {
                println!("-> Invalid input. Please enter a number from 0 to 4.");
            }
        }
    }
}

fn make_pazzle() -> [[i32; 4]; 4] {
    let mut panel: [[i32; 4]; 4] = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]];
    let hint: [[i32; 4]; 4] = [[10, 11, 11, 9], [14, 15, 15, 13], [14, 15, 15, 13], [6, 7, 7, 5]];
    let turn: [i32; 9] = [0, 2, 1, 0, 8, 0, 0, 0, 4];

    let mut k: [i32; 4] = [0, 0, 0, 0];

    // 座標は配列のインデックスなので usize 型が最適
    let mut y: usize = 3;
    let mut x: usize = 3;
    let mut n: i32 = 0;

    for _ in 0..1000 {
        let mut h = hint[y][x];
        let mut d = 1;
        let mut i: usize = 0;

        for _ in 0..4 {
            if h % 2 == 1 && n % 2 == 0 {
                k[i as usize] = d;
                i += 1;
            }
            h = h / 2;
            n = n / 2;
            d = d * 2;
        }
        // 移動できる方向がない場合(i=0)はゼロ除算を避ける
        if i == 0 {
            continue;
        }
        let r: u8 = rand::random();
        d = k[r as usize % i];

        // 移動量は負になる可能性があるため isize 型を使う
        let t = d / 4;
        let ry: isize = (t / 2) as isize - (t % 2) as isize;
        let t = d % 4;
        let rx: isize = (t / 2) as isize - (t % 2) as isize;

        // isize と usize を安全に計算し、結果を usize にしてインデックスとして使う
        let next_y = (y as isize + ry) as usize;
        let next_x = (x as isize + rx) as usize;

        panel[y][x] = panel[next_y][next_x];
        panel[next_y][next_x] = 0;

        y = next_y;
        x = next_x;

        n = turn[d as usize];
    }

    panel

}

fn show_panel(panel: [[i32; 4]; 4]) {
    for y in 0..4 {
        for x in 0..4 {
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
fn find_zero(panel: &[[i32; 4]; 4]) -> Option<(usize, usize)> {
    for y in 0..4 {
        for x in 0..4 {
            if panel[y][x] == 0 {
                return Some((y, x));
            }
        }
    }
    None // 理論上ここには来ない
}

// パズルが完成したかチェックする
fn is_cleared(panel: [[i32; 4]; 4]) -> bool {
    let solved_panel: [[i32; 4]; 4] = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]];
    panel == solved_panel
}