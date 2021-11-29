#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Claim {
    id: u64,
    px: usize,
    py: usize,
    sx: usize,
    sy: usize,
}

fn required_size(claims: &[Claim]) -> Option<(usize, usize)> {
    let x = claims.iter().map(|c| c.px + c.sx).max()?;
    let y = claims.iter().map(|c| c.py + c.sy).max()?;
    Some((x, y))
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Fabric {
    sx: usize,
    sy: usize,
    area: Vec<Vec<u64>>,
}

impl Fabric {
    fn with_size(sx: usize, sy: usize) -> Fabric {
        Fabric {
            sx,
            sy,
            area: (0..(sx * sy)).map(|_| Vec::new()).collect(),
        }
    }
    fn claim(&mut self, claim_id: u64, x: usize, y: usize) {
        // no timez for da stability!
        self.area[x + y * self.sx].push(claim_id);
    }
    fn process_claims(&mut self, claims: &[Claim]) {
        for claim in claims {
            for x in claim.px..(claim.px + claim.sx) {
                for y in claim.py..(claim.py + claim.sy) {
                    self.claim(claim.id, x, y);
                }
            }
        }
    }
    fn count_double_claimed(&self) -> usize {
        self.area.iter().filter(|b| b.len() > 1).count()
    }
    fn find_non_overlapping_claims(&self, claims: &[Claim]) -> HashSet<u64> {
        let mut ids: HashSet<u64> = claims.iter().map(|c| c.id).collect();
        for cell in &self.area {
            if cell.len() > 1 {
                for id in cell {
                    ids.remove(&id);
                }
            }
        }
        return ids;
    }
}

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("No file name given.".to_owned())?;
    let content = read_file(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let claims = parse_claims(&content);
    let (xsize, ysize) = required_size(&claims).ok_or_else(|| "No claims".to_owned())?;

    println!("Claims require size {} × {}", xsize, ysize);

    let mut fabric = Fabric::with_size(xsize, ysize);

    fabric.process_claims(&claims);
    let double_count = fabric.count_double_claimed();

    println!("{} squares are claimed at least twice", double_count);

    let non_overlapping = fabric.find_non_overlapping_claims(&claims);
    println!(
        "Number of non-Overlapping claims: {}",
        non_overlapping.len()
    );
    for id in non_overlapping {
        println!("{}", id);
    }

    Ok(())
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let ifile = File::open(path)?;
    let mut bufr = BufReader::new(ifile);
    let mut result = String::with_capacity(2048);
    bufr.read_to_string(&mut result)?;
    return Ok(result);
}

fn parse_claims(input: &str) -> Vec<Claim> {
    input.split('\n').filter_map(parse_claim).collect()
}

fn parse_claim(line: &str) -> Option<Claim> {
    lazy_static! {
        static ref RE_CLAIM: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let capture = RE_CLAIM.captures(line)?;
    let id: u64 = capture.get(1)?.as_str().parse().ok()?;
    let px: usize = capture.get(2)?.as_str().parse().ok()?;
    let py: usize = capture.get(3)?.as_str().parse().ok()?;
    let sx: usize = capture.get(4)?.as_str().parse().ok()?;
    let sy: usize = capture.get(5)?.as_str().parse().ok()?;

    Some(Claim { id, px, py, sx, sy })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fabric_counts_overlapping_claims() {
        // given
        let mut fabric = Fabric::with_size(8, 8);
        let claims = [
            Claim {
                id: 1,
                px: 1,
                py: 3,
                sx: 4,
                sy: 4,
            },
            Claim {
                id: 2,
                px: 3,
                py: 1,
                sx: 4,
                sy: 4,
            },
            Claim {
                id: 3,
                px: 5,
                py: 5,
                sx: 2,
                sy: 2,
            },
        ];

        // when
        fabric.process_claims(&claims);
        let count = fabric.count_double_claimed();

        // then
        assert_eq!(count, 4);
    }

    #[test]
    fn read_claim_reads_valid_claim() {
        // given
        let input = "#10 @ 505,954: 23x15";

        // when
        let claim = parse_claim(input).unwrap();

        // then
        assert_eq!(
            claim,
            Claim {
                id: 10,
                px: 505,
                py: 954,
                sx: 23,
                sy: 15
            }
        );
    }

    #[test]
    fn read_claim_rejects_invalid_claims() {
        assert!(parse_claim("").is_none());
        assert!(parse_claim("# @ 505,9: 23x15").is_none());
    }

    #[test]
    fn parse_claims_parses_claims_and_ignores_invalid_lines() {
        // given
        let claims = "#1 @ 1,3: 4x4\nfoobar\n#3 @ 5,5: 2x3\n";

        // when
        let result = parse_claims(claims);

        // then
        assert_eq!(
            &result,
            &[
                Claim {
                    id: 1,
                    px: 1,
                    py: 3,
                    sx: 4,
                    sy: 4
                },
                Claim {
                    id: 3,
                    px: 5,
                    py: 5,
                    sx: 2,
                    sy: 3
                }
            ]
        );
    }
}
