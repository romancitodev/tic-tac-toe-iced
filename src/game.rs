#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum Entity {
    #[default]
    Empty, // ""
    Computer, // "X"
    Human,    // "O"
}

/// [`GameState`] its an enum that represents the game state
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub enum GameState {
    #[default]
    /// The board is ready to been played
    Ready,
    /// Players movements.
    Playing(Entity),
    /// If the player select an incorrect cell, this member is used.
    /// So the game understands which entity use in next turn.
    Repeat(Entity),
    /// Only for finals (Someone win | Draw)
    Win(Entity),
    Draw,
}

#[derive(Default)]
pub struct Game {
    board: Board,
    state: GameState,
}

#[derive(Default)]
pub struct Computer;

pub type Board = [[Entity; 3]; 3];

impl Game {
    pub fn reset(&self) -> Game {
        Game::default()
    }

    pub fn board(&self) -> &Board {
        &self.board
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

    pub fn state(&self) -> GameState {
        self.state.clone()
    }

    pub fn start(&mut self) {
        self.set_state(GameState::Playing(Entity::Human));
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
    fn set_move(&self, board: &mut Board, entity: Entity, x: usize, y: usize) {
        board[x][y] = entity
    }

    fn undo_move(&self, board: &mut Board, x: usize, y: usize) {
        board[x][y] = Entity::Empty
    }

    fn is_winner(&self, entity: Entity, board: &Board) -> bool {
        for i in 0..3 {
            if (0..3).all(|j| board[i][j] == entity) || (0..3).all(|j| board[j][i] == entity) {
                return true;
            }
        }

        (0..3).all(|i| board[i][i] == entity) || (0..3).all(|i| board[i][2 - i] == entity)
    }

    pub fn best_play(&mut self, mut board: Board) -> (usize, usize) {
        let mut best_score = i32::MIN;
        let mut best_move = (0, 0);

        let actions = self.actions(&board);

        for (row, col) in actions {
            if board[row][col] == Entity::Empty {
                self.set_move(&mut board, Entity::Computer, row, col);

                let (score, _) = self.minimax(&mut board, Entity::Human, i32::MIN, i32::MAX, 0);

                self.undo_move(&mut board, row, col);

                if score > best_score {
                    best_score = score;
                    best_move = (row, col);
                }
            }
        }

        best_move
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
        if self.is_winner(player, board)
            | self.is_winner(!player, board)
            | board.iter().flatten().all(|e| *e != Entity::Empty)
        {
            return (self.evaluate(board), depth);
        }
        // set the functions:
        let func: fn(i32, i32) -> i32;
        let mut m;
        if player == Entity::Computer {
            func = |a: i32, b: i32| a.max(b);
            m = i32::MIN;
        } else {
            func = |a: i32, b: i32| a.min(b);
            m = i32::MAX;
        }

        for (row, col) in self.actions(board) {
            self.set_move(board, player, row, col);
            let (value, m_depth) = self.minimax(board, !player, alpha, beta, depth + 1);
            depth = m_depth;
            m = func(m, value);
            self.undo_move(board, row, col);
            if player == Entity::Computer {
                alpha = func(alpha, m);
            } else {
                beta = func(beta, m);
            }
            if beta <= alpha {
                break;
            }
        }

        (m, depth)
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

#[allow(dead_code)]
impl GameState {
    pub fn is_finished(&self) -> bool {
        matches!(self, GameState::Draw | GameState::Win(_))
    }

    pub fn is_playable(&self) -> bool {
        matches!(
            self,
            GameState::Playing(_) | GameState::Repeat(_) | GameState::Ready
        )
    }
}

impl Entity {
    pub fn as_str(&self) -> &str {
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
