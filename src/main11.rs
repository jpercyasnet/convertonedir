// use rfd::FileDialog;
use iced::widget::{button, column, row, text_input, text, scrollable, progress_bar};
use iced::{Alignment, Element, Command, Application, Length, Settings, Color, Theme};
use iced::theme;
use iced::executor;
mod get_dirlist;
mod dirpressx;
mod diroutpressx;
mod create_mergelist;
mod parse_moddate;
mod dump_file;
mod get_strvector;
mod mergepressx;
mod copypressx;
mod asynccopy;
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
//    err_value: bool,
//    Loaded: Copyx,
//    Errored: String,
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
//    Copydone(Color, String),
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
//               Loaded: { Copyx { number: 33, name: "name start".to_string(), description: "desc start".to_string() }},
//               Errored: "error value".to_string(),
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
//                   Command::perform(asynccopy::asynccopy(self.dir_value.clone(),self.outdir_value.clone(), self.mergescrol_value.clone()), Message::Copydone(self.mess_color, self.msg_value))
                    Command::perform(Copyx::search(), Message::CopyxFound)

               } else {
                   Command::none()
               }
            }
//            Message::Copydone(valuea, valueb) => {
//               self.msg_value = valueb.to_string();
//               self.mess_color = valuea;
//               Command::none()
//            }
              Message::CopyxFound(Ok(copyx)) => {
//                 let subtitle = match self {
//                   self.Loaded { copyx } => &copyx.name,
//                   Counterx::Errored { .. } => "Whoops!",
//                 };
//                 self.msg_value = subtitle;
                self.msg_value = copyx.name.clone();
//                  self.msg_value = "copy completed".to_string();
                Command::none()
             }
             Message::CopyxFound(Err(_error)) => {
                self.msg_value = "error in copyx routine".to_string();

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
    number: u16,
    name: String,
    description: String,
//    image: image::Handle,
}

impl Copyx {
//    const TOTAL: u16 = 807;

    async fn search() -> Result<Copyx, Error> {
//        use rand::Rng;
//        use serde::Deserialize;

//        #[derive(Debug, Deserialize)]
//        struct Entry {
//            name: String,
//            flavor_text_entries: Vec<FlavorText>,
//        }

//        #[derive(Debug, Deserialize)]
 //       struct FlavorText {
 //           flavor_text: String,
 //           language: Language,
 //       }

 //       #[derive(Debug, Deserialize)]
 //       struct Language {
 //           name: String,
 //       }
          let id = 33;
          let name = "value name".to_string();
          let description = "value name".to_string();
//       let id = {
  //          let mut rng = rand::rngs::OsRng::default();

 //          rng.gen_range(0, Pokemon::TOTAL)
  //      };

 //       let fetch_entry = async {
 //           let url =
 //               format!("https://pokeapi.co/api/v2/pokemon-species/{}", id);

 //           reqwest::get(&url).await?.json().await
 //       };

 //       let (entry, image): (Entry, _) =
 //           futures::future::try_join(fetch_entry, Self::fetch_image(id))
 //               .await?;

 //       let description = entry
 //           .flavor_text_entries
 //           .iter()
  //          .find(|text| text.language.name == "en")
  //          .ok_or(Error::LanguageError)?;

        Ok(Copyx {
            number: id,
//            name: entry.name.to_uppercase(),
            name: name,
            description: description,
//                .flavor_text
//                .chars()
//                .map(|c| if c.is_control() { ' ' } else { c })
//                .collect(),
//            image,
        })
    }
}
#[derive(Debug, Clone)]
enum Error {
    APIError,
    LanguageError,
}
