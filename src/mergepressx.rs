use crate::create_mergelist;
use std::path::Path;

pub fn mergepressx (dir_value: String, size_value: String, hhmmss_value: String) -> (u32, String, String) {
     let errcode: u32;
     let errstring: String;
     let mut new_dirlist: String = " ".to_string();
     if Path::new(&dir_value).exists() {
         let (errcd, errstr, newliststr) = create_mergelist(dir_value, size_value, hhmmss_value);
         if errcd == 0 {
             new_dirlist  = newliststr;
             errstring = "created merge list".to_string();
             errcode = 0;
         } else {
             errstring = errstr.to_string();
             errcode = 1;
         }
     } else {
         errstring = "the directory does not exist".to_string();
         errcode = 2;
     }
    (errcode, errstring, new_dirlist)
}

