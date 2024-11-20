use crate::utils::utils::get_input_lines;

struct Elf {
    number: usize,
    calories: i32,
}

pub fn solve() {
    let lines = get_input_lines("assets/advent_01.txt");
    let mut elves = lines
        .map(|line| {
            let line = line.unwrap();
            let calories: i32 = line.parse().unwrap_or(-1);
            calories
        })
        .collect::<Vec<i32>>()
        .split(|el| el.clone() == -1)
        .into_iter()
        .enumerate()
        .map(|(i, calories)| Elf {
            number: i,
            calories: calories.into_iter().sum(),
        })
        .collect::<Vec<Elf>>();

    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    let sorted_elves = &elves;

    let top_elf = &sorted_elves.first().unwrap();
    let top_three_elves = &sorted_elves
        .into_iter()
        .take(3)
        .map(|elf| elf.calories)
        .sum::<i32>();

    println!("======= Day1: =======");
    println!(
        "The biggest elf is {} with {} calories",
        &top_elf.number, &top_elf.calories
    );
    println!("Top three summed calories: {}", top_three_elves);
    println!("=====================");
}
