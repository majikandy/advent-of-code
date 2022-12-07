use std::{
    fs::{read_to_string, File},
    io::Read,
    mem::take,
};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let groups = contents.split_once("\n\n");

    let mut columns: Vec<Vec<char>> = vec![vec![]; 9];

    let stacks = groups.unwrap().0.lines();
    stacks.for_each(|line| {
        if !line.starts_with(" 1") {
            line.chars().enumerate().for_each(|(i, c)| {
                if i != 0 && (i - 1) % 4 == 0 && !c.is_whitespace() {
                    columns[(i - 1) / 4].push(c);
                }
            });
        }
    });

    columns.iter_mut().for_each(|column| {
        column.reverse();
    });

    let moves = groups.unwrap().1.lines();
    //move 5 from 4 to 7
    moves.for_each(|line| {
        let mut iter = line.split_whitespace();
        let num_of_crates = iter.nth(1).unwrap().parse::<usize>().unwrap();
        let from = iter.nth(1).unwrap().parse::<usize>().unwrap();
        let to = iter.nth(1).unwrap().parse::<usize>().unwrap();

        let mut crates = vec![];
        for i in 0..num_of_crates {
            let container = columns[from - 1].pop().unwrap();
            crates.push(container);
        }
        crates.reverse();

        columns[to - 1].append(crates.as_mut());
    });

    let mut output = "".to_string();
    for i in 0..9 {
        output = format!(
            "{}{}",
            output,
            columns[i].last().unwrap_or(&'$').to_string()
        );
    }
    println!("{}", output);
}
