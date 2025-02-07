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

struct TranspositionTable {
    cache: HashMap<u64, (i32, u32)>, // hash, (score, depth)
    max_entries: usize,
}

impl TranspositionTable {
    fn new(max_entries: usize) -> Self {
        TranspositionTable {
            cache: HashMap::new(),
            max_entries,
        }
    }

    fn get(&self, hash: u64, depth: u32) -> Option<i32> {
        self.cache.get(&hash)
            .filter(|&(_, cached_depth)| *cached_depth >= depth)
            .map(|(score, _)| *score)
    }

    fn insert(&mut self, hash: u64, score: i32, depth: u32) {
        if self.cache.len() >= self.max_entries {
            // Use a different approach to remove an entry
            let key_to_remove = self.cache.keys().cloned().next();
            if let Some(key) = key_to_remove {
                self.cache.remove(&key);
            }
        }
        self.cache.insert(hash, (score, depth));
    }
}

#[derive(Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    // pub white_castle_possible: bool,
    // pub black_castle_possible: bool,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [[EMPTY; 8]; 8],
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
        temp_board.squares[row][col] = None;
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
            if self.is_square_under_attack(to.0, to.1, color) { // Check if the square IS under attack
                // println!("{to:?} is under attack, cannot escape");
                continue; // Important: Continue to the next move
            } else {
                // println!("{to:?} escapes");
                return false; // Square is safe, king can escape - NOT checkmate
            }
        }
       
        // 2. Block/capture:
        let all_moves = self.generate_all_moves(color); // Get all possible moves for the current player

        for (from, to) in all_moves {
            let mut temp_board = self.clone();
            temp_board.apply_move((from,to));
            if !temp_board.is_in_check(color) {
                // println!("{from:?} {to:?} escapes");
                return false; // Block or capture successful
            }
        }
        true // No escape, no block, no capture = Checkmate
    }

    pub fn is_draw(&self, color: Color) -> bool {
        self.is_stalemate(color) || !self.has_sufficient_material()
    }

    fn is_stalemate(&self, color: Color) -> bool {
        if self.is_in_check(color) {
            return false;
        }
        let moves = self.generate_all_moves(color);
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
        if !legal_moves.contains(&(from, to)) {
            return false;
        }

        // Simulate the move to check if it leaves the king in check
        let mut simulated_board = self.clone();
        simulated_board.apply_move((from, to));

        if simulated_board.is_in_check(piece.color) {
            return false; // Move is invalid if it leaves the king in check
        }

        true
    }
    
}

fn evaluate_board(board: &Board) -> i32 {
    let mut score = 0;
    // Define material values
    let values = |piece: &Piece| -> i32 {
        match piece.kind {
            PieceType::Pawn => 10,
            PieceType::Knight => 30,
            PieceType::Bishop => 30,
            PieceType::Rook => 50,
            PieceType::Queen => 90,
            PieceType::King => 9000,
        }
    };
    

    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board.squares[row][col] {
                let val = values(&piece);
                score += if piece.color == Color::White {
                    val
                } else {
                    -val
                };
            }
        }
    }

    score
}

fn order_moves(board: &Board, moves: &[((usize, usize), (usize, usize))], color: Color) -> Vec<((usize, usize), (usize, usize))> {
    let mut scored_moves: Vec<_> = moves.iter()
        .map(|&m| {
            let mut new_board = board.clone();
            new_board.apply_move(m);
            
            // Score moves based on:
            // 1. Captures (prioritize capturing high-value pieces)
            // 2. Checks
            // 3. Piece development
            let mut score = 0;
            
            // Capture score
            if let Some(captured_piece) = new_board.squares[m.1.0][m.1.1] {
                score += match captured_piece.kind {
                    PieceType::King => 9000,
                    PieceType::Queen => 90,
                    PieceType::Rook => 50,
                    PieceType::Bishop | PieceType::Knight => 30,
                    PieceType::Pawn => 10,
                };
            }
            
            // Check bonus
            if new_board.is_in_check(opposite_color(color)) {
                score += 10;
            }
            
            (m, score)
        })
        .collect();
    
    // Sort in descending order of score
    scored_moves.sort_by_key(|&(_, score)| Reverse(score));
    
    // Return just the moves
    scored_moves.into_iter().map(|(m, _)| m).collect()
}

