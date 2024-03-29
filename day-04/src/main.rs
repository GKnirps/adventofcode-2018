use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let mut lines: Vec<&str> = content.lines().collect();
    lines.sort_unstable();
    let lines = lines;

    let sleep_times = guard_sleep_times(&lines);

    let sleepiest_guard = sleep_times
        .iter()
        .map(|(id, sheet)| (id, sheet, sheet.iter().sum::<u32>()))
        .max_by_key(|(_, _, sum)| *sum)
        .ok_or_else(|| "No guards!".to_owned())?;

    let sleepiest_minute = sleepiest_guard
        .1
        .iter()
        .enumerate()
        .max_by_key(|(_, times)| *times)
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
                .max_by_key(|(_, times)| *times)
                .map(|(minute, times)| (id, sheet, minute, times))
        })
        .max_by_key(|(_, _, _, times)| *times)
        .ok_or_else(|| "No guards!".to_owned())?;
    println!(
        "Guard {} sleeps most often in minute {}. Puzzle 2 result: {}",
        sleepiest_minute_guard.0,
        sleepiest_minute_guard.2,
        *sleepiest_minute_guard.0 as usize * sleepiest_minute_guard.2
    );

    Ok(())
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
                result
                    .entry(current_guard)
                    .or_insert_with(|| (0..60).map(|_| 0).collect());
                // also not very stable but who cares
                if let Some(asleep_since) = asleep_since {
                    if let Some(sleepsheet) = result.get_mut(&current_guard) {
                        for i in asleep_since..minutes {
                            sleepsheet[i as usize] += 1;
                        }
                    }
                }
            }
        }
    }
    result
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Event {
    Begin(u32),
    FallsAsleep(u32),
    WakesUp(u32),
}

fn parse_log_line(line: &str) -> Option<Event> {
    let (time, event) = line.split_once("] ")?;
    let minutes: u32 = time.split_once(':')?.1.parse().ok()?;
    match event {
        "falls asleep" => Some(Event::FallsAsleep(minutes)),
        "wakes up" => Some(Event::WakesUp(minutes)),
        s => Some(Event::Begin(
            s.strip_prefix("Guard #")?
                .strip_suffix(" begins shift")?
                .parse()
                .ok()?,
        )),
    }
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
