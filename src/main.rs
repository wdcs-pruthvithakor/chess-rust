use iced::{
    border::Radius, widget::{button, image, slider, Row, Column, Text, Container, Button},Theme, Task, Background, Border, Color as IcedColor, Element, Length, Shadow
};
use iced::widget::Image;
mod engine;
use engine::{Board, Color, PieceType, best_move_for_color,improved_best_move_for_color, opposite_color};

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Winner(Color),
    Draw
}

#[derive(Debug, Clone)]
enum AppState {
    SelectingDifficulty,
    Playing,
    GameOver(GameResult),
}

#[derive(Debug, Clone)]
enum Message {
    SquareClicked(usize, usize),
    BotMove,
    DifficultySelected,
    SliderChanged(f32),
    EndGame(GameResult),
    Restart
}

// #[derive(Debug)]
struct ChessApp {
    board: Board,
    // The currently selected square by the human player, if any.
    selected: Option<(usize, usize)>,
    // Whose turn it is. We assume the human plays White.
    current_turn: Color,
    // Difficulty (minimax depth) for the bot.
    difficulty: u32,
    slider_value: f32,
    state: AppState, // Add a state tracker
}

impl Default for ChessApp {
    fn default() -> Self {
        ChessApp {
            board: Board::new(),
            selected: None,
            current_turn: Color::White,
            difficulty: 3, // Adjust for desired bot strength.
            slider_value: 3.0,
            state: AppState::SelectingDifficulty, // Start with difficulty selection
        }
    }
}

/// Update function for the application.
/// It receives a mutable reference to our state and a message,
/// and returns a Command that can trigger asynchronous actions.
fn update(app: &mut ChessApp, message: Message) -> Task<Message>  {
    match message {
        Message::SliderChanged(value) => {
            app.slider_value = value;
        }
        Message::Restart => {
            *app = ChessApp::default();
        }
        Message::DifficultySelected => {
            app.difficulty = app.slider_value.round() as u32; // Save slider value as difficulty
            app.state = AppState::Playing;
        }
        Message::SquareClicked(row, col) => {
            // Allow human moves only when it's White's turn.
            println!("turn: {:?}", app.current_turn);
            if app.current_turn == Color::White {
                
                if let Some((sel_row, sel_col)) = app.selected {
                    println!("selected: {} {}", sel_row, sel_col);
                    // Attempt to move from the selected square to the clicked square.
                    if app.board.is_valid_move((sel_row, sel_col), (row, col)) {
                        app.board.apply_move(((sel_row, sel_col), (row, col)));
                        app.selected = None;
                        app.current_turn = opposite_color(app.current_turn);
                        if app.board.is_checkmate(app.current_turn) || app.board.find_king(app.current_turn) == Some((row, col)) {
                            let winner = GameResult::Winner(opposite_color(app.current_turn));
                            return Task::perform(async { () }, move |_| Message::EndGame(winner));
                        } else if app.board.is_draw(app.current_turn) {
                            return Task::perform(async { () },  |_| Message::EndGame(GameResult::Draw));
                        }
                        // After the human move, trigger the bot move asynchronously.
                        return Task::perform(async { () }, |_| Message::BotMove);
                    } else {
                        // Clear selection on an invalid move.
                        app.selected = None;
                        println!("invalid move");
                    }
                } else {
                    println!("selectting");
                    // No square is currently selected; select the square if it contains a White piece.
                    if let Some(piece) = app.board.squares[row][col] {
                        if piece.color == Color::White {
                            app.selected = Some((row, col));
                        }
                    }
                }
            }
        }
        Message::BotMove => {
            // Bot moves as Black.
            if app.current_turn == Color::Black {
                if app.board.is_in_check(opposite_color(app.current_turn)) {
                    let winner = GameResult::Winner(app.current_turn);
                    return Task::perform(async { () }, move |_| Message::EndGame(winner));
                }
                if let Some(mv) = improved_best_move_for_color(&app.board, Color::Black, app.difficulty) {
                    app.board.apply_move(mv);
                    app.current_turn = opposite_color(app.current_turn);
                    if app.board.is_checkmate(app.current_turn){
                        let winner = GameResult::Winner(opposite_color(app.current_turn));
                        return Task::perform(async { () }, move |_| Message::EndGame(winner));
                    } else if app.board.is_draw(app.current_turn) {
                        return Task::perform(async { () },  |_| Message::EndGame(GameResult::Draw));
                    }
                }
            }
        }
        Message::EndGame(result) => {
            app.state = AppState::GameOver(result);
        }
    }
    Task::none()
}

