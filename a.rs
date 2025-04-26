use std::collections::*;

#[derive(Clone, Copy)]
struct Conf {
    debug: bool,
}

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

struct Rng {
    x: u64,
}

impl Rng {
    fn next(&mut self) -> u32 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        ((x ^ x << 10) >> 32) as _
    }
}

fn simulate(x: usize, y: usize, init_board: &[u32], mv: &[(char, char)]) -> Option<(usize, usize, Vec<u32>)> {
    let n = init_board.len();
    let mut x = x;
    let mut y = y;
    let mut board = init_board.to_vec();
    if board[x] & (1 << y) != 0 {
        return None;
    }

    for &(op1, op2) in mv {
        let (dx, dy): (i32, i32) = match op2 {
            'D' => (1, 0),
            'R' => (0, 1),
            'U' => (-1, 0),
            'L' => (0, -1),
            _ => panic!("Invalid move"),
        };
        match op1 {
            'M' => {
                x = x.wrapping_add(dx as usize);
                y = y.wrapping_add(dy as usize);
                if x >= n || y >= n {
                    return None;
                }
                if board[x] & (1 << y) != 0 {
                    return None;
                }
            }
            'S' => {
                while x < n && y < n && board[x] & (1 << y) == 0 {
                    let nx = x.wrapping_add(dx as usize);
                    let ny = y.wrapping_add(dy as usize);
                    if nx >= n || ny >= n || board[nx] & (1 << ny) != 0 {
                        break;
                    }
                    x = nx;
                    y = ny;
                }
            }
            'A' => {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if !(nx < n && ny < n) {
                    return None;
                }
                board[nx] ^= 1 << ny;
            }
            _ => panic!(),
        }
    }
    Some((x, y, board))
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

fn opt_one(x: usize, y: usize, tx: usize, ty: usize, board: &[u32]) -> Option<Vec<(char, char)>> {
    let n = board.len();
    if board[tx] & (1 << ty) != 0 {
        return None;
    }
    if board[x] & (1 << y) != 0 {
        return None;
    }
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
            if nx < n && ny < n && (board[nx] & (1 << ny)) == 0 {
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
    Some(mv)
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

fn simple_opt(xy: &[(usize, usize)], board: &[u32]) -> Option<Vec<(char, char)>> {
    let m = xy.len();
    let mut mv = vec![];
    for i in 1..m {
        let x = xy[i - 1].0;
        let y = xy[i - 1].1;
        let cur = opt_one(x, y, xy[i].0, xy[i].1, &board)?;
        mv.extend(cur);
    }
    Some(mv)
}

const STONE_COST: i32 = 1_000_000;

fn try_stone(board: &[u32], x: usize, y: usize, rest: &[(usize, usize)]) -> Option<(Vec<(char, char)>, Vec<(char, char)>, Vec<u32>)> {
    if rest.is_empty() {
        return None;
    }
    let (tx, ty) = rest[0];
    let n = board.len();
    let board = board.to_vec();
    let cur = opt_one(x, y, tx, ty, &board)?;
    let coords = move_to_coords(&cur, x, y, &board);
    let dxy = [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)];
    let dirs = ['D', 'R', 'U', 'L'];
    let mut opt = {
        let tmp = simple_opt(&rest, &board)?;
        let mut cur2 = cur.clone();
        cur2.extend_from_slice(&tmp);
        (cur2, tmp, board.clone())
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
                if let Some(me_rest) = simple_opt(&rest, &new_board) {
                    let mut me = cur.clone();
                    me.insert(step, ('A', dirs[i]));
                    if &simulate(x, y, &board, &me) == &Some((tx, ty, new_board.clone())) {
                        let last = rest[rest.len() - 1];
                        if simulate(tx, ty, &new_board, &me_rest) == Some((last.0, last.1, new_board.clone())) {
                            if me.len() + me_rest.len() <= opt.0.len() + opt.1.len() {
                                opt = (me, me_rest, new_board);
                            }
                        }
                    }
                }
            }
        }
    }
    Some(opt)
}

fn main() {
    let mut conf = Conf {
        debug: false,
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

    let init_board = vec![0; n];
    let mut board = vec![0; n];
    let mut mv = simple_opt(&xy, &init_board).unwrap();
    let mut frm = vec![];
    for i in 0..m - 1 {
        let sim = simulate(xy[0].0, xy[0].1, &init_board, &mv);
        if let Some((tx, ty, _)) = sim {
            if (tx, ty) != xy[m - 1] {
                eprintln!("WTF 7: {i} {:?} != {:?}", xy[i + 1], sim);
                break;
            }
        } else {
            eprintln!("WTF 6: {i} {:?}", sim);
            break;
        }
        let mut next_frm = frm.clone();
        next_frm.extend(opt_one(xy[i].0, xy[i].1, xy[i + 1].0, xy[i + 1].1, &board).unwrap());
        let sim = simulate(xy[0].0, xy[0].1, &init_board, &frm);
        if sim != Some((xy[i].0, xy[i].1, board.clone())) {
            eprintln!("WTF: {i} {:?} != {:?}", xy[i], sim);
        }
        if let Some((cur, lat, new_board)) = try_stone(&board, xy[i].0, xy[i].1, &xy[i + 1..]) {
            let sim = simulate(xy[i].0, xy[i].1, &board, &cur);
            if sim != Some((xy[i + 1].0, xy[i + 1].1, new_board.clone())) {
                eprintln!("WTF 4: {i} {:?} != {:?}", (xy[i + 1], new_board.clone()), sim);
            }
            if frm.len() + cur.len() + lat.len() < mv.len() {
                let mut new_frm = frm.clone();
                new_frm.extend(cur.clone());
                let sim = simulate(xy[0].0, xy[0].1, &init_board, &new_frm);
                if sim != Some((xy[i + 1].0, xy[i + 1].1, new_board.clone())) {
                    eprintln!("WTF 1: {i} {:?} != {:?}", (xy[i + 1], new_board.clone()), sim);
                    frm = next_frm;
                    continue;
                }

                let mut new_mv = frm.clone();
                new_mv.extend(cur.clone());
                new_mv.extend(lat);
                let sim = simulate(xy[0].0, xy[0].1, &init_board, &new_mv);
                if let Some((tx, ty, ref _new_board)) = sim {
                    if (tx, ty) != (xy[m - 1].0, xy[m - 1].1) {
                        eprintln!("WTF 5: {i} {:?} != {:?}", (xy[m - 1], new_board.clone()), sim);
                        frm = next_frm;
                        continue;
                    }
                } else {
                    eprintln!("WTF 3: {i} {:?} != {:?}", (xy[m - 1], new_board.clone()), sim);
                    frm = next_frm;
                    continue;
                }
                mv = new_mv;
                frm = new_frm;
                board = new_board;
            }
        }
    }
    assert!(mv.len() <= 2 * n * m);
    for &(a, b) in &mv {
        println!("{} {}", a, b);
    }
    let turn = mv.len();
    eprintln!("# turn = {}, score = {}", turn, m + 2 * n * m - turn);
}
