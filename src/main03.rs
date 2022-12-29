use rfd::FileDialog;
use iced::widget::{button, column, row, text_input, text, scrollable};
use iced::{Alignment, Element, Sandbox, Length, Settings};
use iced::theme;

mod get_dirlist;

use get_dirlist::get_dirlist;

pub fn main() -> iced::Result {
    Counterx::run(Settings::default())
}

struct Counterx {
    dir_value: String,
    msg_value: String,
    scrol_value: String,
    hhmmss_value: String,
    size_value: String,
    err_value: bool,
}

// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
enum Message {
    DirPressed,
    HhmmssChanged(String),
    SizeChanged(String),
}

impl Sandbox for Counterx {
    type Message = Message;
    fn new() -> Self {
        Self { err_value: false, dir_value: "no directory".to_string(), msg_value: "no message".to_string(), hhmmss_value: "hh:mm:ss".to_string(),
               size_value: "10".to_string(), scrol_value: " No directory selected \n \
                            ".to_string()}
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
                    self.dir_value = folder.as_ref().expect("REASON").display().to_string();
                    let current_dir = folder;
                    let (errcd, errstr, newliststr) = get_dirlist(current_dir.unwrap());
                    if errcd == 0 {
                        self.scrol_value  = newliststr;
                        self.msg_value = "got directory".to_string();
                    } else {
                        self.msg_value = errstr.to_string();
                    }
               }
            }
            Message::HhmmssChanged(value) => self.hhmmss_value = value,
            Message::SizeChanged(value) => self.size_value = value,
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
            row![text("hhmmss predecessor").size(20),
                 text_input("No input....", &self.hhmmss_value, Message::HhmmssChanged).padding(10).size(20),
                 text("size predecessor"),
                 text_input("No input....", &self.size_value, Message::SizeChanged).padding(10).size(20),
            ].align_items(Alignment::Center).spacing(10).padding(10),
         ]
        .padding(10)
        .align_items(Alignment::Start)
        .into()
    }
}
