// engine.rs
use std::collections::HashMap;
use std::cmp::Reverse;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceType,
}

pub const EMPTY: Option<Piece> = None;


#[derive(Clone, Debug)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub half_move_clock: u32, // Tracks moves since last pawn move or capture
    // pub white_castle_possible: bool,
    // pub black_castle_possible: bool,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [[EMPTY; 8]; 8],
            half_move_clock: 0,
            // white_castle_possible: true,
            // black_castle_possible: true,
        };

        // Initialize board with pieces (only a few for brevity)
        for col in 0..8 {
            board.squares[1][col] = Some(Piece {
                color: Color::White,
                kind: PieceType::Pawn,
            });
            board.squares[6][col] = Some(Piece {
                color: Color::Black,
                kind: PieceType::Pawn,
            });
        }

        board.squares[0][4] = Some(Piece {
            color: Color::White,
            kind: PieceType::King,
        });
        board.squares[7][4] = Some(Piece {
            color: Color::Black,
            kind: PieceType::King,
        });

        board.squares[0][3] = Some(Piece {
            color: Color::White,
            kind: PieceType::Queen,
        });
        board.squares[7][3] = Some(Piece {
            color: Color::Black,
            kind: PieceType::Queen,
        });

        board.squares[0][0] = Some(Piece {
            color: Color::White,
            kind: PieceType::Rook,
        });
        board.squares[7][0] = Some(Piece {
            color: Color::Black,
            kind: PieceType::Rook,
        });

        board.squares[0][7] = Some(Piece {
            color: Color::White,
            kind: PieceType::Rook,
        });
        board.squares[7][7] = Some(Piece {
            color: Color::Black,
            kind: PieceType::Rook,
        });

        board.squares[0][1] = Some(Piece {
            color: Color::White,
            kind: PieceType::Knight,
        });
        board.squares[7][1] = Some(Piece {
            color: Color::Black,
            kind: PieceType::Knight,
        });

        board.squares[0][6] = Some(Piece {
            color: Color::White,
            kind: PieceType::Knight,
        });
        board.squares[7][6] = Some(Piece {
            color: Color::Black,
            kind: PieceType::Knight,
        });

        board.squares[0][2] = Some(Piece {
            color: Color::White,
            kind: PieceType::Bishop,
        });
        board.squares[7][2] = Some(Piece {
            color: Color::Black,
            kind: PieceType::Bishop,
        });

        board.squares[0][5] = Some(Piece {
            color: Color::White,
            kind: PieceType::Bishop,
        });
        board.squares[7][5] = Some(Piece {
            color: Color::Black,
            kind: PieceType::Bishop,
        });

        board
    }

    // Insert your custom move generation, evaluation, minimax, etc. here.

    // Returns a vector of moves in the form: ((from_row, from_col), (to_row, to_col))
    pub fn generate_moves_for_piece(
        &self,
        row: usize,
        col: usize,
    ) -> Vec<((usize, usize), (usize, usize))> {
        let mut moves = Vec::new();
        if let Some(piece) = self.squares[row][col] {
            match piece.kind {
                PieceType::Pawn => {
                    // Determine the moving direction based on the pawn's color
                    let direction: isize = match piece.color {
                        Color::White => 1,  // White moves up (toward row 7)
                        Color::Black => -1, // Black moves down (toward row 0)
                    };
                    let new_row = row as isize + direction;
                    
                    // Simple forward move (1 square ahead)
                    if new_row >= 0 && new_row < 8 && self.squares[new_row as usize][col].is_none() {
                        moves.push(((row, col), (new_row as usize, col)));
                    }

                    // Double forward move (only allowed on the starting row and if both squares are empty)
                    let starting_row = if piece.color == Color::White { 1 } else { 6 };
                    if row == starting_row && self.squares[new_row as usize][col].is_none() {
                        let double_row = new_row + direction;  // Calculate the row 2 squares ahead
                        if double_row >= 0 && double_row < 8 && self.squares[double_row as usize][col].is_none() {
                            // Check that the square two steps ahead is empty
                            moves.push(((row, col), (double_row as usize, col)));
                        }
                    }
                
                    // Diagonal captures (both left and right)
                    for &dc in &[-1, 1] {
                        let new_col = col as isize + dc;
                        if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                            if let Some(dest_piece) = self.squares[new_row as usize][new_col as usize] {
                                if dest_piece.color != piece.color {
                                    moves.push(((row, col), (new_row as usize, new_col as usize)));
                                }
                            }
                        }
                    }
                }
                PieceType::Knight => {
                    // L-shaped moves for knights
                    let knight_moves = [
                        (2, 1),
                        (1, 2),
                        (-1, 2),
                        (-2, 1),
                        (-2, -1),
                        (-1, -2),
                        (1, -2),
                        (2, -1),
                    ];
                    for (dr, dc) in knight_moves.iter() {
                        let new_row = row as isize + dr;
                        let new_col = col as isize + dc;
                        if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                            // Allow move if the destination is either empty or occupied by an enemy piece
                            if let Some(dest_piece) =
                                self.squares[new_row as usize][new_col as usize]
                            {
                                if dest_piece.color != piece.color {
                                    moves.push(((row, col), (new_row as usize, new_col as usize)));
                                }
                            } else {
                                moves.push(((row, col), (new_row as usize, new_col as usize)));
                            }
                        }
                    }
                }
                PieceType::King => {
                    let king_moves = [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ];
                    for (dr, dc) in king_moves.iter() {
                        let new_row = row as isize + dr;
                        let new_col = col as isize + dc;
                        if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                            if let Some(dest_piece) =
                                self.squares[new_row as usize][new_col as usize]
                            {
                                // Check if the destination is either empty or occupied by an enemy piece
                                if dest_piece.color != piece.color {
                                    // Check if the destination is under attack
                                    // if !self.is_square_under_attack(new_row as usize, new_col as usize, piece.color) {
                                        moves.push(((row, col), (new_row as usize, new_col as usize)));
                                    // }
                                }
                            } else {
                                // Check if the destination is under attack
                                // if !self.is_square_under_attack(new_row as usize, new_col as usize, piece.color) {
                                    moves.push(((row, col), (new_row as usize, new_col as usize)));
                                // }
                            }
                        }
                    }
                

                }                
                PieceType::Queen => {
                    // Queen moves are a combination of Rook and Bishop moves
                    moves.extend(self.generate_moves_in_direction(row, col, 1, 0, piece)); // Right
                    moves.extend(self.generate_moves_in_direction(row, col, -1, 0, piece)); // Left
                    moves.extend(self.generate_moves_in_direction(row, col, 0, 1, piece)); // Down
                    moves.extend(self.generate_moves_in_direction(row, col, 0, -1, piece)); // Up
                    moves.extend(self.generate_moves_in_direction(row, col, 1, 1, piece)); // Down-Right
                    moves.extend(self.generate_moves_in_direction(row, col, 1, -1, piece)); // Up-Right
                    moves.extend(self.generate_moves_in_direction(row, col, -1, 1, piece)); // Down-Left
                    moves.extend(self.generate_moves_in_direction(row, col, -1, -1, piece));
                    // Up-Left
                }
                PieceType::Rook => {
                    moves.extend(self.generate_moves_in_direction(row, col, 1, 0, piece)); // Right
                    moves.extend(self.generate_moves_in_direction(row, col, -1, 0, piece)); // Left
                    moves.extend(self.generate_moves_in_direction(row, col, 0, 1, piece)); // Down
                    moves.extend(self.generate_moves_in_direction(row, col, 0, -1, piece));
                    // Up
                }
                PieceType::Bishop => {
                    moves.extend(self.generate_moves_in_direction(row, col, 1, 1, piece)); // Down-Right
                    moves.extend(self.generate_moves_in_direction(row, col, 1, -1, piece)); // Up-Right
                    moves.extend(self.generate_moves_in_direction(row, col, -1, 1, piece)); // Down-Left
                    moves.extend(self.generate_moves_in_direction(row, col, -1, -1, piece));
                    // Up-Left
                }

            }
        }
        moves
    }

    // Generate moves for the current player (assume you pass which color is moving)
    fn generate_all_moves(&self, color: Color) -> Vec<((usize, usize), (usize, usize))> {
        let mut all_moves = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.squares[row][col] {
                    if piece.color == color {
                        all_moves.extend(self.generate_moves_for_piece(row, col));
                    }
                }
            }
        }
        all_moves
    }

    fn generate_moves_in_direction(
        &self,
        row: usize,
        col: usize,
        dr: isize,
        dc: isize,
        piece: Piece,
    ) -> Vec<((usize, usize), (usize, usize))> {
        let mut moves = Vec::new();
        let mut new_row = row as isize + dr;
        let mut new_col = col as isize + dc;

        while new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
            let dest_piece = self.squares[new_row as usize][new_col as usize];
            if let Some(dest_piece) = dest_piece {
                if dest_piece.color != piece.color {
                    moves.push(((row, col), (new_row as usize, new_col as usize)));
                }
                break; // Stop after capturing a piece
            } else {
                moves.push(((row, col), (new_row as usize, new_col as usize)));
            }
            new_row += dr;
            new_col += dc;
        }
        moves
    }

    pub fn apply_move(&mut self, m: ((usize, usize), (usize, usize))) {
        let ((from_row, from_col), (to_row, to_col)) = m;
        // if self.can_castle((from_row, from_col), (to_row, to_col)) {
        //     self.castle((from_row, from_col), (to_row, to_col));
        //     return;
        // }
        if let Some(mut piece) = self.squares[from_row][from_col] {
            self.squares[from_row][from_col] = EMPTY;

            // Update half-move clock on captures or pawn moves
            if piece.kind == PieceType::Pawn || self.squares[to_row][to_col].is_some() {
                self.half_move_clock = 0; // Reset clock on pawn move or capture
            } else {
                self.half_move_clock += 1;
            }
            if piece.kind == PieceType::Pawn && (to_row == 0 || to_row == 7) {
                // Promote to a Queen (can be extended for other choices)
                piece.kind = PieceType::Queen;
            }
            self.squares[to_row][to_col] = Some(piece);
        }
    }    

    pub fn is_square_under_attack(&self, row: usize, col: usize, color: Color) -> bool {
        let opponent_color = opposite_color(color);
        let mut temp_board = self.clone();
        // temp_board.squares[row][col] = None;
        // Check all opponent's pieces
        for r in 0..8 {
            for c in 0..8 {
                if let Some(piece) = temp_board.squares[r][c] {
                    // If the piece is of the opposite color, generate its moves
                    if piece.color == opponent_color {
                        let possible_moves = temp_board.generate_moves_for_piece(r, c);
                        // println!("{:?} {:?}", piece, possible_moves);
                        // If any move attacks the square
                        if possible_moves.iter().any(|&(_, to)| to == (row, col)) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    // Check if the given move (from -> to) is a valid castling move
    // pub fn can_castle(&self, from: (usize, usize), to: (usize, usize)) -> bool {
    //     let (from_row, from_col) = from;
    //     let (to_row, to_col) = to;

    //     // Ensure it's a king moving two squares
    //     if (from_col == 4) && (to_col == 6 || to_col == 2) && (from_row == to_row) {
    //         let color = if from_row == 0 { Color::White } else { Color::Black };
    //         let kingside = to_col == 6;
    //         let rook_col = if kingside { 7 } else { 0 };

    //         // Castling flag check
    //         if color == Color::White && !self.white_castle_possible { return false; }
    //         if color == Color::Black && !self.black_castle_possible { return false; }

    //         // Ensure King and Rook are in their original positions
    //         if let Some(Piece { kind: PieceType::King, color: king_color }) = self.squares[from_row][from_col] {
    //             if king_color != color {
    //                 return false;
    //             }
    //         } else {
    //             return false; // King must be present
    //         }

    //         if let Some(Piece { kind: PieceType::Rook, color: rook_color }) = self.squares[from_row][rook_col] {
    //             if rook_color != color {
    //                 return false;
    //             }
    //         } else {
    //             return false; // Rook must be present
    //         }

    //         // Ensure the squares between the King and Rook are empty
    //         let range = if kingside { 5..7 } else { 1..4 };
    //         if range.clone().any(|c| self.squares[from_row][c].is_some()) {
    //             return false; // Path must be clear
    //         }

    //         // Ensure the King is not in check, moving through check, or landing in check
    //         if self.is_in_check(color) { return false; }
    //         for col in range {
    //             if self.is_square_under_attack(from_row, col, color) { return false; }
    //         }

    //         // Passed all checks, castling is valid
    //         return true;
    //     }

    //     false // Not a valid castling move
    // }

    // // Execute the castling move if valid
    // pub fn castle(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
    //     if !self.can_castle(from, to) {
    //         return false;
    //     }

    //     let (row, from_col) = from;
    //     let to_col = to.1;
    //     let kingside = to_col == 6;
    //     let rook_col = if kingside { 7 } else { 0 };
    //     let new_rook_col = if kingside { 5 } else { 3 };

    //     // Move the King
    //     self.squares[row][to_col] = self.squares[row][from_col].take();
        
    //     // Move the Rook
    //     self.squares[row][new_rook_col] = self.squares[row][rook_col].take();

    //     // Disable further castling for this player
    //     if row == 0 {
    //         self.white_castle_possible = false;
    //     } else {
    //         self.black_castle_possible = false;
    //     }

    //     true
    // }

    pub fn find_king(&self, color: Color) -> Option<(usize, usize)> {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.squares[row][col] {
                    if piece.kind == PieceType::King && piece.color == color {
                        return Some((row, col));
                    }
                }
            }
        }
        None
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        if !self.is_in_check(color) {
            return false; // Not in check, can't be checkmate
        }
    
        let king_pos = self.find_king(color).unwrap();
        let king_moves = self.generate_moves_for_piece(king_pos.0, king_pos.1);
    
        // 1. King escape:
        for (_, to) in king_moves {
            let mut temp_board = self.clone();
            temp_board.apply_move((king_pos, to)); // Simulate the king's move
            if !temp_board.is_in_check(color) {
                return false; // King can escape
            }
        }
    
        // 2. Block/capture:
        // let checking_pieces = self.pieces_causing_check(color); // Helper function (see previous response)
        let moves: Vec<_> = self.generate_all_moves(color).into_iter().filter(|m| self.is_valid_move(m.0,m.1)).collect();
        moves.is_empty()

    }

    pub fn is_draw(&self, color: Color) -> bool {
        self.is_stalemate(color) || !self.has_sufficient_material() || self.half_move_clock >= 50
    }

    fn is_stalemate(&self, color: Color) -> bool {
        if self.is_in_check(color) {
            return false;
        }
        let moves: Vec<_> = self.generate_all_moves(color).into_iter().filter(|m| self.is_valid_move(m.0,m.1)).collect();
        moves.is_empty()
    }

    fn has_sufficient_material(&self) -> bool {
        let mut white_major_material = 0;
        let mut black_major_material = 0;
        let mut white_minor_material = 0;
        let mut black_minor_material = 0;
    
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = &self.squares[row][col] {
                    match piece.kind {
                        PieceType::Pawn | PieceType::Rook | PieceType::Queen => {
                            if piece.color == Color::White {
                                white_major_material += 1;
                            } else {
                                black_major_material += 1;
                            }
                        }
                        PieceType::Knight | PieceType::Bishop => {
                            if piece.color == Color::White {
                                white_minor_material += 1;
                            } else {
                                black_minor_material += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    
        // If either side has a Pawn, Rook, or Queen, checkmate is possible
        if white_major_material > 0 || black_major_material > 0 {
            return true;
        }
    
        // Special case: If both sides have only a King, it's a draw
        if white_minor_material == 0 && black_minor_material == 0 {
            return false;
        }
    
        // Special case: A single knight or bishop cannot force checkmate alone
        if (white_minor_material == 1 && black_major_material == 0 && black_minor_material == 0)
            || (black_minor_material == 1 && white_major_material == 0 && white_minor_material == 0)
        {
            return false;
        }
    
        // If both sides have minor pieces but no major pieces, it's a draw unless there are at least two bishops
        if white_major_material == 0 && black_major_material == 0 {
            if white_minor_material <= 1 && black_minor_material <= 1 {
                return false;
            }
        }
    
        // If no early draw conditions matched, checkmate is still possible
        true
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        // Find the king's position
        let king_position: Option<(usize, usize)> = self.find_king(color);

        // If king is not found (should never happen in a valid game), return false
        let (king_row, king_col) = match king_position {
            Some(pos) => pos,
            None => return false,
        };

        // Check if any opponent piece can attack the king's position
        self.is_square_under_attack(king_row, king_col, color)
    }

    pub fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Ensure move is within board bounds
        if from == to || from.0 >= 8 || from.1 >= 8 || to.0 >= 8 || to.1 >= 8 {
            return false;
        }

        // Check if there is a piece at the starting position
        let piece = match self.squares[from.0][from.1] {
            Some(p) => p,
            None => return false,
        };

        // // Check if it is castle move
        // if self.can_castle(from, to) {
        //     return true;
        // }

        // Ensure the piece is not capturing its own color
        if let Some(target_piece) = self.squares[to.0][to.1] {
            if target_piece.color == piece.color {
                return false;
            }
        }

        // Check if the move is in the piece’s legal moves
        let legal_moves = self.generate_moves_for_piece(from.0, from.1);
        println!("Legal moves: {:?}", legal_moves);
        if !legal_moves.contains(&(from, to)) {
            return false;
        }

        // Simulate the move to check if it leaves the king in check
        let mut simulated_board = self.clone();
        simulated_board.apply_move((from, to));
        if simulated_board.is_in_check(piece.color) {
            println!("{:?}", simulated_board);
            return false; // Move is invalid if it leaves the king in check
        }

        true
    }
    
}


pub fn opposite_color(color: Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}
use std::thread;

pub fn improved_best_move_for_color(
    board: &Board,
    color: Color,
    depth: u32,
) -> Option<((usize, usize), (usize, usize))> {
    fn get_piece_value(piece: &Piece) -> i32 {
        match piece.kind {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000,
        }
    }

    fn evaluate_position(board: &Board) -> i32 {
        let mut score = 0;
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = board.squares[row][col] {
                    let mut piece_score = get_piece_value(&piece);
                    if (2..=5).contains(&row) && (2..=5).contains(&col) {
                        piece_score += 10;
                    }
                    score += if piece.color == Color::White { piece_score } else { -piece_score };
                }
            }
        }
        score
    }

    fn score_move(board: &Board, m: &((usize, usize), (usize, usize))) -> i32 {
        let ((_, _), (to_row, to_col)) = *m;
        let mut score = 0;
        if let Some(captured_piece) = board.squares[to_row][to_col] {
            score += get_piece_value(&captured_piece);
        }
        if (2..=5).contains(&to_row) && (2..=5).contains(&to_col) {
            score += 10;
        }
        score
    }

    fn alpha_beta(
        board: &Board,
        depth: u32,
        mut alpha: i32,
        mut beta: i32,
        maximizing_player: bool,
        color: Color,
    ) -> i32 {
        if depth == 0 {
            return evaluate_position(board);
        }

        let mut moves = board.generate_all_moves(color);
        moves.sort_by_key(|m| -score_move(board, m));

        if maximizing_player {
            let mut max_eval = i32::MIN;
            for m in moves {
                let mut new_board = board.clone();
                new_board.apply_move(m);

                if let Some(king_pos) = new_board.find_king(color) {
                    if new_board.is_square_under_attack(king_pos.0, king_pos.1, color) {
                        continue; // Skip this move
                    }
                } else {
                    continue;
                }

                let eval = alpha_beta(
                    &new_board,
                    depth - 1,
                    alpha,
                    beta,
                    false,
                    opposite_color(color),
                );
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for m in moves {
                let mut new_board = board.clone();
                new_board.apply_move(m);

                if let Some(king_pos) = new_board.find_king(color) {
                    if new_board.is_square_under_attack(king_pos.0, king_pos.1, color) {
                        continue;
                    }
                } else {
                    continue;
                }

                let eval = alpha_beta(
                    &new_board,
                    depth - 1,
                    alpha,
                    beta,
                    true,
                    opposite_color(color),
                );
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }

    // Main search logic with threading
    let mut best_move = None;
    let mut best_value = if color == Color::White { i32::MIN } else { i32::MAX };
    let mut moves = board.generate_all_moves(color);

    // Sort moves to improve pruning
    moves.sort_by_key(|m| -score_move(board, m));

    let mut handles = vec![];

    // Launch threads only for legal moves
    for m in moves {
        let mut new_board = board.clone();
        new_board.apply_move(m);

        // Check if move leaves the king in check **before** spawning a thread
        if let Some(king_pos) = new_board.find_king(color) {
            if new_board.is_square_under_attack(king_pos.0, king_pos.1, color) {
                continue; // Skip this move
            }
        } else {
            continue; // Skip this move
        }

        let board_clone = new_board.clone();
        let color_clone = color;
        let depth_clone = depth;

        let handle = thread::spawn(move || {
            alpha_beta(
                &board_clone,
                depth_clone - 1,
                i32::MIN + 1,
                i32::MAX - 1,
                color_clone == Color::Black,
                opposite_color(color_clone),
            )
        });

        handles.push((handle, m));
    }

    // Collect results and determine the best move
    for (handle, m) in handles {
        let value = handle.join().unwrap();

        if color == Color::White {
            if value > best_value {
                best_value = value;
                best_move = Some(m);
            }
        } else {
            if value < best_value {
                best_value = value;
                best_move = Some(m);
            }
        }
    }

    best_move
}
