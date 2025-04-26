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
    let mut x = xy[0].0;
    let mut y = xy[0].1;
    for i in 1..m {
        if mv.len() >= 2 * n * m {
            break;
        }
        while x < xy[i].0 {
            mv.push(('M', 'D'));
            x += 1;
        }
        while y < xy[i].1 {
            mv.push(('M', 'R'));
            y += 1;
        }
        while x > xy[i].0 {
            mv.push(('M', 'U'));
            x -= 1;
        }
        while y > xy[i].1 {
            mv.push(('M', 'L'));
            y -= 1;
        }
    }
    mv.truncate(2 * n * m);
    for (a, b) in mv {
        println!("{} {}", a, b);
    }
}
