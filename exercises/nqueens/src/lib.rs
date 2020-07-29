#[cfg(test)]
mod tests;

pub fn solve_cloning(b: &Vec<isize>, index: usize) -> Vec<Vec<isize>> {
    let mut res = vec![];
    let mut board = b.clone();
    if index >= board.len() {
        return res;
    }
    let last = index == board.len() - 1;
    for i in 0..board.len() {
        board[index] = i as isize;
        if last && check(&board) {
            res.push(board.clone())
        } else {
            let mut s = solve_cloning(&board, index + 1);
            res.append(&mut s);
        }
    }
    res
}
pub fn check(b: &Vec<isize>) -> bool {
    for i in 0..b.len() - 1 {
        let v = b[i];
        if v == -1 {
            return false;
        }
        for j in i + 1..b.len() {
            let c = b[j];
            if (v == c)
                || ((v - c) == ((i as isize) - (j as isize)))
                || ((c - v) == ((i as isize) - (j as isize)))
            {
                return false;
            }
        }
    }
    true
}
fn smart_check(b: &Vec<isize>) -> bool {
    let last_index = (b.len() - 1) as isize;
    let v = b[last_index as usize];
    for i in 0..last_index {
        let c = b[i as usize];
        if (v == c) || ((v - c) == (i - last_index)) || ((c - v) == (i - last_index)) {
            return false;
        }
    }
    true
}

pub fn solve(board: &mut Vec<isize>) -> Vec<Vec<isize>> {
    let mut res = vec![];
    let cap = board.capacity();
    let i = board.len();
    let last = cap == i + 1;
    for i in 0..cap {
        board.push(i as isize);
        if last {
            if smart_check(board) {
                res.push(board.clone());
            }
        } else {
            if check(board) {
                res.append(&mut solve(board));
            }
            //res.append(&mut solve(board));
        }
        board.pop();
    }
    res
}

pub fn pretty_print(board: &Vec<isize>) {
    let l = board.len();
    if l == 0 {
        return ();
    }
    let mut spacer = String::new();
    for _ in 0..l * 2 + 2 {
        spacer.push('-');
    }
    println!("{}", &spacer);
    for v in board.iter() {
        print!("|");
        for i in 0..board.len() {
            if i as isize == *v {
                print!("X|");
            } else {
                print!(" |");
            }
        }
        println!("\n{}", spacer);
    }
}

pub fn run(size: u8) {
    let mut b: Vec<isize> = Vec::with_capacity(size as usize);
    let sol = solve(&mut b);
    println!("Total solutions: {}", sol.len());
    for s in &sol {
        println!("\n");
        pretty_print(s);
    }
}
