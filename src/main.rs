use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Graphite")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_properly() {
        let mut counter = Counter { value: 0 };

        counter.update(Message::Increment);
        counter.update(Message::Increment);
        counter.update(Message::Decrement);

        assert_eq!(counter.value, 1);
    }
}