fn quiescence_search(board: &Board, color: Color, alpha: i32, beta: i32, depth_limit: u32) -> i32 {
    if depth_limit == 0 { // Base case: Reached depth limit
        return evaluate_board(board); // Evaluate statically
    }

    let eval = evaluate_board(board);
    if eval >= beta {
        return beta;
    }
    let mut alpha = alpha.max(eval);

    let all_moves = board.generate_all_moves(color);

    let captures: Vec<_> = all_moves.iter().filter(|&m| is_capture(board, m)).cloned().collect();
    let ordered_captures = order_moves(board, &captures, color);

    for capture in ordered_captures.clone() {
        let ((_, _), (to_row, to_col)) = capture;
           if let Some(captured_piece) = board.squares[to_row][to_col] {
               let captured_value = match captured_piece.kind {  // Define piece values
                    PieceType::Pawn => 10,
                    PieceType::Knight => 30,
                    PieceType::Bishop => 30,
                    PieceType::Rook => 50,
                    PieceType::Queen => 90,
                    PieceType::King => 9000,
               };

               // Delta Pruning:  If the capture is not likely to improve the score, skip it.
               if eval + captured_value < alpha - 100 { // 100 is a margin, adjust as needed
                   continue; // Skip this capture
               }
           }
        let mut new_board = board.clone();
        new_board.apply_move(capture);
        let eval = -quiescence_search(&new_board, opposite_color(color), -beta, -alpha, depth_limit - 1); // Decrement depth
        if eval >= beta {
            return beta;
        }
        alpha = alpha.max(eval);
    }

    // Now, consider quiet moves (if no captures were good enough):
    if ordered_captures.is_empty() { // If no captures were good enough
        let quiet_moves: Vec<_> = all_moves.iter().filter(|&m| !is_capture(board, m)).cloned().collect();
        for quiet_move in quiet_moves {
            let mut new_board = board.clone();
            new_board.apply_move(quiet_move);
            let eval = -quiescence_search(&new_board, opposite_color(color), -beta, -alpha, depth_limit - 1); // Decrement depth
            if eval >= beta {
                return beta;
            }
            alpha = alpha.max(eval);
        }
    }

    alpha
}

// Helper function to check if a move is a capture
fn is_capture(board: &Board, m: &((usize, usize), (usize, usize))) -> bool {
    let ((_, _), (to_row, to_col)) = *m;
    board.squares[to_row][to_col].is_some() // Check if there's a piece at the target square
}

fn minimax(
    board: &Board,
    depth: u32,
    is_maximizing_player: bool,
    color: Color,
    mut alpha: i32,
    mut beta: i32,
    transposition_table: &mut TranspositionTable, // Pass the transposition table
) -> i32 {
    if depth == 0 || board.is_checkmate(Color::White) || board.is_checkmate(Color::Black) {
        return quiescence_search(board, color, alpha, beta, 3);
    }

    let moves = board.generate_all_moves(color);
    let ordered_moves = order_moves(board, &moves, color);
    let board_hash = calculate_board_hash(board); // Hash current board

    if let Some(cached_eval) = transposition_table.get(board_hash, depth) {
        return cached_eval;
    }

    if is_maximizing_player {
        let mut max_eval = i32::MIN;
        for m in ordered_moves {
            let mut new_board = board.clone();
            new_board.apply_move(m);
            let eval = minimax(&new_board, depth - 1, false, opposite_color(color), alpha, beta, transposition_table);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if alpha >= beta {
                break; // Pruning
            }
        }
        transposition_table.insert(board_hash, max_eval, depth); // Store the result
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for m in ordered_moves {
            let mut new_board = board.clone();
            new_board.apply_move(m);
            let eval = minimax(&new_board, depth - 1, true, opposite_color(color), alpha, beta, transposition_table);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if alpha >= beta {
                break; // Pruning
            }
        }
        transposition_table.insert(board_hash, min_eval, depth); // Store the result
        min_eval
    }
}


