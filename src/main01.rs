use iced::widget::{button, column, row, text_input, text, scrollable};
use iced::{Alignment, Element, Task, Length, Color, Theme};

use std::process::Command as stdCommand;
use std::path::Path;

mod get_dirlist;
mod dirpressx;
mod diroutpressx;
mod create_mergelist;
mod parse_moddate;
mod dump_file;
mod get_strvector;
mod mergepressx;
mod copypressx;
mod get_winsize;

use get_dirlist::get_dirlist;
use dirpressx::dirpressx;
use diroutpressx::diroutpressx;
use mergepressx::mergepressx;
use copypressx::copypressx;
use create_mergelist::create_mergelist;
use get_winsize::get_winsize;

pub fn main() -> iced::Result {

     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }
     iced::application(Convert1dir::title, Convert1dir::update, Convert1dir::view)
        .window_size((widthxx, heightxx))
        .theme(Convert1dir::theme)
        .run_with(Convert1dir::new)

}

struct Convert1dir {
    dir_value: String,
    mess_color: Color,
    msg_value: String,
    scrol_value: String,
    hhmmss_value: String,
    size_value: String,
    outdir_value: String,
    mergescrol_value: String,
    scrolheight: f32,

}

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

impl Convert1dir {
    fn new() -> (Self, Task<Message>) {
    
        let mut heightxx: f32 = 175.0;
        let (errcode, errstring, _widtho, heighto) = get_winsize();
        if errcode == 0 {
            heightxx = 175.0 + ((heighto as f32 - 768.0) / 2.0);
            println!("{}", errstring);
        } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
        }
        ( Self { dir_value: "no directory".to_string(), msg_value: "no message".to_string(), hhmmss_value: "-00:00:00:00:00:00".to_string(), 
               size_value: "10".to_string(), mess_color: Color::from([0.0, 0.0, 1.0]), outdir_value: "no directory".to_string(), 
               scrol_value: " No directory selected \n \
                            ".to_string(),
               mergescrol_value: " No directory selected \n \
                            ".to_string(), scrolheight: heightxx,
          },
          Task::none()
        )
    }

    fn title(&self) -> String {
        String::from("Convert one directory -- iced")
    }

    fn update(&mut self, message: Message) -> Task<Message>  {
        match message {
            Message::DirPressed => {
               let (errcode, errstr, newdir, newliststr) = dirpressx(self.dir_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.scrol_value  = newliststr.to_string();
                   self.dir_value = newdir.to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Task::none()
            }
            Message::HhmmssChanged(value) => { self.hhmmss_value = value; Task::none() }
            Message::SizeChanged(value) => { self.size_value = value; Task::none() }
            Message::OutDirPressed => {
               let (errcode, errstr, newdir) = diroutpressx(self.dir_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.outdir_value = newdir.to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Task::none()
            }
            Message::MergePressed => {
               let (errcode, errstr, newliststr) = mergepressx(self.dir_value.clone(),self.size_value.clone(), self.hhmmss_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mergescrol_value  = newliststr;
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Task::none()
            }
            Message::CopyPressed => {
               let (errcode, errstr) = copypressx(self.dir_value.clone(),self.outdir_value.clone(), self.mergescrol_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   Task::perform(Copyx::copyit(self.dir_value.clone(),self.outdir_value.clone(), self.mergescrol_value.clone()), Message::CopyxFound)

               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Task::none()
               }
            }
              Message::CopyxFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                Task::none()
             }
             Message::CopyxFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                Task::none()
             }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(20),
                 text(&self.msg_value).color(*&self.mess_color).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![button("Directory Button").on_press(Message::DirPressed),
                 text(&self.dir_value).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            scrollable(
                column![
                        text(format!("{}",&self.scrol_value))
                ].width(Length::Fill),
            ).height(Length::Fixed(self.scrolheight)),
            row![text("date mod value (-YY:MM:DD:hh:mm:ss): ").size(20),

                 text_input("No input....", &self.hhmmss_value)
                            .on_input(Message::HhmmssChanged).padding(10).size(20),
                 text("     Length of File Description: "),
                 text_input("No input....", &self.size_value).on_input(Message::SizeChanged).padding(10).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![button("outDirectory Button").on_press(Message::OutDirPressed),
                 text(&self.outdir_value).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            scrollable(
                column![
                        text(format!("{}",&self.mergescrol_value))
                ].width(Length::Fill),
            ).height(Length::Fixed(self.scrolheight)),
            row![button("Merge Button").on_press(Message::MergePressed),
                 button("Copy Button").on_press(Message::CopyPressed),
            ].align_y(Alignment::Center).spacing(400).padding(30),
         ]
        .padding(5)
        .align_x(Alignment::Start)
        .into()
    }
    
    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}
#[derive(Debug, Clone)]
struct Copyx {
    errcolor: Color,
    errval: String,
}

impl Copyx {
    async fn copyit(dir_value: String, outdir_value: String, mergescrol_value: String) -> Result<Copyx, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 1.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl];
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[1].to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx;
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          let str_cur_dirout = outdir_value.clone();
          let fileprex = lineparse[2].to_string();
          let filetox = lineparse[3].to_string();
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

          numrow = numrow + 1;
     }
     if bolok {
         errstring = format!("convert copy copied {} files", lenmg1);
         colorx = Color::from([0.0, 1.0, 0.0]);
     }
     Ok(Copyx {
            errcolor: colorx,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
enum Error {
//    APIError,
//    LanguageError,
}
