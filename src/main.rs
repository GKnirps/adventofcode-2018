fn main() -> Result<(), String> {
    // hardcode the input here, no need to read it from file
    let n_players: usize = 430;
    let n_marbles: usize = 71588 + 1;

    let score = winning_score(n_players, n_marbles);
    println!(
        "For {} players with {} marbles, the highest score is {}",
        n_players, n_marbles, score
    );

    Ok(())
}

fn winning_score(n_players: usize, n_marbles: usize) -> usize {
    play_game(n_players, n_marbles)
        .into_iter()
        .max()
        .expect("0 players can't play a game.")
}

fn play_game(n_players: usize, n_marbles: usize) -> Vec<usize> {
    let mut state = State::new(n_players, n_marbles);
    while state.next_marble < n_marbles {
        state = state.turn();
    }
    return state.points;
}

struct State {
    n_players: usize,
    circle: Vec<usize>,
    current_marble: usize,
    current_player: usize,
    points: Vec<usize>,
    next_marble: usize,
}

impl State {
    fn new(n_players: usize, n_marbles: usize) -> State {
        let mut circle: Vec<usize> = Vec::with_capacity(n_marbles);
        circle.push(0);
        let points: Vec<usize> = (0..n_players).map(|_| 0).collect();
        State {
            n_players,
            circle: circle,
            current_marble: 0,
            current_player: 0,
            points,
            next_marble: 1,
        }
    }

    fn turn(mut self) -> State {
        let next_current_marble = if self.next_marble % 23 != 0 {
            let pos = (self.current_marble + 2) % self.circle.len();
            self.circle.insert(pos, self.next_marble);
            pos
        } else {
            self.points[self.current_player] += self.next_marble;
            let pos_remove = (self.current_marble + self.circle.len() - 7) % self.circle.len();
            let removed_value = self.circle.remove(pos_remove);
            self.points[self.current_player] += removed_value;
            pos_remove
        };
        let next_player = (self.current_player + 1) % self.n_players;
        State {
            n_players: self.n_players,
            circle: self.circle,
            current_marble: next_current_marble,
            current_player: next_player,
            points: self.points,
            next_marble: self.next_marble + 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn winning_score_for_examples() {
        assert_eq!(winning_score(9, 25 + 1), 32);
        assert_eq!(winning_score(10, 1618 + 1), 8317);
        assert_eq!(winning_score(13, 7999 + 1), 146373);
        assert_eq!(winning_score(17, 1104 + 1), 2764);
        assert_eq!(winning_score(21, 6111 + 1), 54718);
        assert_eq!(winning_score(30, 5807 + 1), 37305);
    }
}
