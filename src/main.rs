use iced::{button, Align, Button, Column, Element, Font, Sandbox, Settings, Text};

const DEF_FONT: Font = Font::External {
    name: "Iconsolata-Regular.ttf",
    bytes: include_bytes!("../resources/Inconsolata-Regular.ttf"),
};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: String,
    put_box: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PutInBox
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Nozomi")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PutInBox => {
                self.value = String::from("human genitals");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(
                    &mut self.put_box,
                    Text::new("Put in da box").font(DEF_FONT),
                )
                .on_press(Message::PutInBox),
            )
            .push(Text::new(self.value.to_string()).size(50).font(DEF_FONT))
            .into()
    }
}
