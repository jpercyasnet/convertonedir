use iced::widget::{button, column, row, text_input, text};
use iced::{Alignment, Element, Sandbox, Settings, Color};
pub fn main() -> iced::Result {
    Counterx::run(Settings::default())
}

struct Counterx {
    dir_value: String,
    msg_value: String,
    err_value: bool,
}

// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
enum Message {
    DirPressed,
}

impl Sandbox for Counterx {
    type Message = Message;
    fn new() -> Self {
        Self { err_value: false, dir_value: "no directory".to_string(), msg_value: "no message".to_string(),}
    }

    fn title(&self) -> String {
        String::from("Get Directory -- Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DirPressed => {
               self.msg_value = "getting directory".to_string();
//               let path = FileDialog::new()
//                   .set_location("~/Desktop")
//                   .show_open_single_dir()
//                   .unwrap();
//               if path == None {
//                    self.msg_value = "error getting directory -- possible cancel key hit".to_string();
//               } else {
//                    self.dir_value = path.unwrap().into_os_string().into_string().expect("REASON").to_string();
//                    self.msg_value = "got directory".to_string();
//              }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(30),
                 text(&self.msg_value).size(30),].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Directory Button").on_press(Message::DirPressed),
                 text(&self.dir_value).size(30),].align_items(Alignment::Center).spacing(10).padding(10),
        ]
        .padding(100)
        .align_items(Alignment::Center)
        .into()
    }
}
