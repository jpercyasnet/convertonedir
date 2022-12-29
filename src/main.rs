// use rfd::FileDialog;
use iced::widget::{button, column, row, text_input, text, scrollable, progress_bar};
use iced::{Alignment, Element, Command, Application, Length, Settings, Color, Theme, Renderer};
use iced::theme;
use iced::executor;
use std::process::Command as stdCommand;
use std::path::{Path};
use iced::widget::{ProgressBar};

mod get_dirlist;
mod dirpressx;
mod diroutpressx;
mod create_mergelist;
mod parse_moddate;
mod dump_file;
mod get_strvector;
mod mergepressx;
mod copypressx;
// mod asynccopy;
use get_dirlist::get_dirlist;
use dirpressx::dirpressx;
use diroutpressx::diroutpressx;
use mergepressx::mergepressx;
use copypressx::copypressx;
use create_mergelist::create_mergelist;

pub fn main() -> iced::Result {
    Counterx::run(Settings::default())
}

//#[derive(Debug)]
//enum Copyxex {
//    Loaded { copyx: Copyx },
//    Errored,
//}

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
    CopyxFound(Result<Copyx, Error>),
}

impl Application for Counterx {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;
    fn new(_flags: Self::Flags) -> (Counterx, iced::Command<Message>) {
        ( Self { dir_value: "no directory".to_string(), msg_value: "no message".to_string(), hhmmss_value: "-00:00:00:00:00:00".to_string(),
               size_value: "10".to_string(), mess_color: Color::from([0.0, 0.0, 0.0]), outdir_value: "no directory".to_string(), progval: 0.0,
               scrol_value: " No directory selected \n \
                            ".to_string(),
               mergescrol_value: " No directory selected \n \
                            ".to_string(),
          },
          Command::none()
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
               let (colorout, errstr, newliststr) = mergepressx(self.dir_value.clone(),self.size_value.clone(), self.hhmmss_value.clone());
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               self.mergescrol_value  = newliststr;
               Command::none()
            }
            Message::CopyPressed => {
               let (errcode, colorout, errstr) = copypressx(self.dir_value.clone(),self.outdir_value.clone(), self.mergescrol_value.clone());
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               if errcode == 0 {
                   Command::perform(Copyx::copyit(self.dir_value.clone(),self.outdir_value.clone(), self.mergescrol_value.clone()), Message::CopyxFound)

               } else {
                   Command::none()
               }
            }
              Message::CopyxFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                Command::none()
             }
             Message::CopyxFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                Command::none()
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
#[derive(Debug, Clone)]
struct Copyx {
    errcolor: Color,
    errval: String,
}

impl Copyx {
//    const TOTAL: u16 = 807;

    async fn copyit(dir_value: String, outdir_value: String, mergescrol_value: String) -> Result<Copyx, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 0.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl].clone();
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[1].clone().to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx;
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          let str_cur_dirout = outdir_value.clone();
          let fileprex = lineparse[2].clone().to_string();
          let filetox = lineparse[3].clone().to_string();
          let fullto = str_cur_dirout.clone() + "/" + &fileprex + "_" + &filetox;
          if Path::new(&fullto).exists() {
              errstring = format!("********* convert Copy: ERROR {} already exists **********", fullto);
              colorx = Color::from([1.0, 0.0, 0.0]);
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
         errstring = format!("convert copy copied {} files", lenmg1);
         colorx = Color::from([0.0, 0.0, 0.0]);
     }
     Ok(Copyx {
            errcolor: colorx,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
enum Error {
    APIError,
    LanguageError,
}
