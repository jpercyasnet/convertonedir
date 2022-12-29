use rfd::FileDialog;
use iced::widget::{button, column, row, text_input, text, scrollable, progress_bar};
use iced::{Alignment, Element, Sandbox, Length, Settings, Color};
use iced::theme;
use std::path::Path;
mod get_dirlist;
mod create_mergelist;
mod parse_moddate;
mod dump_file;
mod get_strvector;

use get_dirlist::get_dirlist;
use create_mergelist::create_mergelist;


pub fn main() -> iced::Result {
    Counterx::run(Settings::default())
}

struct Counterx {
    dir_value: String,
    mess_color: Color,
    msg_value: String,
    scrol_value: String,
    hhmmss_value: String,
    size_value: String,
    outdir_value: String,
    mergescrol_value: String,
    progval: f32,
    err_value: bool,
}

// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
enum Message {
    DirPressed,
    OutDirPressed,
    HhmmssChanged(String),
    SizeChanged(String),
    MergePressed,
    CopyPressed,
}

impl Sandbox for Counterx {
    type Message = Message;
    fn new() -> Self {
        Self { err_value: false, dir_value: "no directory".to_string(), msg_value: "no message".to_string(), hhmmss_value: "-00:00:00:00:00:00".to_string(),
               size_value: "10".to_string(), mess_color: Color::from([0.0, 0.0, 0.0]), outdir_value: "no directory".to_string(), progval: 0.0,
               scrol_value: " No directory selected \n \
                            ".to_string(),
               mergescrol_value: " No directory selected \n \
                            ".to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("Convert one directory -- iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DirPressed => {
               self.msg_value = "getting directory".to_string();
               self.mess_color = Color::from([0.0, 0.0, 0.0]);
               let folder = FileDialog::new()
                    .pick_folder();
               if folder == None {
                    self.msg_value = "error getting directory -- possible cancel key hit".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
               } else {
                    self.dir_value = folder.as_ref().expect("REASON").display().to_string();
                    let current_dir = folder;
                    let (errcd, errstr, newliststr) = get_dirlist(current_dir.unwrap());
                    if errcd == 0 {
                        self.scrol_value  = newliststr;
                        self.msg_value = "got directory".to_string();
                        self.mess_color = Color::from([0.0, 0.0, 0.0]);
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    }
               }
            }
            Message::HhmmssChanged(value) => self.hhmmss_value = value,
            Message::SizeChanged(value) => self.size_value = value,
            Message::OutDirPressed => {
               self.msg_value = "getting output directory".to_string();
               self.mess_color = Color::from([0.0, 0.0, 0.0]);
               let folder = FileDialog::new()
                    .pick_folder();
               if folder == None {
                    self.msg_value = "error getting output directory -- possible cancel key hit".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
               } else {
                    self.outdir_value = folder.as_ref().expect("REASON").display().to_string();
                    self.msg_value = "convert output directory selected".to_string();
                    self.mess_color = Color::from([0.0, 0.0, 0.0]);
               }
            }
            Message::MergePressed => {
               self.msg_value = "merge input into list directory".to_string();
               self.mess_color = Color::from([0.0, 0.0, 0.0]);
               if Path::new(&self.dir_value).exists() {
                   let (errcd, errstr, newliststr) = create_mergelist(self.dir_value.clone(), self.size_value.clone(), self.hhmmss_value.clone());
                   if errcd == 0 {
                       self.mergescrol_value  = newliststr;
                       self.msg_value = "created merge list".to_string();
                       self.mess_color = Color::from([0.0, 0.0, 0.0]);
                   } else {
                       self.msg_value = errstr.to_string();
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   }
               } else {
                   self.msg_value = "the directory does not exist".to_string();
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
            }
            Message::CopyPressed => {
               self.msg_value = "Copy button pressed".to_string();
               self.mess_color = Color::from([0.0, 0.0, 0.0]);
            }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(30),
                 text(&self.msg_value).size(30).style(*&self.mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Directory Button").on_press(Message::DirPressed).style(theme::Button::Secondary),
                 text(&self.dir_value).size(30),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            scrollable(
                column![
                        text(format!("{}",&self.scrol_value))
                ].width(Length::Fill),
            ).height(Length::Units(100)),
            row![text("date mod value (-YY:MM:DD:hh:mm:ss): ").size(20),
                 text_input("No input....", &self.hhmmss_value, Message::HhmmssChanged).padding(10).size(20),
                 text("     Length of File Description: "),
                 text_input("No input....", &self.size_value, Message::SizeChanged).padding(10).size(20),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("outDirectory Button").on_press(Message::OutDirPressed).style(theme::Button::Secondary),
                 text(&self.outdir_value).size(30),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            scrollable(
                column![
                        text(format!("{}",&self.mergescrol_value))
                ].width(Length::Fill),
            ).height(Length::Units(100)),
            row![button("Merge Button").on_press(Message::MergePressed).style(theme::Button::Secondary),
                 button("Copy Button").on_press(Message::CopyPressed).style(theme::Button::Secondary),
            ].align_items(Alignment::Center).spacing(100).padding(10),
            progress_bar(0.0..=100.0,self.progval),
         ]
        .padding(10)
        .align_items(Alignment::Start)
        .into()
    }
}
