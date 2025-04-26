use std::collections::*;

#[derive(Clone, Copy)]
struct Conf {
    debug: bool,
    climb0_count: i32,
}

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn to_block(d: char, board: &[u32], to_b: &mut [Vec<(usize, usize)>]) {
    // board is a bitboard. TODO write a function to return the cell right before the block
    let n = board.len();
    match d {
        'D' => for y in 0..n {
            let mut cx = n - 1;
            for x in (0..n).rev() {
                if board[x] & (1 << y) != 0 {
                    cx = x.max(1) - 1;
                } else {
                    to_b[x][y] = (cx, y);
                }
            }
        }
        'R' => for x in 0..n {
            let mut cy = n - 1;
            for y in (0..n).rev() {
                if board[x] & (1 << y) != 0 {
                    cy = y.max(1) - 1;
                } else {
                    to_b[x][y] = (x, cy);
                }
            }
        }
        'U' => for y in 0..n {
            let mut cx = 0;
            for x in 0..n {
                if board[x] & (1 << y) != 0 {
                    cx = x + 1;
                } else {
                    to_b[x][y] = (cx, y);
                }
            }
        }
        'L' => for x in 0..n {
            let mut cy = 0;
            for y in 0..n {
                if board[x] & (1 << y) != 0 {
                    cy = y + 1;
                } else {
                    to_b[x][y] = (x, cy);
                }
            }
        }
        _ => panic!("Invalid direction"),
    }
}

fn opt_one_move(mut x: usize, mut y: usize, tx: usize, ty: usize, board: &[u32]) -> Vec<(char, char)> {
    let mut mv = vec![];
    while x < tx {
        mv.push(('M', 'D'));
        x += 1;
    }
    while y < ty {
        mv.push(('M', 'R'));
        y += 1;
    }
    while x > tx {
        mv.push(('M', 'U'));
        x -= 1;
    }
    while y > ty {
        mv.push(('M', 'L'));
        y -= 1;
    }
    mv
}

fn opt_one(x: usize, y: usize, tx: usize, ty: usize, board: &[u32]) -> Vec<(char, char)> {
    let n = board.len();
    // TODO better calc
    let mut to_b = vec![vec![vec![(0, 0); n]; n]; 4];
    let dirs = ['D', 'R', 'U', 'L'];
    let dxy = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for i in 0..4 {
        to_block(dirs[i], board, &mut to_b[i]);
    }
    let mut que = VecDeque::new();
    que.push_back((0, x, y, '.', '@', n, n));
    let mut dist = vec![vec![2 * n; n]; n];
    let mut op = vec![vec![('-', '-', n, n); n]; n];
    while let Some((dis, x, y, op1, op2, ox, oy)) = que.pop_front() {
        if dis > 2 * n - 2 {
            continue;
        }
        if dis >= dist[x][y] {
            continue;
        }
        dist[x][y] = dis;
        op[x][y] = (op1, op2, ox, oy);
        for (i, &d) in dirs.iter().enumerate() {
            let (dx, dy) = dxy[i];
            let (nx, ny) = to_b[i][x][y];
            if (x, y) != (nx, ny) {
                que.push_back((dis + 1, nx, ny, 'S', d, x, y));
            }
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx < n && ny < n {
                que.push_back((dis + 1, nx, ny, 'M', d, x, y));
            }
        }
    }
    // path recovery
    let mut mv = vec![];
    let mut cx = tx;
    let mut cy = ty;
    while (cx, cy) != (x, y) {
        let (op1, op2, ox, oy) = op[cx][cy];
        mv.push((op1, op2));
        cx = ox;
        cy = oy;
    }
    mv.reverse();
    mv
}

fn main() {
    let mut conf = Conf {
        debug: false,
        climb0_count: 0,
    };

    let first_line = getline().trim().to_string();
    let first_line: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, m) = (first_line[0], first_line[1]);
    let mut xy = vec![];
    for _ in 0..m {
        let line = getline().trim().to_string();
        let line: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        xy.push((line[0], line[1]));
    }

    if conf.debug {
        eprintln!("n: {}, m: {}", n, m);
    }

    let mut mv = vec![];
    let board = vec![0; n];
    for i in 1..m {
        let x = xy[i - 1].0;
        let y = xy[i - 1].1;
        if mv.len() >= 2 * n * m {
            break;
        }
        let cur = opt_one(x, y, xy[i].0, xy[i].1, &board);
        mv.extend(cur);
    }
    assert!(mv.len() <= 2 * n * m);
    for &(a, b) in &mv {
        println!("{} {}", a, b);
    }
    let turn = mv.len();
    eprintln!("# turn = {}, score = {}", turn, m + 2 * n * m - turn);
}
