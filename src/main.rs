use std::fs;

#[derive(Debug)]
struct Race {
    time: u32,
    best_distance: u32,
}

fn count_winnings(race: &Race) -> u32 {
    let mut count: u32 = 0;
    for secs_pressed in 0..race.time {
        let distance_traveled = secs_pressed * (race.time - secs_pressed);
        if distance_traveled > race.best_distance {
            count += 1;
        }
    }
    println!("{:?}: {}", race, count);
    count
}

fn compute_multiplication(races: Vec<Race>) -> u32 {
    races.iter().fold(1, |acc, e| acc * count_winnings(e))
}

fn read_lines(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
}

fn read_races(path: &str) -> Vec<Race> {
    let lines = read_lines(path);
    let mut it = lines.iter();

    let times: Vec<&str> = it.next().unwrap().split_whitespace().skip(1).collect();
    let distances: Vec<&str> = it.next().unwrap().split_whitespace().skip(1).collect();

    let mut races = Vec::new();

    assert_eq!(times.len(), distances.len());
    for i in 0..times.len() {
        races.push(Race {
            time: times[i].parse().unwrap(),
            best_distance: distances[i].parse().unwrap(),
        });
    }

    races
}

fn main() {
    let races = read_races("res/data.txt");
    // let races = read_races("res/data_light.txt");
    println!("{}", compute_multiplication(races));
}
