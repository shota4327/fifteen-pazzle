fn main() {
    let panel = make_pazzle();
    show_panel(panel);
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