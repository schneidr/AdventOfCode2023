use std::fs::read_to_string;

fn main() {
    let mut times: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();
    let mut win_options: u32 = 1;
    for line in read_to_string("races.txt").unwrap().lines() {
        if line.starts_with("Time:") {
            times = line[11..].split_whitespace()
                .map(|time|time.parse()
                .unwrap())
                .collect();
        }
        else if line.starts_with("Distance:") {
            distances = line[11..].split_whitespace()
                .map(|time|time.parse()
                .unwrap())
                .collect();
        }
    }
    for race in 0..times.len() {
        println!("*** Race {race}: {0}ms, {1}mm ***", times[race], distances[race]);
        let mut race_win_options = 0;
        for charge_time in 1..times[race] {
            let distance = charge_time * (times[race] - charge_time);
            if distance > distances[race] {
                race_win_options += 1;
                // println!("YES: {charge_time}ms -> {distance}mm");
            } 
            // else {
            //     println!("NO:  {charge_time}ms -> {distance}mm");
            // }
        }
        println!("{race_win_options} win options");
        win_options *= race_win_options;
    }
    println!("Total win options: {win_options}");
}
