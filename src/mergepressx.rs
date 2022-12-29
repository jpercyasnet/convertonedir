use iced::Color;
use crate::create_mergelist;
use std::path::Path;

pub fn mergepressx (dir_value: String, size_value: String, hhmmss_value: String) -> (Color, String, String) {
     let errstring: String;
     let mut new_dirlist: String = " ".to_string();
     let colorx: Color;
     if Path::new(&dir_value).exists() {
         let (errcd, errstr, newliststr) = create_mergelist(dir_value, size_value, hhmmss_value);
         if errcd == 0 {
             new_dirlist  = newliststr;
             errstring = "created merge list".to_string();
             colorx = Color::from([0.0, 0.0, 0.0]);
         } else {
             errstring = errstr.to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
         }
     } else {
         errstring = "the directory does not exist".to_string();
         colorx = Color::from([1.0, 0.0, 0.0]);
     }
    (colorx, errstring, new_dirlist)
}

