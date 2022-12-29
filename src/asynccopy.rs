extern crate exif;
extern crate chrono;
use iced::{Color, Renderer};
use iced::widget::{ProgressBar};
use std::path::{Path};
use std::process::Command as stdCommand;
pub async fn asynccopy (dir_value: String, outdir_value: String, mergescrol_value: String) -> (Color, String) {
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
     (colorx, errstring)
}
