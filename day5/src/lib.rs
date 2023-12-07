use rayon::prelude::*;
use std::ops::Range;

#[derive(Clone)]
struct Subsection {
    source_range: Range<usize>,
    destination_range: Range<usize>,
}

fn parse_subsections(section: &str) -> Vec<Subsection> {
    section
        .split_once(":\n")
        .unwrap()
        .1
        .lines()
        .map(str::split_whitespace)
        .map(|line| line.map(str::parse).map(Result::unwrap).collect::<Vec<_>>())
        .map(|nums| Subsection {
            source_range: nums[1]..nums[1] + nums[2],
            destination_range: nums[0]..nums[0] + nums[2],
        })
        .collect()
}

fn transform_seed(subsections: Vec<Subsection>, seed: usize) -> usize {
    for subsection in subsections {
        if subsection.source_range.contains(&seed) {
            return subsection.destination_range.start + (seed - subsection.source_range.start);
        }
    }

    seed
}

fn transform_seed_from_start_to_end(all_subsections: Vec<Vec<Subsection>>, seed: usize) -> usize {
    let mut result = seed;

    for subsections in all_subsections {
        result = transform_seed(subsections, result);
    }

    result
}

pub fn solve_part_1(file_content: &str) -> usize {
    let sections = file_content.split("\n\n").collect::<Vec<&str>>();

    let subsections = sections[1..]
        .iter()
        .map(|section| parse_subsections(section))
        .collect::<Vec<Vec<Subsection>>>();

    let seeds = sections[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    seeds
        .map(|seed| transform_seed_from_start_to_end(subsections.clone(), seed))
        .min()
        .unwrap()
}

pub fn solve_part_2(file_content: &str) -> usize {
    let sections = file_content.split("\n\n").collect::<Vec<&str>>();

    let subsections = sections[1..]
        .iter()
        .map(|section| parse_subsections(section))
        .collect::<Vec<Vec<Subsection>>>();

    let seeds = sections[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .fold(Vec::new(), |mut acc, chunk| {
            for i in chunk[0]..chunk[0] + chunk[1] {
                acc.push(i);
            }

            acc
        });

    seeds
        .into_par_iter()
        .map(|seed| transform_seed_from_start_to_end(subsections.clone(), seed))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(SAMPLE_DATA), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(SAMPLE_DATA), 46);
    }
}
