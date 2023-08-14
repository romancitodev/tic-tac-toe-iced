use iced::{
    widget::{button, column, container, row, text},
    Application, Element, Length, Renderer, Settings,
};

#[derive(Debug, Clone)]
enum Message {
    UserClicked(usize, usize),
    ComputerClicked(usize, usize),
    Reset,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
enum Entity {
    #[default]
    Empty,
    Computer,
    Human,
}

type Board = [[Entity; 3]; 3];

#[derive(Default)]
struct Game {
    board: Board,
    state: GameState,
}

#[derive(Default)]
struct Computer {
    board: Board,
    state: GameState,
    entity: Entity,
}

impl Game {
    fn reset(&self) -> Game {
        Game::default()
    }

    fn is_valid_position(&self, x: usize, y: usize) -> bool {
        self.board[x][y] == Entity::Empty
    }

    fn update_board(&mut self, entity: Entity, x: usize, y: usize) {
        self.board[x][y] = entity
    }

    fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    fn state(&self) -> GameState {
        self.state.clone()
    }

    pub fn start(&mut self, entity: Entity) {
        self.set_state(GameState::Playing(entity));
    }

    fn is_winner(&self, entity: Entity, x: usize, y: usize) -> bool {
        if (0..3).all(|i| self.board[x][i] == entity) | (0..3).all(|i| self.board[i][y] == entity) {
            return true;
        }

        if x == y && (0..3).all(|i| self.board[i][i] == entity) {
            return true;
        }

        if x + y == 2 && (0..3).all(|i| self.board[i][2 - i] == entity) {
            return true;
        }

        false
    }

    pub fn update(&mut self, x: usize, y: usize) {
        let entity = match self.state {
            GameState::Playing(s) | GameState::Repeat(s) => s,
            _ => return,
        };

        if !self.is_valid_position(x, y) {
            return self.set_state(GameState::Repeat(entity));
        };

        self.update_board(entity, x, y);

        if self.is_winner(entity, x, y) {
            return self.set_state(GameState::Win(entity));
        }

        if self.board.iter().flatten().all(|e| *e != Entity::Empty) {
            return self.set_state(GameState::Draw);
        }

        self.set_state(GameState::Playing(!entity));
    }
}

impl Computer {
    fn reset(&self) -> Self {
        Self::default()
    }

    fn set_move(&self, board: &mut Board, entity: Entity, x: usize, y: usize) {
        board[x][y] = entity
    }

    fn undo_move(&self, board: &mut Board, x: usize, y: usize) {
        board[x][y] = Entity::Empty
    }

    fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    fn is_winner(&self, entity: Entity, board: &Board) -> bool {
        for i in 0..3 {
            if (0..3).all(|j| board[i][j] == entity) || (0..3).all(|j| board[j][i] == entity) {
                return true;
            }
        }

        (0..3).all(|i| board[i][i] == entity) || (0..3).all(|i| board[i][2 - i] == entity)
    }

    pub fn start(&mut self, entity: Entity) {
        self.set_state(GameState::Playing(entity));
        self.entity = entity;
    }

    pub fn best_play(&mut self, mut board: Board) -> (usize, usize) {
        let (mut score, mut depth) = (i32::MIN, i32::MAX);
        let (min_score, max_score) = (i32::MIN, i32::MAX);
        let mut result = (0, 0);

        let board = &mut board;

        for (row, col) in self.actions(board) {
            self.set_move(board, Entity::Computer, col, row);
            let (v, d) = self.minimax(board, Entity::Human, min_score, max_score, 0);
            if (v > score) | (v == score && d < depth) {
                result = (col, row);
                score = v;
                depth = d;
            }
            self.undo_move(board, col, row);
        }

        result
    }

    fn minimax(
        &mut self,
        board: &mut Board,
        player: Entity,
        mut alpha: i32,
        mut beta: i32,
        mut depth: i32,
    ) -> (i32, i32) /* (score, depth) */ {
        // Check if the board is finished:
        if self.is_winner(player, board) | self.is_winner(!player, board) {
            return (self.evaluate(board), depth);
        }
        // set the functions:
        let func: fn(i32, i32) -> i32;
        let mut score;
        if player == Entity::Computer {
            func = |a: i32, b: i32| a.max(b);
            score = i32::MIN;
        } else {
            func = |a: i32, b: i32| a.min(b);
            score = i32::MAX;
        }

        for (row, col) in self.actions(board) {
            self.set_move(board, player, row, col);
            let (value, m_depth) = self.minimax(board, !player, alpha, beta, depth + 1);
            depth = m_depth;
            score = func(score, value);
            self.undo_move(board, row, col);
            if player == Entity::Computer {
                alpha = func(alpha, value);
            } else {
                beta = func(beta, value);
            }
            if beta <= alpha {
                break;
            }
        }

        (score, depth)
    }

    fn actions(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut positions = vec![];
        for (col_index, col) in board.iter().enumerate() {
            for (row_index, entity) in col.iter().enumerate() {
                if *entity == Entity::Empty {
                    positions.push((col_index, row_index))
                }
            }
        }
        positions
    }