/// View function for the application.
/// It receives an immutable reference to our state and returns an Element.
fn view(app: &ChessApp) -> Element<Message> {
     match &app.state {
        AppState::SelectingDifficulty => {
            Column::new()
                .push(Text::new("Select Difficulty"))
                .push(
                    slider(2.0..=9.0, app.slider_value, Message::SliderChanged)
                        .step(1.0) // Step makes it snap to whole numbers
                )
                .push(Text::new(format!("Difficulty: {}", app.slider_value.round() as u32)))
                .push(
                    Button::new(Text::new("Start Game"))
                        .on_press(Message::DifficultySelected)
                )
                .padding(20)
                .spacing(10)
                .into()
        }
        AppState::Playing => {
            let mut board_view = Column::new().spacing(0);

            for r in 0..8 {
                let mut row_view = Row::new().spacing(0);
                for c in 0..8 {
                    let is_light = (r + c) % 2 == 0;
                    let square_color = if is_light { "#F0D9B5" } else { "#B58863" };

                    // Highlight selected square
                    let highlight_color = if let Some((sel_row, sel_col)) = app.selected {
                        if r == sel_row && c == sel_col {
                            "#FFDD00" // A bright yellow for the selected square
                        } else {
                            square_color // Default square color
                        }
                    } else {
                        square_color // Default square color if nothing is selected
                    };

                    let square_content: Element<'static, Message> = app.board.squares[r][c].and_then(|piece| {
                        let asset: &str = match (piece.color, piece.kind) {
                            (Color::White, PieceType::Pawn) => "assets/white_pawn.jpeg",
                            (Color::Black, PieceType::Pawn) => "assets/black_pawn.png",
                            (Color::White, PieceType::King) => "assets/white_king.jpeg",
                            (Color::Black, PieceType::King) => "assets/black_king.png",
                            (Color::White, PieceType::Queen) => "assets/white_queen.jpeg",
                            (Color::Black, PieceType::Queen) => "assets/black_queen.jpeg",
                            (Color::White, PieceType::Rook) => "assets/white_rook.png",
                            (Color::Black, PieceType::Rook) => "assets/black_rook.png",
                            (Color::White, PieceType::Knight) => "assets/white_knight.jpeg",
                            (Color::Black, PieceType::Knight) => "assets/black_knight.jpeg",
                            (Color::White, PieceType::Bishop) => "assets/white_bishop.jpeg",
                            (Color::Black, PieceType::Bishop) => "assets/black_bishop.png",
                        };
                        let handle = image::Handle::from_path(asset); // Create the handle
                        Some(Image::new(handle).into())
                    }).unwrap_or_else(|| { // Handle the None case directly
                        Container::new(Text::new(""))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x(Length::Fill)
                            .center_y(Length::Fill)
                            .into()
                    });

                    let square = Button::new(square_content) // Use Button directly with container
                        .style(|_theme: &Theme, _style| BoardSquareStyle { color: highlight_color }.style()) // Style the Button
                        .on_press(Message::SquareClicked(r, c))
                        .width(Length::FillPortion(1))
                        .height(Length::FillPortion(1));

                        row_view = row_view.push(square); // Reassign row_view
                    }
                    board_view = board_view.push(row_view); // Reassign board_view
                }
            
                board_view.into() // Convert the final Column to an Element

        }
        AppState::GameOver(result) => {
            let result_text = match result {
                GameResult::Winner(color) => format!("{:?} Wins!", color),
                GameResult::Draw => "It's a Draw!".to_string(),
            };

            Column::new()
                .push(Text::new("Game Over"))
                .push(Text::new(result_text))
                .push(
                    Button::new(Text::new("Play Again"))
                        .on_press(Message::Restart) // Restart game
                )
                .padding(20)
                .spacing(10)
                .into()
        }
    }
}

/// Helper struct for styling a board square.
struct BoardSquareStyle {
    color: &'static str,
}

impl BoardSquareStyle {
    /// Returns a style for a container representing a board square.
    fn style(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(hex_to_color(self.color))),
            border: Border {
                color:IcedColor::BLACK,
                width: 0.0,
                radius: Radius::new(1.0),
            },
            text_color: IcedColor::TRANSPARENT,
            shadow: Shadow::default(),
        }
    }
}

/// Converts a hex color string (e.g. "#F0D9B5") to an IcedColor.
fn hex_to_color(hex: &str) -> IcedColor {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
    IcedColor::from_rgb8(r, g, b)
}

fn main() -> iced::Result {
    // Use the iced::application helper to create and run the application.
    iced::application("Rust Chess", update, view)
        .run()
}
