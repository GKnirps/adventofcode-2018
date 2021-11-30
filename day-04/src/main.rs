#[macro_use]
extern crate lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let mut lines: Vec<&str> = content.split('\n').collect();
    lines.sort_unstable();
    let lines = lines;

    let sleep_times = guard_sleep_times(&lines);

    let sleepiest_guard = sleep_times
        .iter()
        .map(|(id, sheet)| (id, sheet, sheet.iter().sum::<u32>()))
        .max_by_key(|(_, _, sum)| sum.clone())
        .ok_or_else(|| "No guards!".to_owned())?;

    let sleepiest_minute = sleepiest_guard
        .1
        .iter()
        .enumerate()
        .max_by_key(|(_, times)| times.clone())
        .ok_or_else(|| "No time?!?".to_owned())?;
    println!(
        "Guard {} sleeps the most! Sleepiest minute: {}. Puzzle 1 result: {}",
        sleepiest_guard.0,
        sleepiest_minute.0,
        *sleepiest_guard.0 as usize * sleepiest_minute.0
    );

    let sleepiest_minute_guard = sleep_times
        .iter()
        .filter_map(|(id, sheet)| {
            sheet
                .iter()
                .enumerate()
                .max_by_key(|(_, times)| times.clone())
                .map(|(minute, times)| (id, sheet, minute, times))
        })
        .max_by_key(|(_, _, _, times)| times.clone())
        .ok_or_else(|| "No guards!".to_owned())?;
    println!(
        "Guard {} sleeps most often in minute {}. Puzzle 2 result: {}",
        sleepiest_minute_guard.0,
        sleepiest_minute_guard.2,
        *sleepiest_minute_guard.0 as usize * sleepiest_minute_guard.2
    );

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

fn guard_sleep_times(lines: &[&str]) -> HashMap<u32, Vec<u32>> {
    let mut result: HashMap<u32, Vec<u32>> = HashMap::with_capacity(lines.len());
    let mut asleep_since: Option<u32> = None;
    let mut current_guard = 0xdeadbeef;
    for event in lines.iter().filter_map(|line| parse_log_line(line)) {
        match event {
            Event::Begin(guard_id) => {
                asleep_since = None;
                current_guard = guard_id;
            }
            Event::FallsAsleep(minutes) => {
                // we assume the logs are correct, so we don't check if the guard is
                // already asleep
                asleep_since = Some(minutes);
            }
            Event::WakesUp(minutes) => {
                if !result.contains_key(&current_guard) {
                    result.insert(current_guard, (0..60).map(|_| 0).collect());
                }
                // also not very stable but who cares
                if let Some(asleep_since) = asleep_since {
                    if let Some(sleepsheet) = result.get_mut(&current_guard) {
                        for i in asleep_since..minutes {
                            sleepsheet[i as usize] = sleepsheet[i as usize] + 1;
                        }
                    }
                }
            }
        }
    }
    return result;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Event {
    Begin(u32),
    FallsAsleep(u32),
    WakesUp(u32),
}

fn parse_log_line(line: &str) -> Option<Event> {
    lazy_static! {
        static ref RE_LOG: Regex = Regex::new(
            r"\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (Guard|wakes up|falls asleep)( #(\d+))?"
        )
        .unwrap();
    }
    let capture = RE_LOG.captures(line)?;
    let minutes: u32 = capture.get(1)?.as_str().parse().ok()?;
    let discriminator = capture.get(2)?.as_str();
    return if discriminator == "falls asleep" {
        Some(Event::FallsAsleep(minutes))
    } else if discriminator == "wakes up" {
        Some(Event::WakesUp(minutes))
    } else if discriminator == "Guard" {
        let guard_id: u32 = capture.get(4)?.as_str().parse().ok()?;
        Some(Event::Begin(guard_id))
    } else {
        None
    };
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_log_line_parses_correctly() {
        assert_eq!(
            parse_log_line("[1518-11-01 00:00] Guard #10 begins shift"),
            Some(Event::Begin(10))
        );
        assert_eq!(
            parse_log_line("[1518-11-01 00:05] falls asleep"),
            Some(Event::FallsAsleep(5))
        );
        assert_eq!(
            parse_log_line("[1518-11-01 00:25] wakes up"),
            Some(Event::WakesUp(25))
        );
    }
}
