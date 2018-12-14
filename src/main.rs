fn main() {
    let last_scores = create_recipes(380621);
    println!("The last 10 scores are {:?}", last_scores);

    let recipes_until_pattern = create_recipes_until_pattern(&[3, 8, 0, 6, 2, 1]);
    println!(
        "The pattern occurs after {} recipes.",
        recipes_until_pattern
    );
}

fn step(state: State) -> State {
    let State {
        mut scoreboard,
        current_1,
        current_2,
    } = state;
    let combined = scoreboard[current_1] + scoreboard[current_2];
    if combined > 9 {
        scoreboard.push(combined / 10);
    }
    scoreboard.push(combined % 10);

    let next_1 = (current_1 + 1 + scoreboard[current_1] as usize) % scoreboard.len();
    let next_2 = (current_2 + 1 + scoreboard[current_2] as usize) % scoreboard.len();

    return State {
        scoreboard,
        current_1: next_1,
        current_2: next_2,
    };
}

fn create_recipes(n: usize) -> Vec<u8> {
    let mut state: State = State::with_capacity(n + 10);
    while state.scoreboard.len() < n + 10 {
        state = step(state);
    }
    return state.scoreboard[n..n + 10].to_vec();
}

fn create_recipes_until_pattern(pattern: &[u8]) -> usize {
    let mut state: State = State::with_capacity(2048);
    loop {
        state = step(state);
        let score_len = state.scoreboard.len();
        if score_len >= pattern.len()
            && &state.scoreboard[score_len - pattern.len()..score_len] == pattern
        {
            return score_len - pattern.len();
        }
        if score_len >= pattern.len() + 1
            && &state.scoreboard[score_len - 1 - pattern.len()..score_len - 1] == pattern
        {
            return score_len - pattern.len() - 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    scoreboard: Vec<u8>,
    current_1: usize,
    current_2: usize,
}

impl State {
    fn with_capacity(c: usize) -> State {
        let mut scoreboard: Vec<u8> = Vec::with_capacity(c);
        scoreboard.push(3);
        scoreboard.push(7);
        State {
            scoreboard,
            current_1: 0,
            current_2: 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_recipes_works_for_examples() {
        assert_eq!(create_recipes(9), vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
        assert_eq!(create_recipes(5), vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
        assert_eq!(create_recipes(18), vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);
        assert_eq!(create_recipes(2018), vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);
    }

    #[test]
    fn create_recipes_until_pattern_works_for_examples() {
        assert_eq!(create_recipes_until_pattern(&[5, 1, 5, 8, 9]), 9);
        assert_eq!(create_recipes_until_pattern(&[0, 1, 2, 4, 5]), 5);
        assert_eq!(create_recipes_until_pattern(&[9, 2, 5, 1, 0]), 18);
        assert_eq!(create_recipes_until_pattern(&[5, 9, 4, 1, 4]), 2018);
    }
}
