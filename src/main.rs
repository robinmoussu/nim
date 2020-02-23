struct Player {
    state: Vec<Vec<i32>>,
}

impl Player {
    pub fn new(total_number_of_matches: i32) -> Player {
        Player {
            state: vec![Vec::new(); total_number_of_matches as usize],
        }
    }
    pub fn draw(&mut self, current_number_of_matches: i32) -> i32 {
        use rand::distributions::{Distribution, Uniform};

        let between = Uniform::from(1..3);
        let mut rng = rand::thread_rng();

        let state = &mut self.state[(current_number_of_matches - 1) as usize];
        if state.is_empty() {
            Self::seed(state);
        }

        return between.sample(&mut rng);
    }
    pub fn invalid_draw(&mut self) {}
    pub fn loose(&mut self) {}
    pub fn win(&mut self) {}
    fn seed(state: &mut Vec<i32>) {
        *state = vec![3, 3, 3];
    }
    fn limit(&mut self) {}
}

struct Game {
    total_number_of_matches: i32,
}

impl Game {
    fn new(total_number_of_matches: i32) -> Game {
        Game {
            total_number_of_matches,
        }
    }
    fn add_player(&self) -> Player {
        Player::new(self.total_number_of_matches)
    }
    fn play<'a>(&self, players: &'a mut Vec<&'a mut Player>) -> &'a Player {
        let mut current_number_of_matches = self.total_number_of_matches;
        let mut player_id = 0;
        while current_number_of_matches > 0 {
            let current_player = &mut players[player_id];
            let mut matches_played;
            while current_number_of_matches > 0 {
                matches_played = current_player.draw(current_number_of_matches);
                if matches_played <= current_number_of_matches {
                    current_number_of_matches -= matches_played;
                } else {
                    current_player.invalid_draw();
                }
            }
            player_id = (player_id + 1) % players.len();
        }
        for (id, player) in players.iter_mut().enumerate() {
            if player_id != id {
                player.loose();
            } else {
                println!("Player {} win", player_id);
                player.win();
            }
        }
        players[player_id]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_play() {
        let board = Game::new(18);
        let mut player0 = board.add_player();
        let mut player1 = board.add_player();
        board.play(&mut vec![&mut player0, &mut player1]);
    }
}
