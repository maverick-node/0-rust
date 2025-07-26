pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if vertical('O', table) || horizontal('O', table) || diagonals('O', table) {
        return "player O won".to_string();
    }
    if vertical('X', table) || horizontal('X', table) || diagonals('X', table) {
        return "player X won".to_string();
    }
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }

    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }

    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[i][0] == player && table[i][1] == player && table[i][2] == player {
            return true;
        }
    }
    false
}
pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[0][i] == player && table[1][i] == player && table[2][i] == player {
            return true;
        }
    }
    false
}
