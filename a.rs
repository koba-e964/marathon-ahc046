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

fn opt_one(mut x: usize, mut y: usize, tx: usize, ty: usize, board: &[Vec<i32>]) -> Vec<(char, char)> {
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
    let board = vec![vec![0; n]; n];
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
