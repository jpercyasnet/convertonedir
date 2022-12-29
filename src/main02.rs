use rfd::FileDialog;
use iced::widget::{button, column, row, text_input, text, scrollable};
use iced::{Alignment, Element, Sandbox, Length, Settings};
use iced::theme;
pub fn main() -> iced::Result {
    Counterx::run(Settings::default())
}

struct Counterx {
    dir_value: String,
    msg_value: String,
    scrol_value: String,
    input_value: String,
    err_value: bool,
}

// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
enum Message {
    DirPressed,
    InputChanged(String),
}

impl Sandbox for Counterx {
    type Message = Message;
    fn new() -> Self {
        Self { err_value: false, dir_value: "no directory".to_string(), msg_value: "no message".to_string(), input_value: "-----------".to_string(),
               scrol_value: " Some should wrap within the \n \
                            scrollable. Let's output a lot of short words, so \n \
                            that we'll make sure to see how wrapping works \n \
                            that we'll make sure to see how wrapping works \n \
                            that we'll make sure to see how wrapping works \n \
                            that we'll make sure to see how wrapping works \n \
                            that we'll make sure to see how wrapping works \n \
                            that we'll make sure to see how wrapping works \n \
                            with these scrollbars.".to_string()}
    }

    fn title(&self) -> String {
        String::from("Convert one directory -- iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DirPressed => {
               self.msg_value = "getting directory".to_string();
               let folder = FileDialog::new()
                    .pick_folder();
               if folder == None {
                    self.msg_value = "error getting directory -- possible cancel key hit".to_string();
               } else {
                    self.dir_value = folder.unwrap().into_os_string().into_string().expect("REASON").to_string();
                    self.msg_value = "got directory".to_string();
               }
            }
            Message::InputChanged(value) => self.input_value = value,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(30),
                 text(&self.msg_value).size(30),].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Directory Button").on_press(Message::DirPressed).style(theme::Button::Secondary),
                 text(&self.dir_value).size(30),].align_items(Alignment::Center).spacing(10).padding(10),
            scrollable(
                column![
                        text(format!("{}",&self.scrol_value))
                ]
                .width(Length::Fill),
            )
             .height(Length::Units(100)),
            row![text_input("Type something...", &self.input_value, Message::InputChanged).padding(10).size(20),
            ].align_items(Alignment::Center).spacing(10).padding(10),
         ]
        .padding(100)
        .align_items(Alignment::Center)
        .into()
    }
}
