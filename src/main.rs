use std::io;

fn solution(a: usize, b: usize) -> u8 {
    const TABLE: [[u8; 4]; 10] = [
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [2, 4, 8, 6],
        [3, 9, 7, 1],
        [4, 6, 4, 6],
        [5, 5, 5, 5],
        [6, 6, 6, 6],
        [7, 9, 3, 1],
        [8, 4, 2, 6],
        [9, 1, 9, 1],
    ];

    let a_last_digit = a % 10;

    (TABLE[a_last_digit][(b - 1) % 4] + 9) % 10 + 1
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split(' ').flat_map(&str::parse::<usize>);
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        println!("{}", solution(a, b));
    }
}
