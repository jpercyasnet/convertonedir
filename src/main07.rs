// use rfd::FileDialog;
use iced::widget::{button, column, row, text_input, text, scrollable, progress_bar, ProgressBar};
use iced::{Alignment, Element, Command, Application, Length, Settings, Color, Theme, Renderer};
use iced::theme;
use iced::executor;
use std::path::Path;
mod get_dirlist;
mod dirpressx;
mod diroutpressx;
mod create_mergelist;
mod parse_moddate;
mod dump_file;
mod get_strvector;

use get_dirlist::get_dirlist;
use dirpressx::dirpressx;
use diroutpressx::diroutpressx;
use create_mergelist::create_mergelist;
use std::process::Command as stdCommand;
use std::fs;

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

impl Application for Counterx {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;
    fn new(_flags: Self::Flags) -> (Counterx, iced::Command<Message>) {
        ( Self { err_value: false, dir_value: "no directory".to_string(), msg_value: "no message".to_string(), hhmmss_value: "-00:00:00:00:00:00".to_string(),
               size_value: "10".to_string(), mess_color: Color::from([0.0, 0.0, 0.0]), outdir_value: "no directory".to_string(), progval: 0.0,
               scrol_value: " No directory selected \n \
                            ".to_string(),
               mergescrol_value: " No directory selected \n \
                            ".to_string(),
          },
          Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Convert one directory -- iced")
    }

    fn update(&mut self, message: Message) -> Command<Message>  {
        match message {
            Message::DirPressed => {
               let (colorout, errstr, newdir, newliststr) = dirpressx();
               self.scrol_value  = newliststr.to_string();
               self.dir_value = newdir.to_string();
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               Command::none()
            }
            Message::HhmmssChanged(value) => { self.hhmmss_value = value; Command::none() }
            Message::SizeChanged(value) => { self.size_value = value; Command::none() }
            Message::OutDirPressed => {
               let (colorout, errstr, newdir) = diroutpressx();
               self.outdir_value = newdir.to_string();
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               Command::none()
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
               if Path::new(&self.dir_value).exists() {
                   if Path::new(&self.outdir_value).exists() {
                       let mut bolok = true;
                       for entry1 in fs::read_dir(&self.outdir_value).unwrap() {
                            let entry = entry1.unwrap();
                            if let Ok(metadata) = entry.metadata() {
                                if let Ok(_file_name) = entry.file_name().into_string() {
                                    if metadata.is_file() {
                                        bolok = false;
                                    }
                                }
                            }
                       }
                       if bolok {
                           let mergelistvec: Vec<&str> = self.mergescrol_value[0..].split("\n").collect();
                           let mut lenmg1 = mergelistvec.len();
                           if lenmg1 < 2 {
                               self.msg_value = "no values in merge list".to_string();
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                           } else {
                               let mut numrow = 0;
                               let mut numprocess = 0;
                               lenmg1 = lenmg1 -1;
                               for indl in 0..lenmg1 {
                                    let str_cur_dirfrom = self.dir_value.clone();
                                    let linestr = mergelistvec[indl].clone();
                                    let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
                                    let filefromx = lineparse[1].clone().to_string();
                                    let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx;
                                    if !Path::new(&fullfrom).exists() {
                                        self.msg_value = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
                                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                        bolok = false;
                                        break;
                                    }
                                    let str_cur_dirout = self.outdir_value.clone();
                                    let fileprex = lineparse[2].clone().to_string();
                                    let filetox = lineparse[3].clone().to_string();
                                    let fullto = str_cur_dirout.clone() + "/" + &fileprex + "_" + &filetox;
                                    if Path::new(&fullto).exists() {
                                        self.msg_value = format!("********* convert Copy: ERROR {} already exists **********", fullto);
                                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                        bolok = false;
                                        break;
                                    }
                                    if numprocess < 4 {
                                        stdCommand::new("cp")
                                          .arg("-p")
                                          .arg(&fullfrom)
                                          .arg(&fullto)
                                          .spawn()
                                          .expect("failed to execute process");
                                        numprocess = numprocess + 1;
                                    } else {
                                        let _output = stdCommand::new("cp")
                                                    .arg("-p")
                                                    .arg(&fullfrom)
                                                    .arg(&fullto)
                                                    .output()
                                                    .expect("failed to execute process");
                                        numprocess = 0;
                                    }

//                                    println!("cp -p {} {}", fullfrom, fullto);

                                    numrow = numrow + 1;
                                    let progval = (numrow as f32 / lenmg1 as f32) * 100.0;
                                    ProgressBar::<Renderer>::new(0.0..=100.0, progval);
                               }
                               if bolok {
                                   self.msg_value = format!("convert copy copied {} files", lenmg1);
                                   self.mess_color = Color::from([0.0, 0.0, 0.0]);
                               }
                           }
                       } else {
                               self.msg_value = "the output directory has files in it".to_string();
                               self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       }
                   } else {
                       self.msg_value = "the output directory does not exist".to_string();
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   }
               } else {
                   self.msg_value = "the directory does not exist".to_string();
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
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
