extern crate exif;
extern crate chrono;

// use gtk::prelude::*;
// use std::io::BufReader;
// use exif::{Reader, In, Tag};
// use std::fs;
// use chrono::prelude::*;

// use std::fs::File;
use std::path::{PathBuf};

use crate::parse_moddate::parse_moddate;
use crate::get_strvector::get_strvector;

pub fn create_mergelist (str_cur_dir1: String, str_filesize: String, str_datemod: String) -> (u32, String, String) {
    let mut errcode: u32 = 0;
    let mut errstring: String = " ".to_string();
    let mut dateyr1 = 0;
    let mut datemo1 = 0;
    let mut dateday1 = 0;
    let mut datehr1 = 0;
    let mut datemin1 = 0;
    let mut datesec1 = 0;
    let mut filesize_int: i32 = 0;
    let mut new_mergelist: String = " ".to_string();

    let mut bolok = true;

// see if filesize exists and is between 4 and 16
    if bolok {
        filesize_int = str_filesize.parse().unwrap_or(-99);
        if filesize_int > 0 {
            if (filesize_int < 4) | (filesize_int > 16) {
                errstring = "********* convert Merge: Invalid file length. Must be between 4 and 16 *********".to_string();
                errcode = 1;
                bolok = false;
            }
        } else if filesize_int == -99 {
            errstring = "********* convert Merge: Files length is not an integer **********".to_string();
            errcode = 2;
            bolok = false;
        } else {
            errstring = "********* convert Merge: File length is not positive integer **********".to_string();
            errcode = 3;
            bolok = false;
        }
    }
// validate date mod 
        if bolok {
            let datemod1_text = str_datemod;
            let (baddate1, dateyr, datemo, dateday, datehr, datemin, datesec) = parse_moddate(datemod1_text.to_string());
            if baddate1 != 0 {
                errstring = "********* convert Merge: Date Mod 1 is not formatted correctly **********".to_string();
                errcode = 4;
                bolok = false;
            } else {
                dateyr1 = dateyr;
                datemo1 = datemo;
                dateday1 = dateday;
                datehr1 = datehr;
                datemin1 = datemin;
                datesec1 = datesec;
            }
        }
        if bolok {
            let current_dir = PathBuf::from(&str_cur_dir1);
            let (errcd1, errstr1, newvector1) = get_strvector(current_dir, 1, filesize_int, false, dateyr1, dateday1, datemo1, datehr1, datemin1, datesec1);
            let mut newvectormut = newvector1;
            let mut chgseq2 = false;
            if errcd1 != 0 {
                errstring = errstr1;
                errcode = 5;
                bolok = false;
            } else {
                newvectormut.sort();
                let newvectormutlen = newvectormut.len();
                let newtoi = newvectormutlen as i32 ;
                if newtoi > 1 {
                    let mut chgx = true;
                    while chgx {
                       let mut listitems: Vec<String> = Vec::new();
                       chgx = false;
                       for indexi in 1..newtoi {
                            let strinput1split: Vec<&str> = newvectormut[(indexi - 1) as usize].split("|").collect();
                            let strinputsplit: Vec<&str> = newvectormut[indexi as usize].split("|").collect();
                            let mut file_prefixdate1;
                            let file_prefixdate2;
                            if chgseq2 {
                                chgseq2 = false; 
                                let prefix1: String = strinput1split[0][0..19].parse().unwrap();
                                let mut seq2_int: i32 = strinput1split[0][20..].parse().unwrap_or(-9999);
                                if seq2_int == -9999 {
                                    bolok = false;
                                    chgx = false;
                                    errstring = "********* convert Merge: seq number not numeric **********".to_string();
                                    errcode = 6;
                                    break;
                                } else {
                                    if seq2_int < 999 {
                                        seq2_int = seq2_int + 1;
                                        chgx = true;
                                    }
                                    file_prefixdate1 = format!("{}_{:03}", prefix1, seq2_int);
                                }
                            } else {
                                file_prefixdate1 = format!("{}", strinput1split[0]);
                            }
                            file_prefixdate2 = format!("{}", strinputsplit[0]);
                            if file_prefixdate1 == file_prefixdate2 {
                                chgseq2 = true;
                                let prefix1: String = strinput1split[0][0..19].parse().unwrap();
                                let mut seq1_int: i32 = strinput1split[0][20..].parse().unwrap_or(-9999);
                                if seq1_int == -9999 {
                                    bolok = false;
                                    chgx = false;
                                    errstring = "********* convert Merge: seq number not numeric **********".to_string();
                                    errcode = 7;
                                    break;
                                } else {
                                    if seq1_int > 0 {
                                        seq1_int = seq1_int - 1;
                                        chgx = true;
                                    }
                                }    
                                file_prefixdate1 = format!("{}_{:03}", prefix1, seq1_int);
                            }
                            let stroutput = format!("{}|{}|{}|{}|{}", file_prefixdate1, strinput1split[1], strinput1split[2], strinput1split[3], strinput1split[4]);
                            listitems.push(stroutput);
                       }

                       let current_dira = PathBuf::from(&str_cur_dir1);
                       let (errcda, errstra, newvectora) = get_strvector(current_dira, 1, filesize_int, false, dateyr1, dateday1, datemo1, datehr1, datemin1, datesec1);
                       let mut newvectormuta = newvectora;
                       if errcda != 0 {
                           errstring = errstra;
                           errcode = 8;
                           bolok = false;
                       } else {
                           newvectormuta.sort();
                           let strinputxsplit: Vec<&str> = newvectormuta[(newtoi - 1) as usize].split("|").collect();
                           let file_prefixdatex;
                           if chgseq2 {
                               chgseq2 = false; 
                               let prefixx: String = strinputxsplit[0][0..19].parse().unwrap();
                               let mut seqx_int: i32 = strinputxsplit[0][20..].parse().unwrap_or(-9999);
                               if seqx_int == -9999 {
                                   bolok = false;
                                   errstring = "********* convert Merge: seq number not numeric **********".to_string();
                                   errcode = 10;
                                   break;
                               } else {
                                   if seqx_int < 999 {
                                       seqx_int = seqx_int + 1;
                                       chgx = true;
                                   }
                                   file_prefixdatex = format!("{}_{:03}", prefixx, seqx_int);
                               }
                           } else {
                               file_prefixdatex = format!("{}", strinputxsplit[0]);
                           }
                           let stroutputx = format!("{}|{}|{}|{}|{}", file_prefixdatex, strinputxsplit[1], strinputxsplit[2], strinputxsplit[3], strinputxsplit[4]);
                           listitems.push(stroutputx);
                           newvectormut = listitems;
                           newvectormut.sort();
                       }
                    }
                }
            }
            if bolok {
                let newvectormutlen = newvectormut.len();
                let newtoi = newvectormutlen as i32 ;
                for indexi in 0..newtoi {
                     let strinputx = &newvectormut[indexi as usize];
                     let strinputspx: Vec<&str>  = strinputx.split("|").collect();
                     let newlinelist = strinputspx[1].to_owned() + " | " + strinputspx[2] + " | " + strinputspx[0] + " | " + strinputspx[3] + " | " + strinputspx[4];
                     new_mergelist = new_mergelist + &newlinelist + "\n ";
                }
                errstring = format!("convert merge merged {} files", newtoi);
                errcode = 0;
            }
        }
        (errcode, errstring, new_mergelist)
 
}

