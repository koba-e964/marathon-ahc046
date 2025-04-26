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

fn move_to_coords(mv: &[(char, char)], mut x: usize, mut y: usize, board: &[u32]) -> Vec<(usize, usize)> {
    let n = board.len();
    let mut to_b = vec![vec![vec![(0, 0); n]; n]; 4];
    for i in 0..4 {
        to_block("DRUL".chars().nth(i).unwrap(), board, &mut to_b[i]);
    }
    let mut coords = vec![(x, y)];
    for &(op1, op2) in mv {
        match op1 {
            'M' => {
                let (dx, dy): (i32, i32) = match op2 {
                    'D' => (1, 0),
                    'R' => (0, 1),
                    'U' => (-1, 0),
                    'L' => (0, -1),
                    _ => panic!("Invalid move"),
                };
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                assert!(nx < n && ny < n);
                coords.push((nx, ny));
                x = nx;
                y = ny;
            }
            'S' => {
                let idx = "DRUL".find(op2).unwrap();
                let (nx, ny) = to_b[idx][x][y];
                assert!(nx < n && ny < n);
                coords.push((nx, ny));
                x = nx;
                y = ny;
            }
            _ => panic!("Invalid operation"),
        }
    }
    coords
}

fn simple_opt(xy: &[(usize, usize)], board: &[u32]) -> Vec<(char, char)> {
    let m = xy.len();
    let mut mv = vec![];
    for i in 1..m {
        let x = xy[i - 1].0;
        let y = xy[i - 1].1;
        let cur = opt_one(x, y, xy[i].0, xy[i].1, &board);
        mv.extend(cur);
    }
    mv
}

const STONE_COST: i32 = 1_000_000;

fn try_stone(board: &[u32], x: usize, y: usize, rest: &[(usize, usize)]) -> Vec<(char, char)> {
    if rest.is_empty() {
        return vec![];
    }
    let mut mv = vec![];
    let (tx, ty) = rest[0];
    if x == tx && y == ty {
        return mv;
    }
    let n = board.len();
    let mut board = board.to_vec();
    let cur = opt_one(x, y, tx, ty, &board);
    let coords = move_to_coords(&cur, x, y, &board);
    let dxy = [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)];
    let dirs = ['D', 'R', 'U', 'L'];
    let mut opt = {
        let tmp = simple_opt(&rest, &board);
        let mut cur2 = cur.clone();
        cur2.extend_from_slice(&tmp);
        cur2
    };
    for step in 0..cur.len() + 1 {
        let next_dir = if step == cur.len() {
            '*'
        } else {
            cur[step].1
        };
        let (now_x, now_y) = coords[step];
        for i in 0..4 {
            if next_dir == dirs[i] {
                continue;
            }
            let (dx, dy) = dxy[i];
            let (nx, ny) = (now_x.wrapping_add(dx as usize), now_y.wrapping_add(dy as usize));
            if nx < n && ny < n {
                // TODO: remove cloning
                let mut new_board = board.to_vec();
                new_board[nx] ^= 1 << ny;
                let mut me_rest = simple_opt(&rest, &new_board);
                let mut me = cur.clone();
                me.insert(step, ('A', dirs[i]));
                me.extend_from_slice(&me_rest);
                if me.len() <= opt.len() {
                    opt = me;
                }
            }
        }
    }
    opt
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

    let board = vec![0; n];
    let mut mv = simple_opt(&xy, &board);
    for i in 0..m {
        let mut frm = simple_opt(&xy[..i + 1], &board);
        let lat = try_stone(&board, xy[i].0, xy[i].1, &xy[i + 1..]);
        frm.extend_from_slice(&lat);
        if frm.len() < mv.len() {
            mv = frm;
        }
    }
    assert!(mv.len() <= 2 * n * m);
    for &(a, b) in &mv {
        println!("{} {}", a, b);
    }
    let turn = mv.len();
    eprintln!("# turn = {}, score = {}", turn, m + 2 * n * m - turn);
}
