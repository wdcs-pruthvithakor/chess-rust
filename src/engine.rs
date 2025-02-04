// engine.rs

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceType,
}

pub const EMPTY: Option<Piece> = None;

#[derive(Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [[EMPTY; 8]; 8],
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
                    if row == starting_row {
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
                                if dest_piece.color != piece.color {
                                    moves.push(((row, col), (new_row as usize, new_col as usize)));
                                }
                            } else {
                                moves.push(((row, col), (new_row as usize, new_col as usize)));
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
        self.squares[to_row][to_col] = self.squares[from_row][from_col];
        self.squares[from_row][from_col] = EMPTY;
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

fn minimax(board: &Board, depth: u32, maximizing: bool, color: Color) -> i32 {
    if depth == 0 {
        return evaluate_board(board);
    }
    let moves = board.generate_all_moves(color);
    if moves.is_empty() {
        return evaluate_board(board);
    }
    if maximizing {
        let mut max_eval = i32::MIN;
        for m in moves {
            let mut new_board = board.clone();
            new_board.apply_move(m);
            let eval = minimax(&new_board, depth - 1, false, opposite_color(color));
            max_eval = max_eval.max(eval);
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for m in moves {
            let mut new_board = board.clone();
            new_board.apply_move(m);
            let eval = minimax(&new_board, depth - 1, true, opposite_color(color));
            min_eval = min_eval.min(eval);
        }
        min_eval
    }
}

pub fn best_move_for_color(
    board: &Board,
    color: Color,
    depth: u32,
) -> Option<((usize, usize), (usize, usize))> {
    let moves = board.generate_all_moves(color);
    let mut best_move = None;
    let mut best_eval = if color == Color::White {
        i32::MIN
    } else {
        i32::MAX
    };

    for m in moves {
        let mut new_board = board.clone();
        new_board.apply_move(m);
        let eval = minimax(
            &new_board,
            depth - 1,
            color == Color::Black,
            opposite_color(color),
        );
        if (color == Color::White && eval > best_eval)
            || (color == Color::Black && eval < best_eval)
        {
            best_eval = eval;
            best_move = Some(m);
        }
    }
    best_move
}

pub fn opposite_color(color: Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}
