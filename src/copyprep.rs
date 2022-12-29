        let mut bolok = true;
        let mut str_cur_dir1 = format!(" ");
        let mut str_cur_dir_o = format!(" ");
//        let mut str_cur_dirfrom = String::new();
        if bolok {
            if let Some(cur_dir1) = cdirectory1_combobox.active_text() {
                str_cur_dir1 = cur_dir1.to_string();
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* convert COPY: ERROR GETTING FROM DIRECTORY IN COMBOBOX **********</span>");
                bolok = false;
            }
        }

// check if outdirectory has files (must not have files)
        if bolok {
            if let Some(cur_dir_o) = cdirectory_o_combobox.active_text() {
                str_cur_dir_o = cur_dir_o.to_string();
                for entry1 in fs::read_dir(&str_cur_dir_o).unwrap() {
                     let entry = entry1.unwrap();
                     if let Ok(metadata) = entry.metadata() {
                         if let Ok(_file_name) = entry.file_name().into_string() {
                             if metadata.is_file() {
                                 messageval_label.set_markup("<span color=\"#FF000000\">********* convert COPY: OUTPUT DIRECTORY HAS FILES IN IT **********</span>");
                                 bolok = false;
                             }
                         }
                     }
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* convert COPY: ERROR GETTING OUT DIRECTORY IN COMBOBOX  **********</span>");
                bolok = false;
           }
        }
// check if merge files and if so process
        if bolok {
            let mut messvalx = format!(" ");
            let view3model = ctree_view3.model();
            if view3model == None {
                messageval_label.set_markup("<span color=\"#FF000000\">********* convert Copy: ERROR NOTHING IN MERGE LIST **********</span>");
            } else {
                progress_progressbar.set_fraction(0.0);
                while glib::MainContext::pending(&glib::MainContext::default()) {
                    glib::MainContext::iteration(&glib::MainContext::default(),true);
                }
                let view3modeluw = view3model.unwrap();
                let mut valid = true;
                let validval = view3modeluw.iter_first().unwrap();
                let mut numrow = 0;
                let numchildren = view3modeluw.iter_n_children(None);
                let mut numprocess = 0;
                while valid {
                      let str_cur_dirfrom = str_cur_dir1.clone();
                      let filefromval = view3modeluw.get_value(&validval,1).get::<String>();
                      let filefromx = filefromval.unwrap().to_string();
                      let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx;
                      if !Path::new(&fullfrom).exists() {
                          messvalx = format!("<span color=\"#FF000000\">********* convert Copy: ERROR {} does not exist **********</span>", fullfrom);
                          bolok = false;
                          break;
                      }
                      let filepreval = view3modeluw.get_value(&validval,2).get::<String>();
                      let fileprex = filepreval.unwrap().to_string();

                      let filetoval = view3modeluw.get_value(&validval,3).get::<String>();
                      let filetox = filetoval.unwrap().to_string();
                      let fullto = str_cur_dir_o.clone() + "/" + &fileprex + "_" + &filetox;
                      if Path::new(&fullto).exists() {
                          messvalx = format!("<span color=\"#FF000000\">********* convert Copy: ERROR {} already exists **********</span>", fullto);
                          bolok = false;
                          break;
                      }
                      valid = view3modeluw.iter_next(&validval);
                      if valid & (numprocess < 4) {
                          Command::new("cp")
                                  .arg("-p")
                                  .arg(&fullfrom)
                                  .arg(&fullto)
                                  .spawn()
                                  .expect("failed to execute process");
                          numprocess = numprocess + 1;
                      } else {
                          let _output = Command::new("cp")
                                                .arg("-p")
                                                .arg(&fullfrom)
                                                .arg(&fullto)
                                                .output()
                                                .expect("failed to execute process");
                          numprocess = 0;
                     }
                     numrow = numrow + 1;
                     let progressfr: f64 = numrow as f64 / numchildren as f64;
                     progress_progressbar.set_fraction(progressfr);
                     while glib::MainContext::pending(&glib::MainContext::default()) {
                         glib::MainContext::iteration(&glib::MainContext::default(),true);
                     }
                }
                if bolok {
                    messvalx = format!("convert copy copied {} files", numchildren);
                }
                messageval_label.set_text(&messvalx);
            }
        }
    }));

//----------------- convert copy button end -----------------------------------

//-------------------- connects end
