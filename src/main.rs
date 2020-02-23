// use rand::prelude::*;

struct Player {
    state: Vec<Vec<i32>>,
}

impl Player {
    pub fn new(total_number_of_matches: i32) -> Player {
        Player {
            state: Vec::with_capacity(total_number_of_matches as usize),
        }
    }
    pub fn draw(&mut self, current_number_of_matches: i32) -> i32 {
        1
    }
    pub fn invalid_draw(&mut self) {}
    fn seed(&mut self) {}
    fn limit(&mut self) {}
    fn loose(&mut self) {}
    fn win(&mut self) {}
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
    fn draw<'a>(&self, players: &'a mut Vec<&'a mut Player>) -> &'a Player {
        let mut current_number_of_matches = self.total_number_of_matches;
        let mut player_id = 0;
        let current_player = while current_number_of_matches > 0 {
            let current_player = players[player_id];
            while let Some(matches_played) = match current_player.draw(current_number_of_matches) {
                matches_played if matches_played >= current_number_of_matches => {
                    Some(matches_played)
                }
                _ => {
                    current_player.invalid_draw();
                    None
                }
            } {}
            player_id = (player_id + 1) % players.len();
            current_player
        };
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
        board.draw(&mut vec![&mut player0, &mut player1]);
    }
}
