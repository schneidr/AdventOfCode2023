use std::fs::read_to_string;

fn main() {
    let mut race_time: u64 = 0;
    let mut race_distance: u64 = 0;
    for line in read_to_string("races.txt").unwrap().lines() {
        if line.starts_with("Time:") {
            let number: String = line[11..].chars().filter(|c| !c.is_whitespace()).collect();
            race_time = number.parse().unwrap();

        }
        else if line.starts_with("Distance:") {
            let number: String = line[11..].chars().filter(|c| !c.is_whitespace()).collect();
            race_distance = number.parse().unwrap();
        }
    }
    println!("*** Race: {0}ms, {1}mm ***", race_time, race_distance);
    let mut race_win_options = 0;
    for charge_time in 1..race_time {
        let distance = charge_time * (race_time - charge_time);
        if distance > race_distance {
            race_win_options += 1;
        } 
    }
    println!("{race_win_options} win options");
}
