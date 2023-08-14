use iced::{
    widget::{button, column, container, row, text},
    Application, Element, Length, Renderer, Settings,
};

mod game;
use game::*;

#[derive(Debug, Clone)]
enum Message {
    UserClicked(usize, usize),
    ComputerClicked(usize, usize),
    Reset,
}

#[derive(Default)]
struct App {
    game: game::Game,
    ia: game::Computer,
    text: String,
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
        if self.game.state() == game::GameState::Ready {
            self.game.start();
        };
        match msg {
            Message::UserClicked(x, y) => {
                self.game.update(x, y);
                self.update_text();
                if let GameState::Playing(_) = self.game.state() {
                    let (x, y) = self.ia.best_play(*self.game.board());
                    return self.update(Message::ComputerClicked(x, y));
                }
            }
            Message::ComputerClicked(x, y) => {
                self.game.update(x, y);
                self.update_text();
            }
            Message::Reset => {
                self.game = self.game.reset();
                self.text.clear()
            }
        };
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let activate = self.game.state().is_playable();
        container(
            column!(
                row![
                    text_button(self.game.board()[0][0].as_str(), 0, 0, activate),
                    text_button(self.game.board()[0][1].as_str(), 0, 1, activate),
                    text_button(self.game.board()[0][2].as_str(), 0, 2, activate)
                ]
                .align_items(iced::Alignment::Center)
                .spacing(10),
                row![
                    text_button(self.game.board()[1][0].as_str(), 1, 0, activate),
                    text_button(self.game.board()[1][1].as_str(), 1, 1, activate),
                    text_button(self.game.board()[1][2].as_str(), 1, 2, activate)
                ]
                .align_items(iced::Alignment::Center)
                .spacing(10),
                row![
                    text_button(self.game.board()[2][0].as_str(), 2, 0, activate),
                    text_button(self.game.board()[2][1].as_str(), 2, 1, activate),
                    text_button(self.game.board()[2][2].as_str(), 2, 2, activate)
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