    fn evaluate(&self, board: &Board) -> i32 {
        if self.is_winner(Entity::Computer, board) {
            return 1;
        } else if self.is_winner(Entity::Human, board) {
            return -1;
        }
        0
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
enum GameState {
    #[default]
    Ready,
    Playing(Entity),
    Repeat(Entity),
    Win(Entity),
    Draw,
}

#[derive(Default)]
struct App {
    game: Game,
    ia: Computer,
    text: String,
}

impl Entity {
    fn as_str(&self) -> &str {
        match self {
            Self::Empty => "-",
            Self::Human => "O",
            Self::Computer => "X",
        }
    }
}

impl std::ops::Not for Entity {
    type Output = Entity;

    fn not(self) -> Self::Output {
        match self {
            Self::Empty => Self::Empty,
            Self::Computer => Self::Human,
            Self::Human => Self::Computer,
        }
    }
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                ..Default::default()
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        "Tic Tac Toe".to_string()
    }

    fn update(&mut self, msg: Self::Message) -> iced::Command<Self::Message> {
        if self.game.state() == GameState::Ready {
            self.game.start(Entity::Computer);
            self.ia.start(Entity::Human);
        };
        match msg {
            Message::UserClicked(x, y) => {
                self.game.update(x, y);
                self.update_text();
                if let GameState::Playing(_) = self.game.state() {
                    let (ia_x, ia_y) = self.ia.best_play(self.game.board);
                    println!("{x} {y} :: {ia_x} {ia_y}");
                    return self.update(Message::ComputerClicked(ia_x, ia_y));
                }
            }
            Message::ComputerClicked(x, y) => {
                self.game.update(x, y);
                self.update_text();
            }
            Message::Reset => {
                self.game = self.game.reset();
                self.ia = self.ia.reset();
                self.text.clear()
            }
        };
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let state = self.game.state();
        container(
            column!(
                row![
                    text_button(
                        self.game.board[0][0].as_str(),
                        0,
                        0,
                        matches!(
                            state,
                            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
                        )
                    ),
                    text_button(
                        self.game.board[0][1].as_str(),
                        0,
                        1,
                        matches!(
                            state,
                            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
                        )
                    ),
                    text_button(
                        self.game.board[0][2].as_str(),
                        0,
                        2,
                        matches!(
                            state,
                            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
                        )
                    )
                ]
                .align_items(iced::Alignment::Center)
                .spacing(10),
                row![
                    text_button(
                        self.game.board[1][0].as_str(),
                        1,
                        0,
                        matches!(
                            state,
                            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
                        )
                    ),
                    text_button(
                        self.game.board[1][1].as_str(),
                        1,
                        1,
                        matches!(
                            state,
                            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
                        )
                    ),
                    text_button(
                        self.game.board[1][2].as_str(),
                        1,
                        2,
                        matches!(
                            state,
                            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
                        )
                    )
                ]
                .align_items(iced::Alignment::Center)
                .spacing(10),
                row![
                    text_button(
                        self.game.board[2][0].as_str(),
                        2,
                        0,
                        matches!(
                            state,
                            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
                        )
                    ),
                    text_button(
                        self.game.board[2][1].as_str(),
                        2,
                        1,
                        matches!(
                            state,
                            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
                        )
                    ),
                    text_button(
                        self.game.board[2][2].as_str(),
                        2,
                        2,
                        matches!(
                            state,
                            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
                        )
                    )
                ]
                .align_items(iced::Alignment::Center)
                .spacing(10),
                text(self.text.clone()),
                button("reset").on_press(Message::Reset).padding([10, 20])
            )
            .align_items(iced::Alignment::Center)
            .spacing(10),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }
}

impl App {
    fn update_text(&mut self) {
        match self.game.state() {
            GameState::Draw => {
                self.text = "It's a draw!".to_string();
            }
            GameState::Win(winner) => {
                self.text = format!("{:?} Won!", winner);
            }
            _ => {}
        }
    }
}

fn text_button<'a>(
    content: impl Into<Element<'a, Message, Renderer>>,
    x: usize,
    y: usize,
    op: bool,
) -> button::Button<'a, Message, Renderer> {
    let mut btn = button(content).style(iced::theme::Button::Text).padding(10);
    if op {
        btn = btn.on_press(Message::UserClicked(x, y));
    }
    btn
}

fn main() -> iced::Result {
    App::run(Settings::default())
}

// static int unbeatable_computer(int player, int depth)
// {
//     int is_draw;
//     if (win_game(computer_sign, &is_draw))
//     {
//         return 1;
//     }
//     else if (win_game(user_sign, &is_draw))
//     {
//         return -1;
//     }
//     else if (is_draw)
//     {
//         return 0;
//     }
//     if (player == computer_sign)
//     {
//         int max_eval = INT_MIN;
//         int movei, movej;
//         for (int i = 0; i < 3; i++)
//         {
//             for (int j = 0; j < 3; j++)
//             {
//                 if (values[i][j] == EMPTY)
//                 {
//                     values[i][j] = computer_sign;
//                     int evaluation = unbeatable_computer(user_sign, depth + 1);
//                     if (evaluation > max_eval)
//                     {
//                         max_eval = evaluation;
//                         movei = i;
//                         movej = j;
//                     }
//                     values[i][j] = EMPTY;
//                 }
//             }
//         }
//         if (depth == 0)
//         {
//             values[movei][movej] = computer_sign;
//         }
//         return max_eval;
//     }
//     else
//     {
//         int min_eval = INT_MAX;
//         int movei, movej;
//         for (int i = 0; i < 3; i++)
//         {
//             for (int j = 0; j < 3; j++)
//             {
//                 if (values[i][j] == EMPTY)
//                 {
//                     values[i][j] = user_sign;
//                     int evaluation = unbeatable_computer(computer_sign, depth + 1);
//                     if (evaluation < min_eval)
//                     {
//                         min_eval = evaluation;
//                         movei = i;
//                         movej = j;
//                     }
//                     values[i][j] = EMPTY;
//                 }
//             }
//         }
//         if (depth == 0)
//         {
//             values[movei][movej] = user_sign;
//
