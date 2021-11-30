mod circle;

use self::circle::CircularList;

fn main() -> Result<(), String> {
    // hardcode the input here, no need to read it from file
    let n_players: usize = 430;
    let highest_marble_puzzle_1: usize = 71588;

    let score1 = winning_score(n_players, highest_marble_puzzle_1 + 1);
    println!(
        "For {} players with {} marbles, the highest score is {}",
        n_players, highest_marble_puzzle_1, score1
    );

    let highest_marble_puzzle_2 = highest_marble_puzzle_1 * 100;
    let score2 = winning_score(n_players, highest_marble_puzzle_2 + 1);
    println!(
        "For {} players with {} marbles, the highest score is {}",
        n_players, highest_marble_puzzle_2, score2
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
    circle: CircularList<usize>,
    current_player: usize,
    points: Vec<usize>,
    next_marble: usize,
}

impl State {
    fn new(n_players: usize, n_marbles: usize) -> State {
        let circle: CircularList<usize> = CircularList::with_capacity(n_marbles, 0);
        let points: Vec<usize> = (0..n_players).map(|_| 0).collect();
        State {
            n_players,
            circle,
            current_player: 0,
            points,
            next_marble: 1,
        }
    }

    fn turn(mut self) -> State {
        if self.next_marble % 23 != 0 {
            self.circle.move_right();
            self.circle.insert_right(self.next_marble);
            self.circle.move_right();
        } else {
            self.points[self.current_player] += self.next_marble;
            self.circle.move_left_n(7);
            self.points[self.current_player] += *self.circle.remove_use_right();
        };
        let next_player = (self.current_player + 1) % self.n_players;
        State {
            n_players: self.n_players,
            circle: self.circle,
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
