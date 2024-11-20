use crate::utils::utils::get_input_lines;
use crate::utils::utils::split_once;

pub fn solve() {
    let lines = get_input_lines("assets/advent_04.txt");
    let overlaps: Vec<usize> = lines
        .map(|line| {
            let line = line.unwrap();

            let (section1, section2) = split_once(line.as_str(), ",");

            let (s1_min, s1_max) = split_once(&section1, "-");
            let (s2_min, s2_max) = split_once(&section2, "-");

            let s1_min: usize = s1_min.parse().unwrap();
            let s1_max: usize = s1_max.parse().unwrap();
            let s2_min: usize = s2_min.parse().unwrap();
            let s2_max: usize = s2_max.parse().unwrap();

            let range1 = s1_min..=s1_max;
            let range2 = s2_min..=s2_max;

            let rs = if range1.contains(&s2_min) && range1.contains(&s2_max)
                || range2.contains(&s1_min) && range2.contains(&s1_max)
            {
                1
            } else {
                0
            };

            rs
        })
        .collect();

    let nr: usize = overlaps.into_iter().sum();

    println!("{nr}");
}
