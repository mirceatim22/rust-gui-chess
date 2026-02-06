use sfml::cpp::FBox;
use sfml::system::{Clock, Vector2f};
type Mutaree = (String, (usize, usize), (usize, usize));
use sfml::{graphics::*, system::*, window::mouse::*, window::*};
pub struct PermisiuniRocada {
    pub white_big_castle: bool,
    pub white_small_castle: bool,
    pub black_big_castle: bool,
    pub black_small_castle: bool,
    pub rocada_mica: bool,
    pub rocada_mare: bool,
}
pub struct DetaliiJoc {
    pub rocada: PermisiuniRocada,
    pub enpassant: bool,
    pub transformari: (bool, bool),
    pub turn: i32,
    pub lista_mutare: Vec<Mutaree>,
    pub index_mutare: usize,
    pub mesaj: String,
    pub remiza_prin_repetare: bool,
    pub drag_info: (i32, (usize, usize), (f32, f32), String),
    pub mutari_pos: Vec<(usize, usize)>,
    pub select: (usize, usize),
    pub timp: (f32, f32, f32, f32),
    pub scroll_offset: usize,
    pub max_scroll_offset: usize,
    pub pozitii: Vec<(Vec<Vec<String>>, i32, i32)>,
    pub gamestate: i32,
    pub loop_cnt: bool,
}
//use sfml::{graphics::*, system::*, window::mouse::*, window::*};
/*use std::io;*/
pub fn init_board(board: &mut [Vec<String>]) {
    board[0][0].push_str("bR");
    board[0][1].push_str("bN");
    board[0][2].push_str("bB");
    board[0][3].push_str("bQ");
    board[0][4].push_str("bK");
    board[0][5].push_str("bB");
    board[0][6].push_str("bN");
    board[0][7].push_str("bR");
    for i in board[1].iter_mut().take(8) {
        i.push_str("bP");
    }
    board[7][0].push_str("wR");
    board[7][1].push_str("wN");
    board[7][2].push_str("wB");
    board[7][3].push_str("wQ");
    board[7][4].push_str("wK");
    board[7][5].push_str("wB");
    board[7][6].push_str("wN");
    board[7][7].push_str("wR");
    for i in board[6].iter_mut().take(8) {
        i.push_str("wP");
    }
}
pub fn king_position(board: &mut [Vec<String>], color: i32) -> (usize, usize) {
    if color == 0 {
        for i in board.iter().enumerate().take(8) {
            for j in i.1.iter().enumerate().take(8) {
                if j.1 == "wK" {
                    return (i.0, j.0);
                }
            }
        }
    } else if color == 1 {
        for i in board.iter().enumerate().take(8) {
            for j in i.1.iter().enumerate().take(8) {
                if j.1 == "bK" {
                    return (i.0, j.0);
                }
            }
        }
    }
    (9, 9)
}
pub fn in_check(board: &mut [Vec<String>], row: usize, col: usize, color: usize) -> bool {
    //sah de la un alt rege
    let d1: Vec<i32> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let d2: Vec<i32> = vec![-1, 0, 1, -1, 1, -1, 0, 1];
    for i in 0..8 {
        let r = row as i32 + d1[i];
        let c = col as i32 + d2[i];
        if (0..8).contains(&r) && (0..8).contains(&c) && ((color == 0 && board[r as usize][c as usize] == "bK") || (color == 1 && board[r as usize][c as usize] == "wK")){
            return true;
        }
    }
    //sah de la un pion
    if color == 0 {
        if row > 0 && col > 0 && board[row - 1][col - 1] == "bP" {
            return true;
        }
        if row > 0 && col < 7 && board[row - 1][col + 1] == "bP" {
            return true;
        }
    } else if color == 1 {
        if row < 7 && col > 0 && board[row + 1][col - 1] == "wP" {
            return true;
        }
        if row < 7 && col < 7 && board[row + 1][col + 1] == "wP" {
            return true;
        }
    }
    if check_from_knight(board, row, col, color){
        return true;
    }
    if check_from_queen_rook(board, row, col, color){
        return true;
    }  
    if check_from_queen_bishop(board, row, col, color){
        return true;
    }    
    false
}
fn check_from_knight(board: &mut [Vec<String>], row: usize, col: usize, color: usize) -> bool{
    let drow: Vec<i32> = vec![-1, 1, 2, 2, 1, -1, -2, -2];
    let dcol: Vec<i32> = vec![-2, -2, -1, 1, 2, 2, 1, -1];
    for k in 0..8 {
        let nrow = row as i32 + drow[k];
        let ncol = col as i32 + dcol[k];
        if nrow >= 0 && ncol >= 0 && nrow < 8 && ncol < 8 {
            if color == 0 && board[nrow as usize][ncol as usize] == "bN" {
                return true;
            }
            if color == 1 && board[nrow as usize][ncol as usize] == "wN" {
                return true;
            }
        }
    }
    false
}
fn check_from_queen_rook(board: &mut [Vec<String>], row: usize, col: usize, color: usize) -> bool {
    let mut nrow = row as i32 - 1;
    while nrow >= 0 {
        if !board[nrow as usize][col].is_empty() {
            if color == 0 {
                if board[nrow as usize][col] == "bR" || board[nrow as usize][col] == "bQ" {
                    return true;
                } else {
                    break;
                }
            } else if color == 1 {
                if board[nrow as usize][col] == "wR" || board[nrow as usize][col] == "wQ" {
                    return true;
                } else {
                    break;
                }
            }
        }
        nrow -= 1;
    }
    nrow = row as i32 + 1;
    while nrow < 8 {
        if !board[nrow as usize][col].is_empty() {
            if color == 0 {
                if board[nrow as usize][col] == "bR" || board[nrow as usize][col] == "bQ" {
                    return true;
                } else {
                    break;
                }
            } else if color == 1 {
                if board[nrow as usize][col] == "wR" || board[nrow as usize][col] == "wQ" {
                    return true;
                } else {
                    break;
                }
            }
        }
        nrow += 1;
    }
    let mut ncol = col as i32 - 1;
    while ncol >= 0 {
        if !board[row][ncol as usize].is_empty() {
            if color == 0 {
                if board[row][ncol as usize] == "bR" || board[row][ncol as usize] == "bQ" {
                    return true;
                } else {
                    break;
                }
            } else if color == 1 {
                if board[row][ncol as usize] == "wR" || board[row][ncol as usize] == "wQ" {
                    return true;
                } else {
                    break;
                }
            }
        }
        ncol -= 1;
    }
    ncol = col as i32 + 1;
    while ncol < 8 {
        if !board[row][ncol as usize].is_empty() {
            if color == 0 {
                if board[row][ncol as usize] == "bR" || board[row][ncol as usize] == "bQ" {
                    return true;
                } else {
                    break;
                }
            } else if color == 1 {
                if board[row][ncol as usize] == "wR" || board[row][ncol as usize] == "wQ" {
                    return true;
                } else {
                    break;
                }
            }
        }
        ncol += 1;
    }
    false
}
fn check_from_queen_bishop(board: &mut [Vec<String>], row: usize, col: usize, color: usize) -> bool {
    let mut nrow = row as i32 - 1;
    let mut ncol = col as i32 - 1;
    while nrow >= 0 && ncol >= 0 {
        if !board[nrow as usize][ncol as usize].is_empty() {
            if color == 0 {
                if board[nrow as usize][ncol as usize] == "bB"
                    || board[nrow as usize][ncol as usize] == "bQ"
                {
                    return true;
                } else {
                    break;
                }
            }
            if color == 1 {
                if board[nrow as usize][ncol as usize] == "wB"
                    || board[nrow as usize][ncol as usize] == "wQ"
                {
                    return true;
                } else {
                    break;
                }
            }
        }
        nrow -= 1;
        ncol -= 1;
    }
    nrow = row as i32 - 1;
    ncol = col as i32 + 1;
    while nrow >= 0 && ncol < 8 {
        if !board[nrow as usize][ncol as usize].is_empty() {
            if color == 0 {
                if board[nrow as usize][ncol as usize] == "bB"
                    || board[nrow as usize][ncol as usize] == "bQ"
                {
                    return true;
                } else {
                    break;
                }
            }
            if color == 1 {
                if board[nrow as usize][ncol as usize] == "wB"
                    || board[nrow as usize][ncol as usize] == "wQ"
                {
                    return true;
                } else {
                    break;
                }
            }
        }
        nrow -= 1;
        ncol += 1;
    }
    nrow = row as i32 + 1;
    ncol = col as i32 - 1;
    while nrow < 8 && ncol >= 0 {
        if !board[nrow as usize][ncol as usize].is_empty() {
            if color == 0 {
                if board[nrow as usize][ncol as usize] == "bB"
                    || board[nrow as usize][ncol as usize] == "bQ"
                {
                    return true;
                } else {
                    break;
                }
            }
            if color == 1 {
                if board[nrow as usize][ncol as usize] == "wB"
                    || board[nrow as usize][ncol as usize] == "wQ"
                {
                    return true;
                } else {
                    break;
                }
            }
        }
        nrow += 1;
        ncol -= 1;
    }
    nrow = row as i32 + 1;
    ncol = col as i32 + 1;
    while nrow < 8 && ncol < 8 {
        if !board[nrow as usize][ncol as usize].is_empty() {
            if color == 0 {
                if board[nrow as usize][ncol as usize] == "bB"
                    || board[nrow as usize][ncol as usize] == "bQ"
                {
                    return true;
                } else {
                    break;
                }
            }
            if color == 1 {
                if board[nrow as usize][ncol as usize] == "wB"
                    || board[nrow as usize][ncol as usize] == "wQ"
                {
                    return true;
                } else {
                    break;
                }
            }
        }
        nrow += 1;
        ncol += 1;
    }
    false
}
pub fn mutari_posibile(board: &mut [Vec<String>], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    let ch: Vec<char> = board[row][col].trim().chars().collect();
    let color = if ch[0] == 'w' { 0 } else { 1 };
    let white_king_pos: (usize, usize) = king_position(board, 0);
    let black_king_pos: (usize, usize) = king_position(board, 1);
    if ch[1] == 'P' {
        if color == 0 {
            res = mutari_posibile_pawn(board, row, col, color, white_king_pos);            
        }
        if color == 1 {
            res = mutari_posibile_pawn(board, row, col, color, black_king_pos);           
        }
    }
    if ch[1] == 'N' {
        res = mutari_posibile_knight(board, row, col, color, white_king_pos, black_king_pos, ch.clone());        
    }
    if ch[1] == 'B' {
        res = mutari_posibile_bishop(board, row, col, color, white_king_pos, black_king_pos, ch.clone());        
    }
    if ch[1] == 'R' {
        res = mutari_posibile_rook(board, row, col, color, white_king_pos, black_king_pos, ch.clone());        
    }
    if ch[1] == 'Q' {
        res = mutari_posibile_queen(board, row, col, color, white_king_pos, black_king_pos, ch.clone());        
    }
    if ch[1] == 'K' {
        res = mutari_posibile_king(board, row, col, color, ch.clone());
    }
    res
}
fn mutari_posibile_pawn(board: &mut [Vec<String>], row: usize, col: usize, color: usize, king_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    if color == 0 {
        if board[row - 1][col].is_empty() {
            let mut p = String::new();
            p.push_str(&board[row][col]);
            board[row][col].clear();
            board[row - 1][col].push_str(&p);
            if !in_check(board, king_pos.0, king_pos.1, 0){
                res.push((row - 1, col));
            }
            board[row - 1][col].clear();
            board[row][col].push_str(&p);
        }
        if row == 6 && board[row - 2][col].is_empty() && board[row - 1][col].is_empty() {
            let mut p = String::new();
            p.push_str(&board[row][col]);
            board[row][col].clear();
            board[row - 2][col].push_str(&p);
            if !in_check(board, king_pos.0, king_pos.1, 0){
                res.push((4, col));
            }
            board[row - 2][col].clear();
            board[row][col].push_str(&p);
        }
        if col > 0 && !board[row - 1][col - 1].is_empty() {
            let c: Vec<char> = board[row - 1][col - 1].trim().chars().collect();
            if c[0] == 'b' {
                let mut p: String = String::new();
                p.push_str(&board[row][col]);
                board[row][col].clear();
                let mut b: String = String::new();
                b.push_str(&board[row - 1][col - 1]);
                board[row - 1][col - 1].clear();
                board[row - 1][col - 1].push_str(&p);
                if !in_check(board, king_pos.0, king_pos.1, 0){
                    res.push((row - 1, col - 1));
                }
                board[row][col].push_str(&p);
                board[row - 1][col - 1].clear();
                board[row - 1][col - 1].push_str(&b);
            }
        }
        if col < 7 && !board[row - 1][col + 1].is_empty() {
            let c: Vec<char> = board[row - 1][col + 1].trim().chars().collect();
            if c[0] == 'b' {
                let mut p: String = String::new();
                p.push_str(&board[row][col]);
                board[row][col].clear();
                let mut b: String = String::new();
                b.push_str(&board[row - 1][col + 1]);
                board[row - 1][col + 1].clear();
                board[row - 1][col + 1].push_str(&p);
                if !in_check(board, king_pos.0, king_pos.1, 0){
                    res.push((row - 1, col + 1));
                }
                board[row][col].push_str(&p);
                board[row - 1][col + 1].clear();
                board[row - 1][col + 1].push_str(&b);
            }
        }
    }
    if color == 1 {
        if board[row + 1][col].is_empty() {
            let mut p: String = String::new();
            p.push_str(&board[row][col]);
            board[row][col].clear();
            board[row + 1][col].push_str(&p);
            if !in_check(board, king_pos.0, king_pos.1, 1){
                res.push((row + 1, col));
            }
            board[row + 1][col].clear();
            board[row][col].push_str(&p);
        }
        if row == 1 && board[row + 2][col].is_empty() && board[row + 1][col].is_empty() {
            let mut p: String = String::new();
            p.push_str(&board[row][col]);
            board[row][col].clear();
            board[row + 2][col].push_str(&p);
            if !in_check(board, king_pos.0, king_pos.1, 1){
                res.push((row + 2, col));
            }
            board[row + 2][col].clear();
            board[row][col].push_str(&p);
        }
        if col > 0 && !board[row + 1][col - 1].is_empty() {
            let c: Vec<char> = board[row + 1][col - 1].trim().chars().collect();
            if c[0] == 'w' {
                let mut p = String::new();
                p.push_str(&board[row][col]);
                board[row][col].clear();
                let mut b = String::new();
                b.push_str(&board[row + 1][col - 1]);
                board[row + 1][col - 1].clear();
                board[row + 1][col - 1].push_str(&p);
                if !in_check(board, king_pos.0, king_pos.1, 1){
                    res.push((row + 1, col - 1));
                }
                board[row + 1][col - 1].clear();
                board[row + 1][col - 1].push_str(&b);
                board[row][col].push_str(&p);
            }
        }
        if col < 7 && !board[row + 1][col + 1].is_empty() {
            let c: Vec<char> = board[row + 1][col + 1].trim().chars().collect();
            if c[0] == 'w' {
                let mut p = String::new();
                p.push_str(&board[row][col]);
                board[row][col].clear();
                let mut b = String::new();
                b.push_str(&board[row + 1][col + 1]);
                board[row + 1][col + 1].clear();
                board[row + 1][col + 1].push_str(&p);
                if !in_check(board, king_pos.0, king_pos.1, 1){
                    res.push((row + 1, col + 1));
                }
                board[row + 1][col + 1].clear();
                board[row + 1][col + 1].push_str(&b);
                board[row][col].push_str(&p);
            }
        }
    }
    res
}
fn mutari_posibile_knight(board: &mut [Vec<String>], row: usize, col: usize, color: usize, white_king_pos: (usize, usize), black_king_pos: (usize, usize), ch: Vec<char>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    let drow: Vec<i32> = vec![-1, 1, 2, 2, 1, -1, -2, -2];
    let dcol: Vec<i32> = vec![-2, -2, -1, 1, 2, 2, 1, -1];
    for k in 0..8 {
        let nrow = row as i32 + drow[k];
        let ncol = col as i32 + dcol[k];
        if nrow >= 0
            && ncol >= 0
            && nrow < 8
            && ncol < 8
            && board[nrow as usize][ncol as usize].is_empty()
        {
            let mut p = String::new();
            p.push_str(&board[row][col]);
            board[row][col].clear();
            board[nrow as usize][ncol as usize].push_str(&p);
            if (color == 0 && !in_check(board, white_king_pos.0, white_king_pos.1, 0))
                || (color == 1 && !in_check(board, black_king_pos.0, black_king_pos.1, 1))
            {
                res.push((nrow as usize, ncol as usize));
            }
            board[nrow as usize][ncol as usize].clear();
            board[row][col].push_str(&p);
        } else if nrow >= 0 && ncol >= 0 && nrow < 8 && ncol < 8 {
            let c: Vec<char> = board[nrow as usize][ncol as usize].trim().chars().collect();
            if ch[0] != c[0] {
                let mut p = String::new();
                p.push_str(&board[row][col]);
                let mut b = String::new();
                b.push_str(&board[nrow as usize][ncol as usize]);
                board[row][col].clear();
                board[nrow as usize][ncol as usize].clear();
                board[nrow as usize][ncol as usize].push_str(&p);
                if (color == 0 && !in_check(board, white_king_pos.0, white_king_pos.1, 0))
                    || (color == 1 && !in_check(board, black_king_pos.0, black_king_pos.1, 1))
                {
                    res.push((nrow as usize, ncol as usize));
                }
                board[nrow as usize][ncol as usize].clear();
                board[nrow as usize][ncol as usize].push_str(&b);
                board[row][col].push_str(&p);
            }
        }
    }
    res
}
fn mutari_posibile_bishop(board: &mut [Vec<String>], row: usize, col: usize, color: usize, white_king_pos: (usize, usize), black_king_pos: (usize, usize), ch: Vec<char>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for (dr, dc) in directions {
        let mut nrow = row as i32 + dr;
        let mut ncol = col as i32 + dc;
        while (0..8).contains(&nrow) && (0..8).contains(&ncol){
            let target = &board[nrow as usize][ncol as usize];
            if target.is_empty() {
                let piece = board[row][col].clone();
                board[row][col].clear();
                board[nrow as usize][ncol as usize].push_str(&piece);
                if (color == 0 && !in_check(board, white_king_pos.0, white_king_pos.1, 0)) || (color == 1 && !in_check(board, black_king_pos.0, black_king_pos.1, 1)){
                    res.push((nrow as usize, ncol as usize));
                }
                board[nrow as usize][ncol as usize].clear();
                board[row][col].push_str(&piece);
            } else {
                let c: Vec<char> = target.trim().chars().collect();
                if ch[0] != c[0] {
                    let piece = board[row][col].clone();
                    let backup = target.clone();
                    board[row][col].clear();
                    board[nrow as usize][ncol as usize].clear();
                    board[nrow as usize][ncol as usize].push_str(&piece);
                    if (color == 0 && !in_check(board, white_king_pos.0, white_king_pos.1, 0)) || (color == 1 && !in_check(board, black_king_pos.0, black_king_pos.1, 1)){
                        res.push((nrow as usize, ncol as usize));
                    }
                    board[nrow as usize][ncol as usize].clear();
                    board[nrow as usize][ncol as usize].push_str(&backup);
                    board[row][col].push_str(&piece);
                }
                break;
            }
            nrow += dr;
            ncol += dc;
        }
    }
    res
}
fn mutari_posibile_rook(board: &mut [Vec<String>], row: usize, col: usize, color: usize, white_king_pos: (usize, usize), black_king_pos: (usize, usize), ch: Vec<char>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dr, dc) in directions {
        let mut nrow = row as i32 + dr;
        let mut ncol = col as i32 + dc;
        while (0..8).contains(&nrow) && (0..8).contains(&ncol) {
            let target = &board[nrow as usize][ncol as usize];
            if target.is_empty() {
                let piece = board[row][col].clone();
                board[row][col].clear();
                board[nrow as usize][ncol as usize].push_str(&piece);
                if (color == 0 && !in_check(board, white_king_pos.0, white_king_pos.1, 0)) || (color == 1 && !in_check(board, black_king_pos.0, black_king_pos.1, 1)){
                    res.push((nrow as usize, ncol as usize));
                }
                board[nrow as usize][ncol as usize].clear();
                board[row][col].push_str(&piece);
            } else {
                let c: Vec<char> = target.trim().chars().collect();
                if ch[0] != c[0] {
                    let piece = board[row][col].clone();
                    let backup = target.clone();
                    board[row][col].clear();
                    board[nrow as usize][ncol as usize].clear();
                    board[nrow as usize][ncol as usize].push_str(&piece);
                    if (color == 0 && !in_check(board, white_king_pos.0, white_king_pos.1, 0)) || (color == 1 && !in_check(board, black_king_pos.0, black_king_pos.1, 1)){
                        res.push((nrow as usize, ncol as usize));
                    }
                    board[nrow as usize][ncol as usize].clear();
                    board[nrow as usize][ncol as usize].push_str(&backup);
                    board[row][col].push_str(&piece);
                }
                break;
            }
            nrow += dr;
            ncol += dc;
        }
    }
    res
}
fn mutari_posibile_queen(board: &mut [Vec<String>], row: usize, col: usize, color: usize, white_king_pos: (usize, usize), black_king_pos: (usize, usize), ch: Vec<char>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    res.extend(mutari_posibile_bishop(board, row, col, color, white_king_pos, black_king_pos, ch.clone()));
    res.extend(mutari_posibile_rook(board, row, col, color, white_king_pos, black_king_pos, ch.clone()));
    res
}
fn mutari_posibile_king(board: &mut [Vec<String>], row: usize, col: usize, color: usize, ch: Vec<char>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    let drow: Vec<i32> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
        let dcol: Vec<i32> = vec![-1, 0, 1, -1, 1, -1, 0, 1];
        for i in 0..8 {
            let ncol = col as i32 + dcol[i];
            let nrow = row as i32 + drow[i];
            if (0..8).contains(&ncol) && (0..8).contains(&nrow) {
                if board[nrow as usize][ncol as usize].is_empty() {
                    let mut p = String::new();
                    p.push_str(&board[row][col]);
                    board[nrow as usize][ncol as usize].push_str(&p);
                    board[row][col].clear();
                    if !in_check(board, nrow as usize, ncol as usize, color){
                        res.push((nrow as usize, ncol as usize));
                    }
                    board[nrow as usize][ncol as usize].clear();
                    board[row][col].push_str(&p);
                }
                if !board[nrow as usize][ncol as usize].is_empty() {
                    let c: Vec<char> = board[nrow as usize][ncol as usize].trim().chars().collect();
                    if (ch[0] != c[0]) && !in_check(board, nrow as usize, ncol as usize, color)
                    {
                        let mut p = String::new();
                        p.push_str(&board[row][col]);
                        let mut b = String::new();
                        b.push_str(&board[nrow as usize][ncol as usize]);
                        board[row][col].clear();
                        board[nrow as usize][ncol as usize].clear();
                        board[nrow as usize][ncol as usize].push_str(&p);
                        if !in_check(board, nrow as usize, ncol as usize, color){
                            res.push((nrow as usize, ncol as usize));
                        }
                        board[nrow as usize][ncol as usize].clear();
                        board[row][col].push_str(&p);
                        board[nrow as usize][ncol as usize].push_str(&b);
                    }
                }
            }
        }
    res
}
pub fn checkmate(board: &mut [Vec<String>], color: i32) -> bool {
    let rege_pos: (usize, usize) = king_position(board, color);
    if !in_check(board, rege_pos.0, rege_pos.1, color as usize){
        return false;
    }
    //apoi verifica daca poti scapa de sah mutand o alta piesa
    for i in 0..8 {
        for j in 0..8 {
            if !board[i][j].is_empty() {
                let c: Vec<char> = board[i][j].trim().chars().collect();
                if (color == 0 && c[0] == 'w') || (color == 1 && c[0] == 'b') {
                    let mut_posibile = mutari_posibile(board, i, j);
                    if !mut_posibile.is_empty() {
                        for a in mut_posibile {
                            let nrow = a.0;
                            let ncol = a.1;
                            println!(
                                "Incercam sa mutam {} la pozitia {} {}",
                                board[i][j], nrow, ncol
                            );
                            let mut p = String::new();
                            p.push_str(&board[i][j]);
                            let mut b = String::new();
                            if !board[nrow][ncol].is_empty() {
                                b.push_str(&board[nrow][ncol]);
                                board[nrow][ncol].clear();
                            }
                            board[i][j].clear();
                            board[nrow][ncol].push_str(&p);
                            let pozitia_actuala = king_position(board, color);
                            if !in_check(board, pozitia_actuala.0, pozitia_actuala.1, color as usize)
                            {
                                board[i][j].push_str(&p);
                                board[nrow][ncol].clear();
                                if !b.is_empty() {
                                    board[nrow][ncol].push_str(&b);
                                }
                                return false;
                            }
                            board[i][j].push_str(&p);
                            board[nrow][ncol].clear();
                            if !b.is_empty() {
                                board[nrow][ncol].push_str(&b);
                            }
                        }
                    }
                }
            }
        }
    }
    if color == 0 {
        println!("Sah mat! Negrul castiga!");
    } else {
        println!("Sah mat! Albul castiga!");
    }
    true
}
pub fn stalemate(board: &mut [Vec<String>], color: i32) -> bool {
    let king_pos = king_position(board, color);
    if in_check(board, king_pos.0, king_pos.1, color as usize){
        return false;
    }
    let c: char = if color == 0 { 'w' } else { 'b' };
    for i in 0..8 {
        for j in 0..8 {
            if !board[i][j].is_empty() {
                let ch: Vec<char> = board[i][j].trim().chars().collect();
                if ch[0] == c {
                    let mutari_pos = mutari_posibile(board, i, j);
                    if !mutari_pos.is_empty() {
                        return false;
                    }
                }
            }
        }
    }
    true
}
pub fn material_insuficient(board: &mut [Vec<String>]) -> bool {
    let mut bishop_cnt = 0;
    let mut knight_cnt = 0;
    for a in board {
        for b in a {
            if !b.is_empty() {
                let c: Vec<char> = b.trim().chars().collect();
                if c[1] == 'B' {
                    bishop_cnt += 1;
                    if bishop_cnt == 2 {
                        return false;
                    }
                }
                if c[1] == 'N' {
                    knight_cnt += 1;
                    if knight_cnt == 2 {
                        return false;
                    }
                }
                if c[1] == 'Q' {
                    return false;
                }
                if c[1] == 'R' {
                    return false;
                }
                if c[1] == 'P' {
                    return false;
                }
            }
        }
    }
    if bishop_cnt == 1 && knight_cnt == 1 {
        return false;
    }
    false
}
pub fn small_castle_possible(board: &mut [Vec<String>], color: i32) -> i32 {
    if color == 0 {
        if !board[7][5].is_empty() || !board[7][6].is_empty() {
            return 0;
        }
        if in_check(board, 7, 4, 0){
            return 0;
        }
        let mut p = String::new();
        p.push_str(&board[7][4]);
        board[7][4].clear();
        board[7][5].push_str(&p);
        if in_check(board, 7, 5, 0){
            board[7][5].clear();
            board[7][4].push_str(&p);
            return 0;
        }
        board[7][5].clear();
        board[7][6].push_str(&p);
        if in_check(board, 7, 6, 0){
            board[7][6].clear();
            board[7][4].push_str(&p);
            return 0;
        }
        board[7][6].clear();
        board[7][4].push_str(&p);
    }
    if color == 1 {
        if !board[0][5].is_empty() || !board[0][6].is_empty() {
            return 0;
        }
        if in_check(board, 0, 4, 1){
            return 0;
        }
        let mut p = String::new();
        p.push_str(&board[0][4]);
        board[0][4].clear();
        board[0][5].push_str(&p);
        if in_check(board, 0, 5, 1){
            board[0][5].clear();
            board[0][4].push_str(&p);
            return 0;
        }
        board[0][5].clear();
        board[0][6].push_str(&p);
        if in_check(board, 0, 6, 1){
            board[0][6].clear();
            board[0][4].push_str(&p);
            return 0;
        }
        board[0][6].clear();
        board[0][4].push_str(&p);
    }
    1
}
pub fn big_castle_possible(board: &mut [Vec<String>], color: i32) -> i32 {
    if color == 0 {
        if !board[7][1].is_empty() || !board[7][2].is_empty() || !board[7][3].is_empty() {
            return 0;
        }
        if in_check(board, 7, 4, 0){
            return 0;
        }
        let mut p = String::new();
        p.push_str(&board[7][4]);
        board[7][4].clear();
        board[7][3].push_str(&p);
        if in_check(board, 7, 3, 0){
            board[7][3].clear();
            board[7][4].push_str(&p);
            return 0;
        }
        board[7][3].clear();
        board[7][2].push_str(&p);
        if in_check(board, 7, 2, 0) {
            board[7][2].clear();
            board[7][4].push_str(&p);
            return 0;
        }
        board[7][2].clear();
        board[7][4].push_str(&p);
    }
    if color == 1 {
        if !board[0][1].is_empty() || !board[0][3].is_empty() || !board[0][2].is_empty() {
            return 0;
        }
        if in_check(board, 0, 4, 1){
            return 0;
        }
        let mut p = String::new();
        p.push_str(&board[0][4]);
        board[0][4].clear();
        board[0][3].push_str(&p);
        if in_check(board, 0, 3, 1){
            board[0][3].clear();
            board[0][4].push_str(&p);
            return 0;
        }
        board[0][3].clear();
        board[0][2].push_str(&p);
        if in_check(board, 0, 2, 1){
            board[0][2].clear();
            board[0][4].push_str(&p);
            return 0;
        }
        board[0][2].clear();
        board[0][4].push_str(&p);
    }
    1
}
pub fn ceas(timp: f32) -> String {
    let mins = (timp / 60.0).floor() as i32;
    let seconds = timp % 60.0;
    let n = (seconds.fract() * 100.0).round() as i32;
    format!("{:.0}:{:02}.{:02}", mins, seconds as i32, n)
}
pub fn text_mutare(mutare: &(String, (usize, usize), (usize, usize))) -> String {
    let mut res = String::new();
    let c: Vec<char> = mutare.0.trim().chars().collect();
    if c[1] == 'K' && mutare.1.1 == 4 {
        if mutare.2.1 == 2 {
            return "0-0-0".to_string();
        }
        if mutare.2.1 == 6 {
            return "0-0".to_string();
        }
    }
    if c[1] != 'P' {
        res.push(c[1]);
    }
    res.push((b'a' + mutare.1.1 as u8) as char);
    res.push((b'8' - mutare.1.0 as u8) as char);
    res.push('-');
    res.push((b'a' + mutare.2.1 as u8) as char);
    res.push((b'8' - mutare.2.0 as u8) as char);
    res
}
pub fn desenare_tabla(width: f32, square_size: f32, height: f32, window: &mut FBox<RenderWindow>, font: &FBox<Font>) {
    desenare_patrate_margine_tabla(width, square_size, height, window, font);
}
pub fn desenare_lista(width: f32, square_size: f32, window: &mut FBox<RenderWindow>, lista_mutare: &[Mutaree], font: &FBox<Font>, scroll_offset: usize) {
    let mouse_pos = window.mouse_position();
            let mut buton_restart = RectangleShape::new();
            buton_restart.set_outline_color(Color::BLACK);
            buton_restart.set_outline_thickness(square_size / 20.0);
            buton_restart.set_fill_color(Color::rgb(210, 210, 210));
            if mouse_pos.x as f32 >= square_size * 0.89
                && mouse_pos.x as f32 <= square_size * 2.89
                && mouse_pos.y as f32 >= 4.5 * square_size
                && mouse_pos.y as f32 <= 5.5 * square_size
            {
                buton_restart.set_fill_color(Color::rgb(222, 160, 160));
            }
            buton_restart.set_size((square_size * 2.0, square_size));
            buton_restart.set_position((square_size * 0.89, 4.5 * square_size));
            let mut text_restart = Text::new(&"Restart".to_string(), font, square_size as u32 / 2);
            text_restart.set_fill_color(Color::BLACK);
            text_restart.set_position((square_size, 4.65 * square_size));
            window.draw(&buton_restart);
            window.draw(&text_restart);
            let mut coloane_dreptunghi = RectangleShape::new();
            coloane_dreptunghi.set_size((3.5 * square_size, 0.5 * square_size));
            coloane_dreptunghi.set_fill_color(Color::WHITE);
            coloane_dreptunghi.set_outline_thickness(square_size / 20.0);
            coloane_dreptunghi.set_outline_color(Color::BLACK);
            coloane_dreptunghi.set_position((width + 9.4 * square_size, square_size / 2.0));
            window.draw(&coloane_dreptunghi);
            let mut txt_nr = Text::new(&"Nr.".to_string(), font, square_size as u32 / 4);
            txt_nr.set_position((width + 9.42 * square_size, square_size / 1.82));
            txt_nr.set_fill_color(Color::BLACK);
            window.draw(&txt_nr);
            let mut txt_white = Text::new(&"White".to_string(), font, square_size as u32 / 4);
            txt_white.set_position((width + 10.15 * square_size, square_size / 1.82));
            txt_white.set_fill_color(Color::BLACK);
            window.draw(&txt_white);
            let mut txt_black = Text::new(&"Black".to_string(), font, square_size as u32 / 4);
            txt_black.set_position((width + 11.75 * square_size, square_size / 1.82));
            txt_black.set_fill_color(Color::BLACK);
            window.draw(&txt_black);
            let mut mutari_dreptunghi = RectangleShape::new();
            mutari_dreptunghi.set_size((3.5 * square_size, 8.0 * square_size));
            mutari_dreptunghi.set_position((width + 9.4 * square_size, square_size));
            mutari_dreptunghi.set_fill_color(Color::WHITE);
            mutari_dreptunghi.set_outline_thickness(square_size / 20.0);
            mutari_dreptunghi.set_outline_color(Color::BLACK);
            let mut linie = RectangleShape::new();
            linie.set_size((square_size / 20.0, 8.5 * square_size));
            linie.set_fill_color(Color::BLACK);
            linie.set_position((width + 9.8 * square_size, square_size / 2.0));
            window.draw(&mutari_dreptunghi);
            window.draw(&linie);
            linie.set_position((width + 11.3 * square_size, square_size / 2.0));
            window.draw(&linie);
            let mut height_mutari = square_size;
            let width_mutari = width + 10.0 * square_size;
            let mutari_font_size = square_size as u32 / 4;
            let mut culoare_mutare = 0;
            for (i, a) in lista_mutare.iter().enumerate() {
                if i / 2 >= scroll_offset && i / 2 <= scroll_offset + 15 {
                    let mut nr_text = ((i / 2) + 1).to_string();
                    nr_text.push('.');
                    let mut nr = Text::new(&nr_text, font, mutari_font_size);
                    nr.set_fill_color(Color::BLACK);
                    nr.set_position((width + 9.5 * square_size, height_mutari));
                    window.draw(&nr);
                    let txt = text_mutare(a);
                    let mut text = Text::new(&txt, font, mutari_font_size);
                    text.set_fill_color(Color::BLACK);
                    text.set_position((
                        width_mutari + culoare_mutare as f32 * square_size * 1.6,
                        height_mutari,
                    ));
                    let mut linie2 = RectangleShape::new();
                    linie2.set_size((3.5 * square_size, square_size / 20.0));
                    linie2.set_fill_color(Color::BLACK);
                    linie2.set_position((
                        width + 9.4 * square_size,
                        height_mutari + square_size / 3.0,
                    ));
                    if height_mutari + (square_size / 3.0) * 2.0 < 9.0 * square_size {
                        window.draw(&linie2);
                    }
                    if culoare_mutare == 1 {
                        height_mutari += square_size / 2.0;
                    }
                    culoare_mutare = 1 - culoare_mutare;
                    window.draw(&text);
                }
            }
}
pub fn init_texturi(texturi: &mut Vec<FBox<Texture>>) {
    let w_p = match Texture::from_file("resurse/White_Pawn.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let b_p = match Texture::from_file("resurse/Black_Pawn.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let w_n = match Texture::from_file("resurse/White_Knight.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let b_n = match Texture::from_file("resurse/Black_Knight.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let w_b = match Texture::from_file("resurse/White_Bishop.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let b_b = match Texture::from_file("resurse/Black_Bishop.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let w_r = match Texture::from_file("resurse/White_Rook.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let b_r = match Texture::from_file("resurse/Black_Rook.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let w_q = match Texture::from_file("resurse/White_Queen.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let b_q = match Texture::from_file("resurse/Black_Queen.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let w_k = match Texture::from_file("resurse/White_King.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let b_k = match Texture::from_file("resurse/Black_King.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    let highlight_texture = match Texture::from_file("resurse/highlight.png") {
        Ok(t) => t,
        Err(..) => {
            println!("eroare la incarcare textura");
            return;
        }
    };
    for i in 0..12{
        let txtura = match i {
            0 => w_p.clone(),
            1 => b_p.clone(),
            2 => w_n.clone(),
            3 => b_n.clone(),
            4 => w_b.clone(),
            5 => b_b.clone(),
            6 => w_r.clone(),
            7 => b_r.clone(),
            8 => w_q.clone(),
            9 => b_q.clone(),
            10 => w_k.clone(),
            11 => b_k.clone(),
            _=> continue,
        };
        texturi.push(txtura);
    }
    texturi.push(highlight_texture);
}
pub fn desenare_piese_highlight(board: &mut [Vec<String>], texturi: &[FBox<Texture>], window: &mut FBox<RenderWindow>, drag_info: &(i32, (usize, usize), (f32, f32), String), mutari_pos: &Vec<(usize, usize)>){
    let width: f32 = window.size().x as f32 / 5.0;
    let height: f32 = window.size().y as f32 / 10.0;
    let remaining_width: f32 = window.size().x as f32 - width;
    let remaining_height: f32 = window.size().y as f32 - height;
    let square_size = remaining_width.min(remaining_height) / 9.0;    
    let mut highlight = Sprite::with_texture(&texturi[12]);
    highlight.set_scale(0.01);
    let mut w_pawn = Sprite::with_texture(&texturi[0]);
    w_pawn.set_scale((1.75, 1.75));
    let mut b_pawn = Sprite::with_texture(&texturi[1]);
    b_pawn.set_scale((1.75, 1.75));
    let mut w_knight = Sprite::with_texture(&texturi[2]);
    w_knight.set_scale((1.75, 1.75));
    let mut b_knight = Sprite::with_texture(&texturi[3]);
    b_knight.set_scale((1.75, 1.75));
    let mut w_bishop = Sprite::with_texture(&texturi[4]);
    w_bishop.set_scale((1.75, 1.75));
    let mut b_bishop = Sprite::with_texture(&texturi[5]);
    b_bishop.set_scale((1.75, 1.75));
    let mut w_rook = Sprite::with_texture(&texturi[6]);
    w_rook.set_scale((1.75, 1.75));
    let mut b_rook = Sprite::with_texture(&texturi[7]);
    b_rook.set_scale((1.75, 1.75));
    let mut w_queen = Sprite::with_texture(&texturi[8]);
    w_queen.set_scale((1.75, 1.75));
    let mut b_queen = Sprite::with_texture(&texturi[9]);
    b_queen.set_scale((1.75, 1.75));
    let mut w_king = Sprite::with_texture(&texturi[10]);
    w_king.set_scale((1.75, 1.75));
    let mut b_king = Sprite::with_texture(&texturi[11]);
    b_king.set_scale((1.75, 1.75));
    for i in board.iter().enumerate().take(8) {
        for j in board.iter().enumerate().take(8) {
            let x = width + j.0 as f32 * square_size;
            let y = height + i.0 as f32 * square_size;
            if drag_info.0 == 1 && drag_info.1 == (i.0, j.0) {
                continue;
            }
            match board[i.0][j.0].as_str() {
                "wP" => {
                    w_pawn.set_position((x, y));
                    window.draw(&w_pawn);
                }
                "bP" => {
                    b_pawn.set_position((x, y));
                    window.draw(&b_pawn);
                }
                "wN" => {
                    w_knight.set_position((x, y));
                    window.draw(&w_knight);
                }
                "bN" => {
                    b_knight.set_position((x, y));
                    window.draw(&b_knight);
                }
                "wB" => {
                    w_bishop.set_position((x, y));
                    window.draw(&w_bishop);
                }
                "bB" => {
                    b_bishop.set_position((x, y));
                    window.draw(&b_bishop);
                }
                "wR" => {
                    w_rook.set_position((x, y));
                    window.draw(&w_rook);
                }
                "bR" => {
                    b_rook.set_position((x, y));
                    window.draw(&b_rook);
                }
                "wQ" => {
                    w_queen.set_position((x, y));
                    window.draw(&w_queen);
                }
                "bQ" => {
                    b_queen.set_position((x, y));
                    window.draw(&b_queen);
                }
                "wK" => {
                    w_king.set_position((x, y));
                    window.draw(&w_king);
                }
                "bK" => {
                    b_king.set_position((x, y));
                    window.draw(&b_king);
                }
                _ => {}
            }
        }
    }
    desenare_highlight_dragged(texturi, window, drag_info, mutari_pos);
}
pub fn desenare_highlight_dragged(texturi: &[FBox<Texture>], window: &mut FBox<RenderWindow>, drag_info: &(i32, (usize, usize), (f32, f32), String), mutari_pos: &Vec<(usize, usize)>) {
    let width: f32 = window.size().x as f32 / 5.0;
    let height: f32 = window.size().y as f32 / 10.0;
    let remaining_width: f32 = window.size().x as f32 - width;
    let remaining_height: f32 = window.size().y as f32 - height;
    let square_size = remaining_width.min(remaining_height) / 9.0;    
    let mut highlight = Sprite::with_texture(&texturi[12]);
    highlight.set_scale(0.01);
    let mut w_pawn = Sprite::with_texture(&texturi[0]);
    w_pawn.set_scale((1.75, 1.75));
    let mut b_pawn = Sprite::with_texture(&texturi[1]);
    b_pawn.set_scale((1.75, 1.75));
    let mut w_knight = Sprite::with_texture(&texturi[2]);
    w_knight.set_scale((1.75, 1.75));
    let mut b_knight = Sprite::with_texture(&texturi[3]);
    b_knight.set_scale((1.75, 1.75));
    let mut w_bishop = Sprite::with_texture(&texturi[4]);
    w_bishop.set_scale((1.75, 1.75));
    let mut b_bishop = Sprite::with_texture(&texturi[5]);
    b_bishop.set_scale((1.75, 1.75));
    let mut w_rook = Sprite::with_texture(&texturi[6]);
    w_rook.set_scale((1.75, 1.75));
    let mut b_rook = Sprite::with_texture(&texturi[7]);
    b_rook.set_scale((1.75, 1.75));
    let mut w_queen = Sprite::with_texture(&texturi[8]);
    w_queen.set_scale((1.75, 1.75));
    let mut b_queen = Sprite::with_texture(&texturi[9]);
    b_queen.set_scale((1.75, 1.75));
    let mut w_king = Sprite::with_texture(&texturi[10]);
    w_king.set_scale((1.75, 1.75));
    let mut b_king = Sprite::with_texture(&texturi[11]);
    b_king.set_scale((1.75, 1.75));
    if !mutari_pos.is_empty() {
        for a in mutari_pos {
            highlight.set_position((
                width + a.1 as f32 * square_size + square_size / 3.1,
                height + a.0 as f32 * square_size + square_size / 3.1,
            ));
            window.draw(&highlight);
        }
    }
    if drag_info.0 == 1 {
        let x = drag_info.2.0 - square_size / 2.0;
        let y = drag_info.2.1 - square_size / 2.0;
        match drag_info.3.as_str() {
            "wP" => {
                w_pawn.set_position((x, y));
                window.draw(&w_pawn);
            }
            "bP" => {
                b_pawn.set_position((x, y));
                window.draw(&b_pawn);
            }
            "wN" => {
                w_knight.set_position((x, y));
                window.draw(&w_knight);
            }
            "bN" => {
                b_knight.set_position((x, y));
                window.draw(&b_knight);
            }
            "wB" => {
                w_bishop.set_position((x, y));
                window.draw(&w_bishop);
            }
            "bB" => {
                b_bishop.set_position((x, y));
                window.draw(&b_bishop);
            }
            "wR" => {
                w_rook.set_position((x, y));
                window.draw(&w_rook);
            }
            "bR" => {
                b_rook.set_position((x, y));
                window.draw(&b_rook);
            }
            "wQ" => {
                w_queen.set_position((x, y));
                window.draw(&w_queen);
            }
            "bQ" => {
                b_queen.set_position((x, y));
                window.draw(&b_queen);
            }
            "wK" => {
                w_king.set_position((x, y));
                window.draw(&w_king);
            }
            "bK" => {
                b_king.set_position((x, y));
                window.draw(&b_king);
            }
            _ => {}
        }
    }
}
pub fn desenare_patrate_margine_tabla(width: f32, square_size: f32, height: f32, window: &mut FBox<RenderWindow>, font: &FBox<Font>) {
   for i in 0..8 {
        for j in 0..8 {
            let mut square =
                RectangleShape::with_size(Vector2f::new(square_size, square_size));
            if (i + j) % 2 == 0 {
                square.set_fill_color(Color::WHITE);
            } else {
                square.set_fill_color(Color::rgb(45, 206, 161));
            }
            square.set_position(Vector2f::new(
                width + j as f32 * square_size,
                height + i as f32 * square_size,
                ));
                window.draw(&square);
        }
    }
    let mut linie = RectangleShape::new();
    linie.set_fill_color(Color::BLACK);
    linie.set_size((8.30 * square_size, square_size / 5.0));
    linie.set_position((width - 0.15 * square_size, square_size - square_size / 5.0));
    window.draw(&linie);
    linie.set_position((width - 0.15 * square_size, 9.0 * square_size));
    window.draw(&linie);
    linie.set_size((square_size / 5.0, 8.40 * square_size));
    linie.set_position((width - square_size / 5.0, square_size - 0.2 * square_size));
    window.draw(&linie);
    linie.set_position((width + 8.0 * square_size, square_size - 0.2 * square_size));
    window.draw(&linie);
    for i in 0..8 {
        let ch = (b'a' + i) as char;
        let mut txt_litera = Text::new(&ch.to_string(), font, (0.2 * square_size) as u32);
        txt_litera.set_fill_color(Color::WHITE);
        txt_litera.set_position((
            width + 0.45 * square_size + i as f32 * square_size,
            square_size * 0.77,
        ));
        window.draw(&txt_litera);
        txt_litera.set_position((
            width + 0.45 * square_size + i as f32 * square_size,
            square_size * 8.97,
        ));
        window.draw(&txt_litera);
        let ch2 = (b'8' - i) as char;
        let mut txt_numar = Text::new(&ch2.to_string(), font, (0.2 * square_size) as u32);
        txt_numar.set_fill_color(Color::WHITE);
        txt_numar.set_position((
            width - square_size / 6.3,
            1.35 * square_size + i as f32 * square_size,
        ));
        window.draw(&txt_numar);
        txt_numar.set_position((
            width + 8.04 * square_size,
            1.35 * square_size + i as f32 * square_size,
        ));
        window.draw(&txt_numar);
    } 
}
pub fn meniu_transformare(transformare_white: bool, transformare_black: bool, window: &mut FBox<RenderWindow>, texturi: &[FBox<Texture>]) {
    let mut w_knight = Sprite::with_texture(&texturi[2]);
    w_knight.set_scale((1.75, 1.75));
    let mut b_knight = Sprite::with_texture(&texturi[3]);
    b_knight.set_scale((1.75, 1.75));
    let mut w_bishop = Sprite::with_texture(&texturi[4]);
    w_bishop.set_scale((1.75, 1.75));
    let mut b_bishop = Sprite::with_texture(&texturi[5]);
    b_bishop.set_scale((1.75, 1.75));
    let mut w_rook = Sprite::with_texture(&texturi[6]);
    w_rook.set_scale((1.75, 1.75));
    let mut b_rook = Sprite::with_texture(&texturi[7]);
    b_rook.set_scale((1.75, 1.75));
    let mut w_queen = Sprite::with_texture(&texturi[8]);
    w_queen.set_scale((1.75, 1.75));
    let mut b_queen = Sprite::with_texture(&texturi[9]);
    b_queen.set_scale((1.75, 1.75));
    let width: f32 = window.size().x as f32 / 5.0;
    let height: f32 = window.size().y as f32 / 10.0;
    let remaining_width: f32 = window.size().x as f32 - width;
    let remaining_height: f32 = window.size().y as f32 - height;
    let square_size = remaining_width.min(remaining_height) / 9.0;  
    if transformare_white {
    let mut square = RectangleShape::with_size(Vector2f::new(square_size, square_size));
    square.set_fill_color(Color::rgb(255, 215, 0));
        for i in 0..4 {
        square.set_position((width + (i + 2) as f32 * square_size, 0.0));
        square.set_outline_color(Color::BLACK);
        square.set_outline_thickness(square_size / 20.0);
        window.draw(&square);
            if i == 0 {
                w_queen.set_position((width + (i + 2) as f32 * square_size, 0.0));
                window.draw(&w_queen)
            } else if i == 1 {
                w_rook.set_position((width + (i + 2) as f32 * square_size, 0.0));
                window.draw(&w_rook);
            } else if i == 2 {
                w_bishop.set_position((width + (i + 2) as f32 * square_size, 0.0));
                window.draw(&w_bishop);
            } else if i == 3 {
                w_knight.set_position((width + (i + 2) as f32 * square_size, 0.0));
                window.draw(&w_knight);
            }
        }
    }
    if transformare_black {
        let mut square = RectangleShape::with_size(Vector2f::new(square_size, square_size));
        square.set_fill_color(Color::rgb(255, 215, 0));
        for i in 0..4 {
            square.set_position((
                width + (i + 2) as f32 * square_size,
                window.size().y as f32 - square_size,
            ));
            square.set_outline_color(Color::BLACK);
            square.set_outline_thickness(square_size / 20.0);
            window.draw(&square);
            if i == 0 {
                b_queen.set_position((
                    width + (i + 2) as f32 * square_size,
                    window.size().y as f32 - square_size,
                ));
                window.draw(&b_queen)
            } else if i == 1 {
                b_rook.set_position((
                width + (i + 2) as f32 * square_size,
                window.size().y as f32 - square_size,
                ));
                window.draw(&b_rook);
            } else if i == 2 {
                b_bishop.set_position((
                    width + (i + 2) as f32 * square_size,
                    window.size().y as f32 - square_size,
                ));
                window.draw(&b_bishop);
            } else if i == 3 {
                b_knight.set_position((
                    width + (i + 2) as f32 * square_size,
                    window.size().y as f32 - square_size,
                ));
                window.draw(&b_knight);
            }
        }
    }
}
pub fn meniu_final(window: &mut FBox<RenderWindow>, gamestate: i32, mesaj: &String, font: &FBox<Font>, buton_restart2: RectangleShape, text_restart2: Text){
    if gamestate == 2 {
        window.clear(Color::WHITE);
        let mut txt = Text::new(mesaj, font, window.size().y / 8);
        txt.set_fill_color(Color::BLACK);
        let txt_x = (window.size().x as f32 - txt.local_bounds().width) / 2.0;
        let txt_y = (window.size().y as f32 - txt.local_bounds().height) / 5.5;
        txt.set_position((txt_x, txt_y));
        window.draw(&txt);
        let mut txt2 = Text::new("White wins!", font, window.size().y / 10);
        txt2.set_fill_color(Color::BLACK);
        let txt2_x = (window.size().x as f32 - txt2.local_bounds().width) / 2.0;
        let txt2_y = (window.size().y as f32 - txt.local_bounds().height) / 3.0;
        txt2.set_position((txt2_x, txt2_y));
        window.draw(&txt2);
        let mut s = RectangleShape::new();
        s.set_fill_color(Color::rgb(255, 127, 80));
        s.set_size((window.size().x as f32 / 6.0, window.size().y as f32));
        s.set_position((0.0, 0.0));
        window.draw(&s);
        s.set_position((window.size().x as f32 - window.size().x as f32 / 6.0, 0.0));
        window.draw(&s);
        window.draw(&buton_restart2);
        window.draw(&text_restart2);
    } else if gamestate == 3 {
        window.clear(Color::BLACK);
        let mut txt = Text::new(mesaj, font, window.size().y / 8);
        txt.set_fill_color(Color::WHITE);
        let txt_x = (window.size().x as f32 - txt.local_bounds().width) / 2.0;
        let txt_y = (window.size().y as f32 - txt.local_bounds().height) / 5.5;
        txt.set_position((txt_x, txt_y));
        window.draw(&txt);
        let mut txt2 = Text::new("Black wins!", font, window.size().y / 10);
        txt2.set_fill_color(Color::WHITE);
        let txt2_x = (window.size().x as f32 - txt2.local_bounds().width) / 2.0;
        let txt2_y = (window.size().y as f32 - txt.local_bounds().height) / 3.0;
        txt2.set_position((txt2_x, txt2_y));
        window.draw(&txt2);
        let mut s = RectangleShape::new();
        s.set_fill_color(Color::rgb(229, 244, 150));
        s.set_size((window.size().x as f32 / 6.0, window.size().y as f32));
        s.set_position((0.0, 0.0));
        window.draw(&s);
        s.set_position((window.size().x as f32 - window.size().x as f32 / 6.0, 0.0));
        window.draw(&s);
        window.draw(&buton_restart2);
        window.draw(&text_restart2);
    } else if gamestate == 4 {
        window.clear(Color::rgb(180, 218, 252));
        let mut txt = Text::new(mesaj, font, window.size().y / 8);
        txt.set_fill_color(Color::BLACK);
        let txt_x = (window.size().x as f32 - txt.local_bounds().width) / 2.0;
        let txt_y = (window.size().y as f32 - txt.local_bounds().height) / 5.5;
        txt.set_position((txt_x, txt_y));
        window.draw(&txt);
        let mut txt2 = Text::new("Draw!", font, window.size().y / 10);
        txt2.set_fill_color(Color::BLACK);
        let txt2_x = (window.size().x as f32 - txt2.local_bounds().width) / 2.0;
        let txt2_y = (window.size().y as f32 - txt.local_bounds().height) / 3.0;
        txt2.set_position((txt2_x, txt2_y));
        window.draw(&txt2);
        let mut s = RectangleShape::new();
        s.set_fill_color(Color::rgb(62, 55, 243));
        s.set_size((window.size().x as f32 / 8.0, window.size().y as f32));
        s.set_position((0.0, 0.0));
        window.draw(&s);
        s.set_position((window.size().x as f32 - window.size().x as f32 / 8.0, 0.0));
        window.draw(&s);
        window.draw(&buton_restart2);
        window.draw(&text_restart2);
    }
}
pub fn update_desenare_ceas(window: &mut FBox<RenderWindow>, font: &FBox<Font>, gamestate: &mut i32, turn: i32, timp: &mut (f32, f32, f32, f32), timer: &FBox<Clock>){
    let width: f32 = window.size().x as f32 / 5.0;
    let height: f32 = window.size().y as f32 / 10.0;
    let remaining_width: f32 = window.size().x as f32 - width;
    let remaining_height: f32 = window.size().y as f32 - height;
    let square_size = remaining_width.min(remaining_height) / 9.0;  
    if turn == 0 {
        timp.2 = timp.0 - timer.elapsed_time().as_seconds();
        timp.3 = timp.1;
        if timp.2 <= 0.0 {
            *gamestate = 3;
        }
    } else {
        timp.2 = timp.0;
        timp.3 = timp.1 - timer.elapsed_time().as_seconds();
        if timp.3 <= 0.0 {
            *gamestate = 2;
        }
    }
    let text_white = ceas(timp.2);
    let text_black = ceas(timp.3);
    let mut ceas_white_text = Text::new(&text_white, font, square_size as u32 / 2);
    let mut ceas_black_text = Text::new(&text_black, font, square_size as u32 / 2);
    ceas_white_text.set_fill_color(Color::BLACK);
    ceas_black_text.set_fill_color(Color::BLACK);
    ceas_black_text.set_position((width - 2.5 * square_size, 2.0 * square_size));
    ceas_white_text.set_position((
        width - 2.5 * square_size,
        window.size().y as f32 - 2.5 * square_size,
    ));
    let mut square_time = RectangleShape::new();
    square_time.set_size((1.8 * square_size, square_size / 1.7));
    square_time.set_outline_thickness(square_size / 20.0);
    square_time.set_outline_color(Color::BLACK);
    if *gamestate == 1 {
        square_time.set_position((square_size, window.size().y as f32 - 2.5 * square_size));
        if turn == 0 {
            square_time.set_fill_color(Color::rgb(246, 93, 93));
        } else {
            square_time.set_fill_color(Color::rgb(156, 144, 144));
        }
        window.draw(&square_time);
        window.draw(&ceas_white_text);
        square_time.set_position((square_size, 2.0 * square_size));
        if turn == 1 {
            square_time.set_fill_color(Color::rgb(246, 93, 93));
        } else {
            square_time.set_fill_color(Color::rgb(156, 144, 144));
        }
        window.draw(&square_time);
        window.draw(&ceas_black_text);
    }
}
pub fn rocada_sau_enpassant(board: &mut [Vec<String>], select: (usize, usize), row: usize, col: usize, enpassant: &mut bool, rocada: &mut PermisiuniRocada){
    if board[select.0][select.1] == "wK" || board[select.0][select.1] == "bK"{
        if select.1 == 4 && col == 2 {
            rocada.rocada_mare = true;
        }
        if select.1 == 4 && col == 6 {
            rocada.rocada_mica = true;
        }
    }
    if board[select.0][select.1] == "wP" {
        if select.1 > 0 && select.1 < 7 && row == 2 && (col == select.1 - 1 || col == select.1 + 1) && board[row][col].is_empty(){
            *enpassant = true;
        }
        if select.1 == 0 && row == 2 && col == 1 && board[row][col].is_empty(){
           *enpassant = true;
        }
        if select.1 == 7 && row == 2 && col == 6 && board[row][col].is_empty(){
            *enpassant = true;
        }
    }
    if board[select.0][select.1] == "bP" {
        if select.1 > 0 && select.1 < 7 && row == 5 && (col == select.1 - 1 || col == select.1 + 1) && board[row][col].is_empty(){
            *enpassant = true;
        }
        if select.1 == 0 && row == 5 && col == 1 && board[row][col].is_empty(){
            *enpassant = true;
        }
        if select.1 == 7 && row == 5 && col == 6 && board[row][col].is_empty(){
            *enpassant = true;
        }
    }
}
pub fn mutare_si_verificari(board: &mut [Vec<String>], select: (usize, usize), row: usize, col: usize, joc: &mut DetaliiJoc) {
    let mut p = String::new();
    p.push_str(&board[select.0][select.1]);
    board[select.0][select.1].clear();
    board[row][col].clear();
    board[row][col].push_str(&p);
    if joc.enpassant{
        if joc.turn == 0 {
            board[row + 1][col].clear();
        } else if joc.turn == 1 {
            board[row - 1][col].clear();
        }
        joc.enpassant = false;
    }
    if joc.rocada.rocada_mare {
        if joc.turn == 0 {
            board[7][0].clear();
            board[7][3].push_str("wR");
        } else if joc.turn == 1 {
            board[0][0].clear();
            board[0][3].push_str("bR");
        }
        joc.rocada.rocada_mare = false;
    }
    if joc.rocada.rocada_mica {
        if joc.turn == 0 {
            board[7][7].clear();
            board[7][5].push_str("wR");
    } else if joc.turn == 1 {
        board[0][7].clear();
        board[0][5].push_str("bR");
    }
        joc.rocada.rocada_mica = false;
    }
    for i in board[7].iter().take(8) {
        if i == "bP" {
            joc.transformari.1 = true;
        }
    }
    for i in board[0].iter().take(8) {
        if i == "wP" {
            joc.transformari.0 = true;
        }
    }
    if board[7][4].is_empty() {
        joc.rocada.white_big_castle = false;
        joc.rocada.white_small_castle = false;
    }
    if board[7][0] != "wR" {
        joc.rocada.white_big_castle = false;
    }
    if board[7][7] != "wR" {
        joc.rocada.white_small_castle = false;
    }
    if board[0][4].is_empty() {
        joc.rocada.black_big_castle = false;
        joc.rocada.black_small_castle = false;
    }
    if board[0][0] != "bR" {
        joc.rocada.black_big_castle = false;
    }
    if board[0][7] != "bR" {
        joc.rocada.black_small_castle = false;
    }
    joc.turn = 1 - joc.turn;
    if joc.index_mutare >= joc.lista_mutare.len() {
        joc.lista_mutare.push((String::new(), (0, 0), (0, 0)));
    }
    joc.lista_mutare[joc.index_mutare].0.push_str(&board[row][col]);
    joc.lista_mutare[joc.index_mutare].1 = (select.0, select.1);
    joc.lista_mutare[joc.index_mutare].2 = (row, col);    
    joc.index_mutare += 1;
}
pub fn mouse_button_released(board: &mut Vec<Vec<String>>, joc: &mut DetaliiJoc, timer: &mut FBox<Clock>, x2: f32, y2: f32, window: &mut FBox<RenderWindow>) {
    let width: f32 = window.size().x as f32 / 5.0;
    let height: f32 = window.size().y as f32 / 10.0;
    let remaining_width: f32 = window.size().x as f32 - width;
    let remaining_height: f32 = window.size().y as f32 - height;
    let square_size = remaining_width.min(remaining_height) / 9.0;  
    if joc.drag_info.0 == 1{
        if x2 >= width && x2 <= width + 8.0 * square_size && y2 >= height && y2 <= height + 8.0 * square_size {
            let col = ((x2 - width) / square_size) as usize;
            let row = ((y2 - height) / square_size) as usize;
            if joc.select != (9, 9) && joc.mutari_pos.contains(&(row, col)) {
                rocada_sau_enpassant(board, joc.select, row, col, &mut joc.enpassant, &mut joc.rocada);
                mutare_si_verificari(board, joc.select, row, col, joc);
                if joc.turn == 1 {
                   joc.timp.0 -= timer.elapsed_time().as_seconds();
                } 
                else {
                    joc.timp.1 -= timer.elapsed_time().as_seconds();
                }
                timer.restart();
                joc.mutari_pos.clear();
                if joc.lista_mutare.len() < 32 {
                   joc.max_scroll_offset = 0;
                } 
                else {
                    joc.max_scroll_offset = (joc.lista_mutare.len() - 31) / 2;
                }
                joc.scroll_offset = joc.max_scroll_offset;
                let pozitie_curenta = (board.to_owned(), joc.turn);
                verificare_pozitii_repetate(joc, &pozitie_curenta);
            }
        }
        joc.drag_info.0 = 0;
        joc.drag_info.3.clear();
        joc.drag_info.1 = (9, 9);
    }
}
pub fn verificare_pozitii_repetate(joc: &mut DetaliiJoc, pozitie_curenta: &(Vec<Vec<String>>, i32)) {
    let mut gasit = false;
    for p in &mut joc.pozitii {
        if p.0 == pozitie_curenta.0 && p.1 == pozitie_curenta.1 {
            gasit = true;
            p.2 += 1;
            if p.2 == 3 {
                joc.remiza_prin_repetare = true;
            }
            break;
        }
    }
    if !gasit {
        joc.pozitii.push((pozitie_curenta.0.clone(), joc.turn, 1));
    }
}
pub fn posibilitati_enpassant(board: &mut [Vec<String>], joc: &mut DetaliiJoc){
    if joc.index_mutare > 0 && board[joc.select.0][joc.select.1] == "wP" && joc.select.0 == 3 && joc.lista_mutare[joc.index_mutare - 1].0 == "bP"{
        enpassant_white(board, joc);
    }
    if joc.index_mutare > 0 && board[joc.select.0][joc.select.1] == "bP" && joc.select.0 == 4 && joc.lista_mutare[joc.index_mutare - 1].0 == "wP"{
        enpassant_black(board, joc);
    }
}
pub fn enpassant_white(board: &mut [Vec<String>], joc: &mut DetaliiJoc) {
    let king_pos = king_position(board, 0);
    if joc.select.1 > 0 && joc.select.1 < 7 {
        if joc.lista_mutare[joc.index_mutare - 1].1 == (1, joc.select.1 - 1) && joc.lista_mutare[joc.index_mutare - 1].2 == (3, joc.select.1 - 1) {
            board[joc.select.0][joc.select.1].clear();
            board[2][joc.select.1 - 1].push_str("wP");
            board[3][joc.select.1 - 1].clear();
            if !in_check(board, king_pos.0, king_pos.1, 0,){
                joc.mutari_pos.push((2, joc.select.1 - 1));
            }
            board[joc.select.0][joc.select.1].push_str("wP");
            board[3][joc.select.1 - 1].push_str("bP");
            board[2][joc.select.1 - 1].clear();
        } else if joc.lista_mutare[joc.index_mutare - 1].1 == (1, joc.select.1 + 1) && joc.lista_mutare[joc.index_mutare - 1].2 == (3, joc.select.1 + 1){
            board[joc.select.0][joc.select.1].clear();
            board[2][joc.select.1 + 1].push_str("wP");
            board[3][joc.select.1 + 1].clear();
            if !in_check(board, king_pos.0, king_pos.1, 0,){
                joc.mutari_pos.push((2, joc.select.1 + 1));
            }
            board[joc.select.0][joc.select.1].push_str("wP");
            board[3][joc.select.1 + 1].push_str("bP");
            board[2][joc.select.1 + 1].clear();
        }
    }
    if joc.select.1 == 0 && joc.lista_mutare[joc.index_mutare - 1].1 == (1, joc.select.1 + 1) && joc.lista_mutare[joc.index_mutare - 1].2 == (3, joc.select.1 + 1){
        board[joc.select.0][joc.select.1].clear();
        board[2][joc.select.1 + 1].push_str("wP");
        board[3][joc.select.1 + 1].clear();
        if !in_check(board, king_pos.0, king_pos.1, 0,){
            joc.mutari_pos.push((2, joc.select.1 + 1));
        }
        board[joc.select.0][joc.select.1].push_str("wP");
        board[3][joc.select.1 + 1].push_str("bP");
        board[2][joc.select.1 + 1].clear();
    }
    if joc.select.1 == 7 && joc.lista_mutare[joc.index_mutare - 1].1 == (1, joc.select.1 - 1) && joc.lista_mutare[joc.index_mutare - 1].2 == (3, joc.select.1 - 1){
        board[joc.select.0][joc.select.1].clear();
        board[2][joc.select.1 - 1].push_str("wP");
        board[3][joc.select.1 - 1].clear();
        if !in_check(board, king_pos.0, king_pos.1, 0,){
            joc.mutari_pos.push((2, joc.select.1 - 1));
        }
        board[joc.select.0][joc.select.1].push_str("wP");
        board[3][joc.select.1 - 1].push_str("bP");
        board[2][joc.select.1 - 1].clear();
    }
}
pub fn enpassant_black(board: &mut [Vec<String>], joc: &mut DetaliiJoc) {
    let king_pos = king_position(board, 1);
    if joc.select.1 > 0 && joc.select.1 < 7 {
        if joc.lista_mutare[joc.index_mutare - 1].1 == (6, joc.select.1 - 1) && joc.lista_mutare[joc.index_mutare - 1].2 == (4, joc.select.1 - 1){
            board[joc.select.0][joc.select.1].clear();
            board[5][joc.select.1 - 1].push_str("bP");
            board[4][joc.select.1 - 1].clear();
            if !in_check(board, king_pos.0, king_pos.1, 1,){
                joc.mutari_pos.push((5, joc.select.1 - 1));
            }
            board[joc.select.0][joc.select.1].push_str("bP");
            board[4][joc.select.1 - 1].push_str("wP");
            board[5][joc.select.1 - 1].clear();
        }
        if joc.lista_mutare[joc.index_mutare - 1].1 == (6, joc.select.1 + 1) && joc.lista_mutare[joc.index_mutare - 1].2 == (4, joc.select.1 + 1){
            board[joc.select.0][joc.select.1].clear();
            board[5][joc.select.1 + 1].push_str("bP");
            board[4][joc.select.1 + 1].clear();
            if !in_check(board, king_pos.0, king_pos.1, 1,){
                joc.mutari_pos.push((5, joc.select.1 + 1));
            }
            board[joc.select.0][joc.select.1].push_str("bP");
            board[4][joc.select.1 + 1].push_str("wP");
            board[5][joc.select.1 + 1].clear();
        }
    }
    if joc.select.1 == 0 && joc.lista_mutare[joc.index_mutare - 1].1 == (6, joc.select.1 + 1) && joc.lista_mutare[joc.index_mutare - 1].2 == (4, joc.select.1 + 1){
        board[joc.select.0][joc.select.1].clear();
        board[5][joc.select.1 + 1].push_str("bP");
        board[4][joc.select.1 + 1].clear();
        if !in_check(board, king_pos.0, king_pos.1, 1,){
            joc.mutari_pos.push((5, joc.select.1 + 1));
        }
        board[joc.select.0][joc.select.1].push_str("bP");
        board[4][joc.select.1 + 1].push_str("wP");
        board[5][joc.select.1 + 1].clear();
    }
    if joc.select.1 == 7 && joc.lista_mutare[joc.index_mutare - 1].1 == (6, joc.select.1 - 1) && joc.lista_mutare[joc.index_mutare - 1].2 == (4, joc.select.1 - 1){
        board[joc.select.0][joc.select.1].clear();
        board[5][joc.select.1 - 1].push_str("bP");
        board[4][joc.select.1 - 1].clear();
        if !in_check(board, king_pos.0, king_pos.1, 1,){
            joc.mutari_pos.push((5, joc.select.1 - 1));
        }
        board[joc.select.0][joc.select.1].push_str("bP");
        board[4][joc.select.1 - 1].push_str("wP");
        board[5][joc.select.1 - 1].clear();
    }
}
pub fn restart_game(board: &mut [Vec<String>], joc: &mut DetaliiJoc, timer: &mut Clock) {
    for row in board.iter_mut(){
        for col in row.iter_mut(){
            col.clear();
        }
    }
    init_board(board);
    joc.select = (9, 9);
    joc.turn = 0;
    joc.mutari_pos.clear();
    joc.rocada.white_big_castle = true;
    joc.rocada.white_small_castle = true;
    joc.rocada.black_big_castle = true;
    joc.rocada.black_small_castle = true;
    joc.rocada.rocada_mare = false;
    joc.rocada.rocada_mica = false;
    joc.enpassant = false;
    joc.transformari.0 = false;
    joc.transformari.1 = false;
    joc.remiza_prin_repetare = false;
    joc.lista_mutare.clear();
    joc.index_mutare = 0;
    joc.pozitii.clear();
    joc.pozitii.push((board.to_owned(), 0, 1));
    joc.timp.0 = 300.0;
    joc.timp.1 = 300.0;
    timer.restart();
    joc.loop_cnt = false;
    joc.scroll_offset = 0;
    joc.max_scroll_offset = 0;
    joc.drag_info.0 = 0;
    joc.drag_info.1 = (9, 9);
    joc.drag_info.2 = (0.0, 0.0);
    joc.drag_info.3.clear();
    joc.mesaj.clear();
    joc.gamestate = 1;
}
pub fn events(event: Event, window: &mut FBox<RenderWindow>, joc: &mut DetaliiJoc, timer: &mut FBox<Clock>, board: &mut [Vec<String>]){    
    match event {
        Event::MouseMoved { x, y } => {
        if joc.drag_info.0 == 1{
            joc.drag_info.2 = (x as f32, y as f32);
        }
    }
    Event::MouseWheelScrolled { delta, .. } => {
        if delta > 0.0 {
            if joc.scroll_offset >= delta as usize {
                joc.scroll_offset -= delta as usize;
            } else {
                joc.scroll_offset = 0;
            }
        }
        if delta < 0.0 {
            let delta_pozitiv = (-delta) as usize;
            if delta_pozitiv + joc.scroll_offset >= joc.max_scroll_offset {
                joc.scroll_offset = joc.max_scroll_offset;
            } else {
                joc.scroll_offset += delta_pozitiv;
            }
        }
    }
    Event::MouseButtonReleased { x, y, .. } => {
        let mut board_vec: Vec<Vec<String>> = board.to_vec();
        mouse_button_released(&mut board_vec, joc, timer, x as f32, y as f32, window);
        for (i, row) in board_vec.into_iter().enumerate() {
            board[i] = row;
        }                   
    }
    Event::MouseButtonPressed { button, x, y } => {
        if button == Button::Left {
            mouse_button_pressed(board, joc, x as f32, y as f32, window, timer);            
            }
        }
        _ => {}
    }
}
pub fn mouse_button_pressed(board: &mut [Vec<String>], joc: &mut DetaliiJoc, x2: f32, y2: f32, window: &mut FBox<RenderWindow>, timer: &mut FBox<Clock>){
    let width: f32 = window.size().x as f32 / 5.0;
    let height: f32 = window.size().y as f32 / 10.0;
    let remaining_width: f32 = window.size().x as f32 - width;
    let remaining_height: f32 = window.size().y as f32 - height;
    let square_size = remaining_width.min(remaining_height) / 9.0;
    if (joc.gamestate == 2 || joc.gamestate == 3 || joc.gamestate == 4) && (x2 >= 0.0 && x2 <= 2.0 * square_size && y2 >= 0.0 && y2 <= square_size){
        restart_game(board, joc, timer);
    }
    if joc.gamestate == 1 {
        if x2 >= square_size * 0.89 && x2 <= square_size * 2.89 && y2 >= 4.5 * square_size&& y2 <= 5.5 * square_size{
            restart_game(board, joc, timer);
        }
        board_click(board, joc, x2, y2, timer, window);
        if joc.transformari.0 && x2 >= width + 2.0 * square_size && x2 <= width + 6.0 * square_size && y2 >= 0.0 && y2 <= square_size{
            let n = ((x2 - (width + 2.0 * square_size)) / square_size) as usize;
            let mut pos: usize = 0;
            for i in board[0].iter().enumerate().take(8) {
                if i.1 == "wP" {
                    pos = i.0;
                }
            }
            board[0][pos].clear();
            if n == 0 {
                board[0][pos].push_str("wQ");
            } else if n == 1 {
                board[0][pos].push_str("wR");
            } else if n == 2 {
                board[0][pos].push_str("wB");
            } else if n == 3 {
                board[0][pos].push_str("wN");
            }
            joc.transformari.0 = false;
        } else if joc.transformari.1 && x2 >= width + 2.0 * square_size && x2 <= width + 6.0 * square_size && y2 >= window.size().y as f32 - square_size && y2 <= window.size().y as f32{
            let n = ((x2 - (width + 2.0 * square_size)) / square_size) as usize;
            let mut pos: usize = 0;
            for i in board[7].iter().enumerate().take(8) {
                if i.1 == "bP" {
                    pos = i.0;
                }
            }
            board[7][pos].clear();
            if n == 0 {
                board[7][pos].push_str("bQ");
            } else if n == 1 {
                board[7][pos].push_str("bR");
            } else if n == 2 {
                board[7][pos].push_str("bB");
            } else if n == 3 {
                board[7][pos].push_str("bN");
            }
            joc.transformari.1 = false;
        }
    }
}
pub fn board_click(board: &mut [Vec<String>], joc: &mut DetaliiJoc, x2: f32, y2: f32, timer: &mut FBox<Clock>, window: &mut FBox<RenderWindow>){
    let width: f32 = window.size().x as f32 / 5.0;
    let height: f32 = window.size().y as f32 / 10.0;
    let remaining_width: f32 = window.size().x as f32 - width;
    let remaining_height: f32 = window.size().y as f32 - height;
    let square_size = remaining_width.min(remaining_height) / 9.0;
    if !joc.transformari.1 && !joc.transformari.0 {
        if x2 >= width && x2 <= width + 8.0 * square_size && y2 >= height && y2 <= height + 8.0 * square_size{
            let col = ((x2 - width) / square_size) as usize;
            let row = ((y2 - height) / square_size) as usize;
            let piesa = &board[row][col];
            let culoare = if piesa.starts_with('w') { 0 } else { 1 };
            if joc.select != (9, 9) && joc.mutari_pos.contains(&(row, col)){
                efectuare_mutare(board, x2, y2, timer, joc, window);
            }
            else if !board[row][col].is_empty() && culoare == joc.turn {
                selectare_piesa(board, x2, y2, joc, window);                
            } else {
                joc.select = (9, 9);
                joc.mutari_pos.clear();
            }
        } else {
            joc.select = (9, 9);
            joc.mutari_pos.clear();
        }
    }
}
pub fn efectuare_mutare(board: &mut [Vec<String>], x2: f32, y2: f32, timer: &mut FBox<Clock>, joc: &mut DetaliiJoc, window: &mut FBox<RenderWindow>) {
    let width: f32 = window.size().x as f32 / 5.0;
    let height: f32 = window.size().y as f32 / 10.0;
    let remaining_width: f32 = window.size().x as f32 - width;
    let remaining_height: f32 = window.size().y as f32 - height;
    let square_size = remaining_width.min(remaining_height) / 9.0;
    let col = ((x2 - width) / square_size) as usize;
    let row = ((y2 - height) / square_size) as usize;
    if joc.select != (9, 9) && joc.mutari_pos.contains(&(row, col)) {
        rocada_sau_enpassant(board, joc.select, row, col, &mut joc.enpassant, &mut joc.rocada);
        mutare_si_verificari(board, joc.select, row, col, joc);
        if joc.turn == 1 {
            joc.timp.0 -= timer.elapsed_time().as_seconds();
        } else {
            joc.timp.1 -= timer.elapsed_time().as_seconds();
        }
        timer.restart();
        joc.select = (9, 9);
        joc.mutari_pos.clear();
        if joc.lista_mutare.len() < 32 {
           joc.max_scroll_offset = 0;
        } else {
            joc.max_scroll_offset = (joc.lista_mutare.len() - 31) / 2;
        }
        joc.scroll_offset = joc.max_scroll_offset;
        let pozitie_curenta = (board.to_owned(), joc.turn);
        let mut gasit = 0;
        for p in &mut joc.pozitii {
            if p.0 == pozitie_curenta.0 && p.1 == pozitie_curenta.1
            {
                gasit = 1;
                p.2 += 1;
                if p.2 == 3 {
                    joc.remiza_prin_repetare = true;
                }
                break;
            }
        }
        if gasit == 0 {
            joc.pozitii.push((board.to_owned(), joc.turn, 1));
        }
    }
}
fn selectare_piesa(board: &mut [Vec<String>], x2: f32, y2: f32, joc: &mut DetaliiJoc, window: &mut FBox<RenderWindow>) {
    let width: f32 = window.size().x as f32 / 5.0;
    let height: f32 = window.size().y as f32 / 10.0;    
    let remaining_width: f32 = window.size().x as f32 - width;
    let remaining_height: f32 = window.size().y as f32 - height;
    let square_size = remaining_width.min(remaining_height) / 9.0;
    let col = ((x2 - width) / square_size) as usize;
    let row = ((y2 - height) / square_size) as usize;
    joc.select = (row, col);
    joc.drag_info.1 = (row, col);
    joc.drag_info.0 = 1;
    joc.drag_info.2 = (x2, y2);
    let mut p = String::new();
    p.push_str(&board[row][col]);
    joc.drag_info.3 = p.clone();
    joc.mutari_pos = mutari_posibile(board, row, col);
    if board[row][col] == "wK" {
        if joc.rocada.white_small_castle && small_castle_possible(board, 0) == 1{
            joc.mutari_pos.push((7, 6));
        }
        if joc.rocada.white_big_castle && big_castle_possible(board, 0) == 1{
            joc.mutari_pos.push((7, 2));
        }
    }
    if board[row][col] == "bK" {
        if joc.rocada.black_small_castle && small_castle_possible(board, 1) == 1{
            joc.mutari_pos.push((0, 6));
        }
        if joc.rocada.black_big_castle && big_castle_possible(board, 1) == 1{
            joc.mutari_pos.push((0, 2));
        }
    }
    posibilitati_enpassant(board, joc);
}
pub fn verifica_final(board: &mut [Vec<String>], joc: &mut DetaliiJoc){
    if joc.gamestate == 1 {
        if checkmate(board, joc.turn){
            joc.mesaj.push_str("Checkmate!");
            sleep(Time::seconds(2.0));
            if joc.turn == 1 {
                joc.gamestate = 2;
            } 
            else if joc.turn == 0 {
                joc.gamestate = 3;
            }
        }
        if stalemate(board, joc.turn){
            joc.mesaj.push_str("Stalemate!");
            sleep(Time::seconds(2.0));
            joc.gamestate = 4;
        }
        if material_insuficient(board){
            joc.mesaj.push_str("Insufficient material!");
            sleep(Time::seconds(2.0));
            joc.gamestate = 4;
        }
        if joc.remiza_prin_repetare {
            joc.mesaj.push_str("Repeated positions!");
            sleep(Time::seconds(2.0));
            joc.gamestate = 4;
        }
    }
}