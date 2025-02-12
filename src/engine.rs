// engine.rs
// use std::cmp::Reverse;
// use std::collections::HashMap;
use rand::Rng;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    pub fn get_name(&self) -> &str {
        match *self {
            PieceType::King => "King",
            PieceType::Queen => "Queen",
            PieceType::Rook => "Rook",
            PieceType::Knight => "Knight",
            PieceType::Bishop => "Bishop",
            PieceType::Pawn => "Pawn",
        }
    }
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
    pub white_castle_possible: bool,
    pub black_castle_possible: bool,
    pub en_passant_target: Option<(usize, usize)>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [[EMPTY; 8]; 8],
            half_move_clock: 0,
            white_castle_possible: true,
            black_castle_possible: true,
            en_passant_target: None,
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
                    if new_row >= 0 && new_row < 8 && self.squares[new_row as usize][col].is_none()
                    {
                        moves.push(((row, col), (new_row as usize, col)));
                    }

                    // Double forward move (only allowed on the starting row and if both squares are empty)
                    let starting_row = if piece.color == Color::White { 1 } else { 6 };
                    if row == starting_row && self.squares[new_row as usize][col].is_none() {
                        let double_row = new_row + direction; // Calculate the row 2 squares ahead
                        if double_row >= 0
                            && double_row < 8
                            && self.squares[double_row as usize][col].is_none()
                        {
                            // Check that the square two steps ahead is empty
                            moves.push(((row, col), (double_row as usize, col)));
                        }
                    }

                    // Diagonal captures (both left and right)
                    for &dc in &[-1, 1] {
                        let new_col = col as isize + dc;
                        if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                            if let Some(dest_piece) =
                                self.squares[new_row as usize][new_col as usize]
                            {
                                if dest_piece.color != piece.color {
                                    moves.push(((row, col), (new_row as usize, new_col as usize)));
                                }
                            }
                        }
                    }

                    // En passant
                    if let Some((target_row, target_col)) = self.en_passant_target {
                        if new_row == target_row as isize && (col as isize + 1 == target_col as isize || col as isize - 1 == target_col as isize) {
                            moves.push(((row, col), (new_row as usize, target_col)));
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
                    // Castling logic
                    if let Some(piece) = self.squares[row][col] {
                        if piece.color == Color::White && self.white_castle_possible {
                            if self.can_castle((row, col), (row, 6)) {
                                // Kingside castling
                                moves.push(((row, col), (row, 6)));
                            }
                            if self.can_castle((row, col), (row, 2)) {
                                // Queenside castling
                                moves.push(((row, col), (row, 2)));
                            }
                        }
                        if piece.color == Color::Black && self.black_castle_possible {
                            if self.can_castle((row, col), (row, 6)) {
                                // Kingside castling
                                moves.push(((row, col), (row, 6)));
                            }
                            if self.can_castle((row, col), (row, 2)) {
                                // Queenside castling
                                moves.push(((row, col), (row, 2)));
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
        if self.can_castle((from_row, from_col), (to_row, to_col)) {
            self.castle((from_row, from_col), (to_row, to_col));
            return;
        }
        if let Some(mut piece) = self.squares[from_row][from_col] {
            self.squares[from_row][from_col] = EMPTY;

            // Update half-move clock on captures or pawn moves
            if piece.kind == PieceType::Pawn || self.squares[to_row][to_col].is_some() {
                // En passant capture
                if Some((to_row, to_col)) == self.en_passant_target {
                    self.squares[from_row][to_col] = None; // Remove captured pawn
                }
                self.half_move_clock = 0; // Reset clock on pawn move or capture
            } else {
                self.half_move_clock += 1;
            }
            if piece.kind == PieceType::Pawn && (to_row == 0 || to_row == 7) {
                // Promote to a Queen (can be extended for other choices)
                piece.kind = PieceType::Queen;
            }
            self.squares[to_row][to_col] = Some(piece);
            // Update en passant target square
            self.en_passant_target = None; // Reset on every move
            if piece.kind == PieceType::Pawn {
                let row_diff = if to_row > from_row {
                    to_row - from_row
                } else {
                    from_row - to_row
                };

                if row_diff == 2 {
                    self.en_passant_target = Some(((from_row + to_row) / 2, from_col));
                }
            }
        }
    }

    pub fn is_square_under_attack(&self, row: usize, col: usize, color: Color) -> bool {
        let opponent_color = opposite_color(color);
        let temp_board = self.clone();
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
    pub fn can_castle(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        // Ensure it's a king moving two squares
        if (from_col == 4) && (to_col == 6 || to_col == 2) && (from_row == to_row) {
            let color = if from_row == 0 {
                Color::White
            } else {
                Color::Black
            };
            let kingside = to_col == 6;
            let rook_col = if kingside { 7 } else { 0 };

            // Castling flag check
            if color == Color::White && !self.white_castle_possible {
                return false;
            }
            if color == Color::Black && !self.black_castle_possible {
                return false;
            }

            // Ensure King and Rook are in their original positions
            if let Some(Piece {
                kind: PieceType::King,
                color: king_color,
            }) = self.squares[from_row][from_col]
            {
                if king_color != color {
                    return false;
                }
            } else {
                return false; // King must be present
            }

            if let Some(Piece {
                kind: PieceType::Rook,
                color: rook_color,
            }) = self.squares[from_row][rook_col]
            {
                if rook_color != color {
                    return false;
                }
            } else {
                return false; // Rook must be present
            }

            // Ensure the squares between the King and Rook are empty
            let range = if kingside { 5..7 } else { 1..4 };
            if range.clone().any(|c| self.squares[from_row][c].is_some()) {
                return false; // Path must be clear
            }

            // Ensure the King is not in check, moving through check, or landing in check
            if self.is_in_check(color) {
                return false;
            }
            for col in range {
                if self.is_square_under_attack(from_row, col, color) {
                    return false;
                }
            }

            // Passed all checks, castling is valid
            return true;
        }

        false // Not a valid castling move
    }

    // Execute the castling move if valid
    pub fn castle(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        if !self.can_castle(from, to) {
            return false;
        }

        let (row, from_col) = from;
        let to_col = to.1;
        let kingside = to_col == 6;
        let rook_col = if kingside { 7 } else { 0 };
        let new_rook_col = if kingside { 5 } else { 3 };

        // Move the King
        self.squares[row][to_col] = self.squares[row][from_col].take();

        // Move the Rook
        self.squares[row][new_rook_col] = self.squares[row][rook_col].take();

        // Disable further castling for this player
        if row == 0 {
            self.white_castle_possible = false;
        } else {
            self.black_castle_possible = false;
        }

        true
    }

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
        let moves: Vec<_> = self
            .generate_all_moves(color)
            .into_iter()
            .filter(|m| self.is_valid_move(m.0, m.1))
            .collect();
        moves.is_empty()
    }

    pub fn is_draw(&self, color: Color) -> bool {
        self.is_stalemate(color) || !self.has_sufficient_material() || self.half_move_clock >= 50
    }

    fn is_stalemate(&self, color: Color) -> bool {
        if self.is_in_check(color) {
            return false;
        }
        let moves: Vec<_> = self
            .generate_all_moves(color)
            .into_iter()
            .filter(|m| self.is_valid_move(m.0, m.1))
            .collect();
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
        if self.can_castle(from, to) {
            return true;
        }

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

// use std::thread;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
struct TranspositionEntry {
    value: i32,
    depth: u32,
    move_type: MoveType,
}

#[derive(Copy, Clone)]
enum MoveType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct PositionKey(u64); // 64-bit key for the board

// Zobrist Hashing Table
pub struct ZobristHasher {
    keys: HashMap<(PieceType, Color, usize, usize), u64>, // Maps each piece on each square to a unique hash
}

impl ZobristHasher {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut keys = HashMap::new();

        // Generate a random hash for each piece type and each square
        for piece_type in &[PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King] {
            for color in &[Color::White, Color::Black] {
                for row in 0..8 {
                    for col in 0..8 {
                        let random_key: u64 = rng.gen();
                        keys.insert((*piece_type, *color, row, col), random_key);
                    }
                }
            }
        }

        ZobristHasher { keys }
    }

    pub fn get_key_for_position(&self, piece: &Piece, row: usize, col: usize) -> u64 {
        *self.keys.get(&(piece.kind, piece.color, row, col)).unwrap()
    }

    // Generate the hash for the entire board
    pub fn compute_board_hash(&self, board: &Board) -> PositionKey {
        let mut hash = 0u64;

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = &board.squares[row][col] {
                    let piece_hash = self.get_key_for_position(piece, row, col);
                    hash ^= piece_hash; // XOR the piece's hash with the current total hash
                }
            }
        }

        PositionKey(hash)
    }
}

pub struct TranspositionTable {
    table: HashMap<PositionKey, TranspositionEntry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, key: &PositionKey) -> Option<&TranspositionEntry> {
        self.table.get(key)
    }

    pub fn insert(&mut self, key: PositionKey, value: i32, depth: u32, move_type: MoveType) {
        self.table.insert(key, TranspositionEntry { value, depth, move_type });
    }
}

// Add game phase calculation
fn calculate_game_phase(board: &Board) -> f32 {
    let mut piece_count = 0;
    let initial_pieces = 16; // 8 pawns + 8 pieces per side
    
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = &board.squares[row][col] {
                if piece.kind != PieceType::King {
                    piece_count += 1;
                }
            }
        }
    }
    
    (piece_count as f32) / (initial_pieces as f32 * 2.0)
}

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
    
    // Piece-Square Tables for positional evaluation
    const PAWN_TABLE: [[i32; 8]; 8] = [
        [0,  0,  0,  0,  0,  0,  0,  0],
        [50, 50, 50, 50, 50, 50, 50, 50],
        [10, 10, 20, 30, 30, 20, 10, 10],
        [5,  5, 10, 25, 25, 10,  5,  5],
        [0,  0,  0, 20, 20,  0,  0,  0],
        [5, -5,-10,  0,  0,-10, -5,  5],
        [5, 10, 10,-20,-20, 10, 10,  5],
        [0,  0,  0,  0,  0,  0,  0,  0]
    ];
    
    const KNIGHT_TABLE: [[i32; 8]; 8] = [
        [-50,-40,-30,-30,-30,-30,-40,-50],
        [-40,-20,  0,  0,  0,  0,-20,-40],
        [-30,  0, 10, 15, 15, 10,  0,-30],
        [-30,  5, 15, 20, 20, 15,  5,-30],
        [-30,  0, 15, 20, 20, 15,  0,-30],
        [-30,  5, 10, 15, 15, 10,  5,-30],
        [-40,-20,  0,  5,  5,  0,-20,-40],
        [-50,-40,-30,-30,-30,-30,-40,-50]
    ];
    
    const BISHOP_TABLE: [[i32; 8]; 8] = [
        [-20,-10,-10,-10,-10,-10,-10,-20],
        [-10,  0,  0,  0,  0,  0,  0,-10],
        [-10,  0,  5, 10, 10,  5,  0,-10],
        [-10,  5,  5, 10, 10,  5,  5,-10],
        [-10,  0, 10, 10, 10, 10,  0,-10],
        [-10, 10, 10, 10, 10, 10, 10,-10],
        [-10,  5,  0,  0,  0,  0,  5,-10],
        [-20,-10,-10,-10,-10,-10,-10,-20]
    ];
    
    fn evaluate_position(board: &Board) -> i32 {
        let mut score = 0;
        let mut white_pawns_per_file = [0; 8];
        let mut black_pawns_per_file = [0; 8];
        let mut white_bishops = 0;
        let mut black_bishops = 0;
        let game_phase = calculate_game_phase(board);
        let mut development_score = 0;
        // First pass: collect piece statistics
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = &board.squares[row][col] {
                    match piece.kind {
                        PieceType::Pawn => {
                            if piece.color == Color::White {
                                white_pawns_per_file[col] += 1;
                            } else {
                                black_pawns_per_file[col] += 1;
                            }
                        }
                        PieceType::Bishop => {
                            if piece.color == Color::White {
                                white_bishops += 1;
                            } else {
                                black_bishops += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    
        // Second pass with updated piece evaluation
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = &board.squares[row][col] {
                    let mut piece_score = get_piece_value(piece);
                    
                    // Early game penalties and development scoring
                    if game_phase > 0.8 { // Early game detection
                        match piece.kind {
                            PieceType::Queen => {
                                // Heavy penalty for early queen moves
                                if (piece.color == Color::White && row != 7) ||
                                (piece.color == Color::Black && row != 0) {
                                    piece_score -= 150; // Significant penalty for early queen development
                                }
                            }
                            PieceType::Knight | PieceType::Bishop => {
                                // Development bonus for minor pieces
                                if (piece.color == Color::White && row != 7) ||
                                (piece.color == Color::Black && row != 0) {
                                    development_score += 30;
                                }
                            }
                            PieceType::Pawn => {
                                // Center pawn development bonus
                                if (3..=4).contains(&row) && (2..=5).contains(&col) {
                                    development_score += 20;
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    // Apply piece-square table bonuses
                    let position_bonus = match piece.kind {
                        PieceType::Pawn => PAWN_TABLE[row][col],
                        PieceType::Knight => KNIGHT_TABLE[row][col],
                        PieceType::Bishop => BISHOP_TABLE[row][col],
                        _ => 0,
                    };
    
                    // Apply color-specific adjustments
                    let (pos_row, pos_bonus) = if piece.color == Color::White {
                        (row, position_bonus)
                    } else {
                        (7 - row, -position_bonus)
                    };
    
                    piece_score += pos_bonus;
    
                    // Pawn structure evaluation
                    if piece.kind == PieceType::Pawn {
                        // Doubled pawns penalty
                        let pawns_in_file = if piece.color == Color::White {
                            white_pawns_per_file[col]
                        } else {
                            black_pawns_per_file[col]
                        };
                        if pawns_in_file > 1 {
                            piece_score -= 20;
                        }
    
                        // Isolated pawns penalty
                        let is_isolated = (col == 0 || white_pawns_per_file[col - 1] == 0) &&
                                        (col == 7 || white_pawns_per_file[col + 1] == 0);
                        if is_isolated {
                            piece_score -= 15;
                        }
    
                        // Passed pawn bonus
                        let is_passed = if piece.color == Color::White {
                            let mut passed = true;
                            for r in (0..pos_row).rev() {
                                if col > 0 && black_pawns_per_file[col - 1] > 0 ||
                                   black_pawns_per_file[col] > 0 ||
                                   col < 7 && black_pawns_per_file[col + 1] > 0 {
                                    passed = false;
                                    break;
                                }
                            }
                            passed
                        } else {
                            let mut passed = true;
                            for r in (pos_row + 1)..8 {
                                if col > 0 && white_pawns_per_file[col - 1] > 0 ||
                                   white_pawns_per_file[col] > 0 ||
                                   col < 7 && white_pawns_per_file[col + 1] > 0 {
                                    passed = false;
                                    break;
                                }
                            }
                            passed
                        };
                        if is_passed {
                            piece_score += 30 + (7 - pos_row as i32) * 5;
                        }
                    }
    
                    // Bishop pair bonus
                    if piece.kind == PieceType::Bishop {
                        if (piece.color == Color::White && white_bishops == 2) ||
                           (piece.color == Color::Black && black_bishops == 2) {
                            piece_score += 30;
                        }
                    }
    
                    // Adjusted mobility scoring
                    let moves = board.generate_moves_for_piece(row, col);
                    let mobility_bonus = match piece.kind {
                        PieceType::Queen => {
                            if game_phase > 0.8 {
                                moves.len() as i32 / 4 // Reduced queen mobility bonus in early game
                            } else {
                                moves.len() as i32 * 2
                            }
                        }
                        PieceType::Knight | PieceType::Bishop => moves.len() as i32 * 2,
                        PieceType::Rook => moves.len() as i32 * 2,
                        _ => moves.len() as i32,
                    };
                    
                    piece_score += mobility_bonus;
                }
            }
        }
    
        // King safety evaluation
        if let Some((white_king_row, white_king_col)) = board.find_king(Color::White) {
            score += evaluate_king_safety(board, Color::White, white_king_row, white_king_col);
        }
        if let Some((black_king_row, black_king_col)) = board.find_king(Color::Black) {
            score -= evaluate_king_safety(board, Color::Black, black_king_row, black_king_col);
        }
    
        score + development_score
    }
    
    fn evaluate_king_safety(board: &Board, color: Color, king_row: usize, king_col: usize) -> i32 {
        let mut safety_score = 0;
    
        // Pawn shield bonus
        let (pawn_row, direction) = if color == Color::White {
            (king_row + 1, -1)
        } else {
            (king_row - 1, 1)
        };
    
        // Check pawn shield
        for col in (king_col.saturating_sub(1))..=(king_col + 1).min(7) {
            if let Some(piece) = &board.squares[pawn_row][col] {
                if piece.kind == PieceType::Pawn && piece.color == color {
                    safety_score += 10;
                }
            }
        }
    
        // Exposed king penalty
        let mut attacking_pieces = 0;
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = &board.squares[row][col] {
                    if piece.color != color {
                        let moves = board.generate_moves_for_piece(row, col);
                        for &(_,(to_row, to_col)) in &moves {
                            if (to_row as i32 - king_row as i32).abs() <= 2 &&
                               (to_col as i32 - king_col as i32).abs() <= 2 {
                                attacking_pieces += 1;
                                safety_score -= match piece.kind {
                                    PieceType::Queen => 4,
                                    PieceType::Rook => 2,
                                    PieceType::Bishop | PieceType::Knight => 1,
                                    _ => 0,
                                };
                            }
                        }
                    }
                }
            }
        }
    
        // Heavy penalty for multiple attackers
        if attacking_pieces > 1 {
            safety_score -= attacking_pieces * 10;
        }
    
        safety_score
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

    let transposition_table = Arc::new(Mutex::new(TranspositionTable::new()));
    let zobrist_hasher = ZobristHasher::new();

    fn alpha_beta(
        board: &Board,
        depth: u32,
        mut alpha: i32,
        mut beta: i32,
        maximizing_player: bool,
        color: Color,
        transposition_table: Arc<Mutex<TranspositionTable>>,
        zobrist_hasher: &ZobristHasher,
    ) -> i32 {
        // Compute the Zobrist hash for the current board state
        let key = zobrist_hasher.compute_board_hash(board);
        let original_alpha = alpha;

        // Check if the position has already been evaluated
        // Enhanced transposition table lookup
        if let Some(entry) = transposition_table.lock().unwrap().get(&key) {
            if entry.depth >= depth {
                match entry.move_type {
                    MoveType::Exact => return entry.value,
                    MoveType::LowerBound => alpha = alpha.max(entry.value),
                    MoveType::UpperBound => beta = beta.min(entry.value),
                }
                if alpha >= beta {
                    return entry.value;
                }
            }
        }
    
        if depth == 0 {
            return evaluate_position(board);
        }
    
        let mut moves = board.generate_all_moves(color);
        moves.sort_by_key(|m| -score_move(board, m));
    
        let mut best_eval = if maximizing_player { i32::MIN } else { i32::MAX };
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
                !maximizing_player,
                opposite_color(color),
                Arc::clone(&transposition_table),
                zobrist_hasher,
            );
            best_eval = if maximizing_player {
                best_eval.max(eval)
            } else {
                best_eval.min(eval)
            };
    
            if maximizing_player {
                alpha = alpha.max(best_eval);
            } else {
                beta = beta.min(best_eval);
            }
    
            if beta <= alpha {
                break;
            }
        }
        let move_type = if best_eval <= original_alpha {
            MoveType::UpperBound
        } else if best_eval >= beta {
            MoveType::LowerBound
        } else {
            MoveType::Exact
        };
    
        transposition_table.lock().unwrap().insert(key, best_eval, depth, move_type); // Store the result in the table
        best_eval
    }

    // Main search logic with thread pool (Rayon example)
    let best_move = Arc::new(Mutex::new(None));
    let best_value = Arc::new(Mutex::new(if color == Color::White {
        i32::MIN
    } else {
        i32::MAX
    }));

    let mut moves = board.generate_all_moves(color);
    moves.sort_by_key(|m| -score_move(board, m));

    // Using Rayon for parallel iteration over moves
    let _handles: Vec<_> = moves.into_par_iter().map(|m| {
        let best_move = Arc::clone(&best_move);
        let best_value = Arc::clone(&best_value);
        let mut new_board = board.clone();
        let new_transposition_table = transposition_table.clone();
        new_board.apply_move(m);

        if let Some(king_pos) = new_board.find_king(color) {
            if new_board.is_square_under_attack(king_pos.0, king_pos.1, color) {
                return (); // Skip invalid move
            }
         } else {
                return (); // Skip invalid move
        }

        let eval = alpha_beta(
            &new_board,
            depth - 1,
            i32::MIN + 1,
            i32::MAX - 1,
            color == Color::Black,
            opposite_color(color),
            new_transposition_table,
            &zobrist_hasher
        );

        let mut best_value = best_value.lock().unwrap();
        let mut best_move = best_move.lock().unwrap();
        if (color == Color::White && eval > *best_value)
            || (color == Color::Black && eval < *best_value)
        {
            *best_value = eval;
            *best_move = Some(m);
        }
    }).collect();

    // Wait for all threads to finish (Rayon handles this internally)
    Arc::try_unwrap(best_move).unwrap().into_inner().unwrap()
}