pub fn best_move_for_color(
    board: &Board,
    color: Color,
    depth: u32,
) -> Option<((usize, usize), (usize, usize))> {
    let mut moves = board.generate_all_moves(color);
    moves = order_moves(board, &moves, color); // Order moves for better pruning

    let mut best_move = None;
    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;
    let mut best_eval = if color == Color::White { i32::MIN } else { i32::MAX };
    let mut transposition_table = TranspositionTable::new(10000); // Create transposition table

    for m in moves {
        let mut new_board = board.clone();
        new_board.apply_move(m);

        let eval = minimax(
            &new_board,
            depth - 1,
            color == Color::Black,
            opposite_color(color),
            alpha,
            beta,
            &mut transposition_table, // Pass the transposition table here
        );

        if color == Color::White {
            if eval > best_eval {
                best_eval = eval;
                best_move = Some(m);
            }
            alpha = alpha.max(best_eval);
        } else {
            if eval < best_eval {
                best_eval = eval;
                best_move = Some(m);
            }
            beta = beta.min(best_eval);
        }

        if alpha >= beta {
            break;
        }
    }
    best_move
}

// Simple board hash function (you'd want a more sophisticated one in practice)
fn calculate_board_hash(board: &Board) -> u64 {
    let mut hash = 0;
    for row in board.squares.iter() {
        for square in row.iter() {
            hash ^= match square {
                Some(piece) => piece.color as u64 | ((piece.kind as u64) << 8),
                None => 0,
            };
            hash = hash.rotate_left(7);
        }
    }
    hash
}

pub fn opposite_color(color: Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}

pub fn improved_best_move_for_color(
    board: &Board,
    color: Color,
    depth: u32,
) -> Option<((usize, usize), (usize, usize))> {
    // Piece values for basic evaluation
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

    // Fast static evaluation
    fn evaluate_position(board: &Board) -> i32 {
        let mut score = 0;
        
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = board.squares[row][col] {
                    let mut piece_score = get_piece_value(&piece);
                    
                    // Simple position bonus for center control
                    if (2..=5).contains(&row) && (2..=5).contains(&col) {
                        piece_score += 10;
                    }
                    
                    score += if piece.color == Color::White {
                        piece_score
                    } else {
                        -piece_score
                    };
                }
            }
        }
        score
    }

    // Simple move scoring for ordering
    fn score_move(board: &Board, m: &((usize, usize), (usize, usize))) -> i32 {
        let ((from_row, from_col), (to_row, to_col)) = *m;
        let mut score = 0;

        // Prioritize captures
        if let Some(captured_piece) = board.squares[to_row][to_col] {
            score += get_piece_value(&captured_piece);
        }

        // Bonus for center control
        if (2..=5).contains(&to_row) && (2..=5).contains(&to_col) {
            score += 10;
        }

        score
    }

    // Alpha-beta search
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
        
        // Basic move ordering - sort by captures
        moves.sort_by_key(|m| -score_move(board, m));

        if maximizing_player {
            let mut max_eval = i32::MIN;
            for m in moves {
                let mut new_board = board.clone();
                new_board.apply_move(m);
                
                // Skip if move leaves king in check
                if new_board.is_in_check(color) {
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
                
                // Skip if move leaves king in check
                if new_board.is_in_check(color) {
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

    // Main search logic
    let mut best_move = None;
    let mut best_value = if color == Color::White { i32::MIN } else { i32::MAX };
    let mut moves = board.generate_all_moves(color);
    
    // Sort moves to improve pruning
    moves.sort_by_key(|m| -score_move(board, m));

    for m in moves {
        let mut new_board = board.clone();
        new_board.apply_move(m);
        
        // Skip if move leaves king in check
        if new_board.is_in_check(color) {
            continue;
        }
        
        let value = alpha_beta(
            &new_board,
            depth - 1,
            i32::MIN + 1,
            i32::MAX - 1,
            color == Color::Black,
            opposite_color(color),
        );

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