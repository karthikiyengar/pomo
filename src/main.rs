mod lib;
use iced::{
    button, executor, text_input, time, Align, Application, Button, Column, Command, Element, Row,
    Settings, Subscription, Text, TextInput,
};
use lib::format::{get_formatted_duration, play_audio};
use libnotify;
use std::time::Duration;

const POMODORO_SECONDS: u32 = 25 * 60;

pub fn main() -> iced::Result {
    match libnotify::init("pomo") {
        Err(e) => eprintln!("Error initialzing libnotify: {:?}", e),
        _ => println!("libnotify intitialized"),
    };

    Pomo::run(Settings::default())
}

struct Task {
    description: String,
}

#[derive(Default)]
struct Pomo {
    value: u32,
    toggle_button: button::State,
    stop_button: button::State,
    task: text_input::State,
    task_value: String,
    entries: Vec<Task>,
    is_running: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Toggle,
    InputChanged(String),
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
                task: text_input::State::new(),
                task_value: String::from(""),
                entries: vec![],
                is_running: false,
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
                    let n = libnotify::Notification::new(
                        "Pomodoro Completed",
                        Some("Take a short break and get right back at it!"),
                        None,
                    );
                    match n.show() {
                        Err(e) => eprintln!("Error showing message: {:#}", e),
                        _ => (),
                    }

                    play_audio(String::from("assets/beep.wav"));
                    self.update(Message::Stop, _clipboard);
                }
            }
            Message::Toggle => {
                self.is_running = !self.is_running;
            }
            Message::Stop => {
                self.is_running = false;
                self.value = POMODORO_SECONDS;
                self.entries.push(Task {
                    description: self.task_value.clone(),
                });
                self.task_value = String::from("");
            }
            Message::InputChanged(changed) => self.task_value = changed,
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

        let text_input = TextInput::new(
            &mut self.task,
            "What are you working on?",
            &self.task_value,
            Message::InputChanged,
        );

        let entries_row: Column<Message> =
            self.entries.iter_mut().fold(Column::new(), |row, entry| {
                row.push(Text::new(&entry.description))
            });

        let actions_row = Row::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(text_input)
            .push(button_toggle)
            .push(button_stop);

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(get_formatted_duration(self.value)).size(50))
            .push(actions_row)
            .push(entries_row)
            .into()
    }
}
