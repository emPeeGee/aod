use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};


// Aption





pub fn solve() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let mut input = File::open("./assets/day03.txt").unwrap();
    let lines = BufReader::new(&input).lines();
    let priorities: Vec<usize> = lines
        .map(|line| {
            let line = line.unwrap();
            let half_length = line.len() / 2;
            let (first, sec) = line.split_at(half_length);

            let halves_unique_ch = first
                .chars()
                .find(|ch| sec.contains(&ch.to_string()))
                .unwrap();

            let ch_priority = alphabet
                .iter()
                .position(|letter| letter == &halves_unique_ch)
                .unwrap()
                + 1;

            ch_priority
        })
        .collect();

    let sum: usize = priorities.iter().sum();
    println!("{sum}");

    // Moving cursor to the beginning of the file
    input.seek(std::io::SeekFrom::Start(0)).unwrap();
    let lines2 = BufReader::new(&input)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let unique_items: Vec<usize> = lines2
        .chunks(3)
        .map(|l| {
            let unique = l[0]
                .chars()
                .find(|ch| l[1].contains(&ch.to_string()) && l[2].contains(&ch.to_string()))
                .unwrap();

            let ch_priority = alphabet
                .iter()
                .position(|letter| letter == &unique)
                .unwrap()
                + 1;

            ch_priority
        })
        .collect();

    let sum: usize = unique_items.iter().sum();
    println!("{sum}");
}
