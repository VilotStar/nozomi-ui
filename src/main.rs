#[allow(unused_imports)]

use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Color,
    Column, Container, Element, Image, Length, Radio, Row, Sandbox, Scrollable,
    Settings, Slider, Space, Text, TextInput
};

const DEF_FONT: Font = Font::External {
    name: "Iconsolata-Regular.ttf",
    bytes: include_bytes!("../resources/Inconsolata-Regular.ttf"),
};

pub fn main() -> iced::Result {
    Main::run(Settings::default())
}

pub struct Main {
    frames: Frames
}

impl Sandbox for Main {
    type Message = Message;

    fn new() -> Main {
        Main { 
            frames: Frames::new()
        }
    }

    fn title(&self) -> String {
        format!("{} - Nozomi", self.frames.title())
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::FrameMessage(frame_msg) => {
                self.frames.update(frame_msg);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Main {
            frames,
            ..
        } = self;

        let controls = Row::new();

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(frames.view().map(Message::FrameMessage))
            .push(controls)
            .into();

        Container::new(content)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

impl Frames {
    fn new() -> Frames {
        Frames {
            frames: vec![
                Frame::LoginFrame
            ],
            current: 0
        }
    }

    fn update(&mut self, msg: FrameMessage) {
        self.frames[self.current].update(msg);
    }

    fn view(&mut self) -> Element<FrameMessage> {
        self.frames[self.current].view()
    }

    fn title(&self) -> &str {
        self.frames[self.current].title()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    FrameMessage(FrameMessage)
}

struct Frames {
    frames: Vec<Frame>,
    current: usize
}

enum Frame {
    LoginFrame {
        value: String
    }
}

#[derive(Debug, Clone)]
pub enum FrameMessage {
    TextInputChanged(String)
}

impl<'a> Frame {
    fn update(&mut self, msg: FrameMessage) {
        match msg {
            FrameMessage::TextInputChanged(value) => {
                if let Frame::TextInput
            }
        }
    }

    fn view(&mut self) -> Element<FrameMessage> {
        match self {
            Frame::LoginFrame { value, state } => Self::loginframe(value, state)
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, FrameMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn title(&self) -> &str {
        match self {
            Frame::LoginFrame { .. } => "Login"
        }
    }

    fn loginframe(state: &'a mut loginframe::State, value &str) -> Column<'a, FrameMessage> {
        let loginframe = TextInput::new(
            state,
            "Username",
            value,
            FrameMessage::TextInputChanged
        )
        .padding(10)
        .size(30);
        Self::container("Login")
            .push(
                Text::new(if value.is_empty() {
                    "Username"
                } else {
                    value
                })
            )
    }
}
