use rfd::FileDialog;
use std::path::{Path};
pub fn diroutpressx (dirval: String) -> (u32, String, String) {
     let errcode: u32;
     let errstring: String;
     let mut new_dir: String;
     if Path::new(&dirval).exists() {
         new_dir = dirval.to_string();
     } else {
         new_dir = "/".to_string();
     }
     let folder = FileDialog::new()
//        .set_location(&new_dir)
//        .show_open_single_dir()
//        .unwrap();
         .set_directory(&new_dir)
         .pick_folder();
     if folder == None {
         errstring = "error getting output directory -- possible cancel key hit".to_string();
         errcode = 1;
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         errstring = "convert output directory selected".to_string();
         errcode = 0;
     } 
     (errcode, errstring, new_dir)
}

