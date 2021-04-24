mod utils;

use iced::time;
use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Row, Settings,
    Subscription, Text,
};
use std::time::Duration;
use utils::format::{get_formatted_duration, play_audio};

const POMODORO_SECONDS: u32 = 5; //25 * 60;

pub fn main() -> iced::Result {
    Pomo::run(Settings::default())
}

#[derive(Default)]
struct Pomo {
    value: u32,
    toggle_button: button::State,
    stop_button: button::State,
    is_running: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    Toggle,
    Stop,
}

impl Application for Pomo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Pomo {
                value: POMODORO_SECONDS,
                toggle_button: button::State::new(),
                stop_button: button::State::new(),
                is_running: true,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Message::Tick => {
                if self.value > 0 {
                    self.value -= 1;
                }
                if self.value == 0 {
                    self.is_running = false;
                    play_audio(String::from("../assets/beep.wav"));
                }
            }
            Message::Toggle => {
                self.is_running = !self.is_running;
            }
            Message::Stop => {
                self.is_running = false;
                self.value = POMODORO_SECONDS;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_running {
            time::every(Duration::from_millis(1000)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let button_toggle = Button::new(
            &mut self.toggle_button,
            Text::new(if self.is_running { "Pause" } else { "Start" }),
        )
        .on_press(Message::Toggle);

        let button_stop =
            Button::new(&mut self.stop_button, Text::new("Stop")).on_press(Message::Stop);

        let actions_row = Row::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(button_toggle)
            .push(button_stop);

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(get_formatted_duration(self.value)).size(50))
            .push(actions_row)
            .into()
    }
}
