#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u32),
    pub p2: (String, u32),
    pub nb_games: u32,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u32) -> GameSession {
        return GameSession {
            id: id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games: nb_games,
        };
    }

    pub fn read_winner(&self) -> Option<&(String, u32)> {
        if self.p1.1 > self.p2.1 {
            return Some(&self.p1);
        } else if self.p1.1 < self.p2.1 {
            return Some(&self.p2);
        } else {
            return None;
        }
    }

    pub fn update_score(&mut self, user_name: &str) {
        if self.p1.1 + self.p2.1 <self.nb_games {
            if self.p1.0 == user_name {
                self.p1.1 += 1;
            }
            if self.p2.0 == user_name {
                self.p2.1 += 1;
            }
        }
    }

    pub fn delete(self) -> String {
        return format!("game deleted: id -> {}", self.id);
    }
}
