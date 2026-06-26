pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let mut player = 'O';
    if horizontal(player,table) || vertical(player,table) || diagonals(player,table) {
        return "player O won".to_string()
    }
    player = 'X';
    if horizontal(player,table) || vertical(player,table) || diagonals(player,table) {
        return "player X won".to_string()
    }
    return "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        println!("1" , );
        return true
    } else if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        println!("10");
        return true
    }
    return false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table {
        if row[0] == player && row[1] == player && row[2] == player {
            println!("2");
            return true
        }
    }
    return false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[0][i] == player && table[1][i] == player && table[2][i] == player {
            println!("3");
            return true
        }
    }
    return false
}