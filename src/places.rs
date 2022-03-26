use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::path::Path;
use xdgkit::basedir;
use xdgkit::icon_theme::IconTheme;
use xdgkit::icon_finder::{multiple_find_icon, generate_dir_list, user_theme, DirList};
//TODO user_dirs
use crate::menu::{Menu, Item};

fn icon_getter(name:&str, size:i32, scale:i32, dir_list_vector:Vec<DirList>, theme:IconTheme) -> String {
    return name.to_string();
    let icon = match multiple_find_icon(name.to_string(), size, scale, dir_list_vector.clone(), theme.clone()) {
        Some(i) => i,
        None => return "".to_string(),
    };
    let icon = match std::fs::canonicalize(icon) {
        Ok(i) => i,
        Err(_) => return "".to_string(),
    };
    let icon = match icon.to_str() {
        Some(i) => i,
        None => return "".to_string(),
    };
    format!("{}", icon)
    //name.to_string()
}

/// get the home directory
fn home_directory() -> String {
    match basedir::home() {
        Err(_) => "".to_string(),
        Ok(h) => h,
    }
}

/// get the cache directory
fn cache_directory() -> String {
    match basedir::cache_home() {
        Ok(ch) => ch,
        Err(_) => {
            let home = home_directory();
            format!("{}/.cache", home.as_str())
        },
    }
}

/// get the cache filename
fn cache_filename() -> String {
    let cache_dir = cache_directory();
    format!("{}/places-menu", cache_dir.as_str())
}

/// check if the home directory exists
fn init() -> u32 {
    match basedir::home()  {
        Err(_) => 1,
        Ok(_) => 0,
    }
}

/// recursively make directory
fn make_dir<P: AsRef<Path>>(dir: P) -> u32 where P: std::fmt::Display {
    let d = dir.to_string().clone();
    let dir_path = Path::new(d.as_str());
    if dir_path.is_dir() {
        return 0;
    }
    match std::fs::create_dir_all(dir) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

/// Get the name of the directory
fn get_dir_name<P: AsRef<Path>>(dir: P) -> String  where P: std::fmt::Display{
    //TODO
    let d = dir.to_string().clone();
    let dir_path = Path::new(d.as_str());
    match dir_path.file_name() {
        Some(dir_path) => match dir_path.to_str() {
            Some(dir_path) => dir_path.to_string(),
            None => d.to_string(),
        },
        None => d.to_string(),
    }
}

/// program line
fn program_line(icon:&str, label:&str, dir:&str) -> String {
    let label = fix_and(label.to_string());
    let dir = fix_dir_output(dir.to_string());
    format!("        <Program icon=\"{}\" label=\"{}\">xdg-open {}</Program>\n", icon, label.as_str(), dir.as_str())
}

/// program item for menu
fn program_item(icon:&str, label:&str, dir:&str) -> Item {
    let label = fix_and(label.to_string());
    let dir = format!("xdg-open {}", fix_dir_output(dir.to_string()));
    Item::new(label.as_str(), dir.as_str(), icon)
}

// Menu line
fn menu_line(icon:&str, label:&str) -> String {
    let label = fix_and(label.to_string());
    format!("    <Menu label=\"{}\" icon=\"{}\" height=\"0\">\n", label.as_str(), icon)
}

/// The big menu
fn big_menu<P: AsRef<Path>>(dir: P, icon:&str, label:&str) -> String where P: std::fmt::Display {
    //let folder_icon = "folder";
    //let open_icon = "folder-open";
    let dir = dir.to_string();
    let return_string:String;
    if label == "Home" || label == "Trash" {
        return_string = program_line(icon, label, dir.as_str());
    } else {
       let menu = menu_line(icon, label);
       let mut current_icon = icon.to_string();
       if current_icon == "folder" {
            current_icon = "folder-open".to_string();
       }
       let current_folder = program_line(current_icon.as_str(), label, dir.as_str());
       let contents = sub_menu(dir, false);
       return_string = format!("{}{}{}    </Menu>", menu, current_folder, contents);
    }
    return_string
}
/// The big menu
fn make_menu<P: AsRef<Path>>(dir: P, icon:&str, label:&str) -> Menu where P: std::fmt::Display {
    let mut menu = Menu::default();
    let dir = dir.to_string();
    let return_string:String;
    if label == "Home" || label == "Trash" {
        menu.items.push(Item::new(icon, label, dir.as_str()));
    } else {
        menu.name = label.to_string();
        menu.icon = icon.to_string();
        menu.tooltip = label.to_string();
        let mut current_icon = icon.to_string();
        if current_icon == "folder" {
            current_icon = "folder-open".to_string();
        }
        let current_folder = Item::new(current_icon.as_str(), label, dir.as_str());
        let contents = sub_menu_menu(dir, false);
        menu.items.push(current_folder);
        if !contents.name.is_empty() && !contents.icon.is_empty() {
            menu.menus.push(contents.clone());
        }
    }
    menu
}

// fix & to &amp;
fn fix_and(line:String) -> String {
    line.replace("&", "&amp;")
}


/// Fix directory spaces for output
fn fix_dir_output(dir:String) -> String{
    //TODO
    let dir = dir.replace(" ", "\\ ");
    fix_and(dir)
}

/// make the sub menu
fn sub_menu_menu<P: AsRef<Path>>(dir: P, subdirectories:bool) -> Menu where P: std::fmt::Display {
    // setup to check if directory exists as a directory
    let d = dir.to_string().clone();
    let dir_path = Path::new(d.as_str());
    let mut menu = Menu::default();
    // not a directory
    if !dir_path.is_dir() {
        return menu;
    }
    // make sure we can read the directory
    let dir_path = match std::fs::read_dir(dir_path) {
        Ok(dir_path) => dir_path,
        Err(_) => return menu,
    };
    // setup the string to return
    let mut return_string = String::new();
    // loook through the directories
    for entry in dir_path.flatten() {
        let path = entry.path();
        // if we encounter a sub-directory we can use it
        if path.is_dir() {
            // get a useable path
            let dir = match path.to_str() {
                Some(dir) => dir,
                None => continue,
            };
            // truncate the directory name to just the end

            let label = get_dir_name(dir.to_string());
            let dir = dir.to_string();
            // are we looking in the subdirectories?
            if subdirectories {
                // add from the big memu
                let sub = make_menu(dir, "folder", label.as_str());
                if !sub.name.is_empty() && !sub.icon.is_empty() {
                    menu.menus.push(sub);
                }
            } else {
                // just give a single program item
                menu.items.push(
                    Item::new("folder", label.as_str(), dir.as_str())
                );
            }
        }
    }
    menu
}
/// make the sub menu
fn sub_menu<P: AsRef<Path>>(dir: P, subdirectories:bool) -> String where P: std::fmt::Display {
    // setup to check if directory exists as a directory
    let d = dir.to_string().clone();
    let dir_path = Path::new(d.as_str());
    // not a directory
    if !dir_path.is_dir() {
        return "".to_string();
    }
    // make sure we can read the directory
    let dir_path = match std::fs::read_dir(dir_path) {
        Ok(dir_path) => dir_path,
        Err(_) => return "".to_string(),
    };
    // setup the string to return
    let mut return_string = String::new();
    // loook through the directories
    for entry in dir_path.flatten() {
        let path = entry.path();
        // if we encounter a sub-directory we can use it
        if path.is_dir() {
            // get a useable path
            let dir = match path.to_str() {
                Some(dir) => dir,
                None => continue,
            };
            // truncate the directory name to just the end

            let label = get_dir_name(dir.to_string());
            let dir = dir.to_string();
            // are we looking in the subdirectories?
            if subdirectories {
                // add from the big memu
                return_string.push_str(
                    big_menu(dir, "folder", label.as_str())
                        .as_str());
            } else {
                // just give a single program line
                return_string.push_str(
                    program_line("folder", label.as_str(), dir.as_str())
                        .as_str());
            }
        }
    }
    return_string
}

pub fn gtk_bookmarks_menu() -> Menu {
    let bookmarks_dir = format!("{}/.config/gtk-3.0", home_directory().as_str());
    let mut menu = Menu::default();
    // check to see if the directory exists
    let dir_path = Path::new(bookmarks_dir.as_str());
    if !dir_path.is_dir() {
        // nothing
        return menu;
    }
    // get the bookmarks file
    let bookmarks = format!("{}/bookmarks", bookmarks_dir);
    // try to open the file
    let file = match File::open(bookmarks.as_str()) {
        Ok(f) => f,
        Err(_) => return menu,
    };
    let file_reader = BufReader::new(file);
    for (_line_number, line) in file_reader.lines().enumerate() {
        if line.is_err() {
            continue;
        }
        let line = line.unwrap();
        if let Some((dir, name)) = line.rsplit_once(' ') {
            if let Some((handler, dir)) = dir.split_once("://") {
                if handler == "sftp" {
                    menu.items.push(
                        Item::new("folder-remote", name, dir)
                    );
                    continue;
                }
                let icon = get_icon(dir);
                let sub = make_menu(dir, icon.as_str(), name);
                if !sub.name.is_empty() && !sub.icon.is_empty() {
                    menu.menus.push(sub);
                }
            }
        }
    }
    menu
}

/// The gtk bookmarks
fn gtk_bookmarks() -> String {
    let mut return_string:String = String::new();
    let bookmarks_dir = format!("{}/.config/gtk-3.0", home_directory().as_str());
    // check to see if the directory exists
    let dir_path = Path::new(bookmarks_dir.as_str());
    if !dir_path.is_dir() {
        // nothing
        return return_string;
    }
    // get the bookmarks file
    let bookmarks = format!("{}/bookmarks", bookmarks_dir);
    // try to open the file
    let file = match File::open(bookmarks.as_str()) {
        Ok(f) => f,
        Err(_) => return return_string,
    };
    let file_reader = BufReader::new(file);
    for (_line_number, line) in file_reader.lines().enumerate() {
        if line.is_err() {
            continue;
        }
        let line = line.unwrap();
        if let Some((dir, name)) = line.rsplit_once(' ') {
            if let Some((handler, dir)) = dir.split_once("://") {
                if handler == "sftp" {
                    return_string.push_str(
                        program_line("folder-remote", name, dir).as_str()
                    );
                    return_string.push('\n');
                    continue;
                }
                let icon = get_icon(dir);
                return_string.push_str(
                    big_menu(dir, icon.as_str(), name).as_str()
                );
                return_string.push('\n');
            }
        }
    }
    return_string
}
fn get_icon(directory:&str) -> String {
    let dir = get_dir_name(directory);
    // icons
    let home_icon = "user-home".to_string();
    let doc_icon = "folder-documents".to_string();
    let dl_icon = "folder-download".to_string();
    let music_icon = "folder-music".to_string();
    let pic_icon = "folder-pictures".to_string();
    let vid_icon = "folder-videos".to_string();
    // names
    let home = "Home";
    let doc = "Documents";
    let dl = "Downloads";
    let music = "Music";
    let pic = "Pictures";
    let vid = "Videos";
    if dir == home {
        return home_icon;
    } else if dir == doc {
        return doc_icon;
    } else if dir == dl {
        return dl_icon;
    } else if dir == music {
        return music_icon;
    } else if dir == vid {
        return vid_icon;
    } else if dir == pic {
        return pic_icon;
    }
    // default
    "folder".to_string()
}

/// JWM places menu
pub fn jwm() -> u32 {
    // see if we have a home
    if init() > 0 {
        return 1;
    }
    // see if we have a cache
    let cache = cache_directory();
    if make_dir(cache) > 0 {
        return 2;
    }
    // get our file to cache in
    let cache_file = cache_filename();
    // icons
    let home_icon = "user-home";
    let doc_icon = "folder-documents";
    let dl_icon = "folder-download";
    let music_icon = "folder-music";
    let pic_icon = "folder-pictures";
    let vid_icon = "folder-videos";
    let rubbish_icon = "user-trash";
    //let media_icon = "file-manager";
    // names
    let home = "Home";
    let doc = "Documents";
    let dl = "Downloads";
    let music = "Music";
    let pic = "Pictures";
    let vid = "Videos";
    let rubbish = "Trash";
    // names
    let home_dir = home_directory();
    let doc_dir = format!("{}/{}", home_dir.as_str(), home);
    if make_dir(doc_dir.clone()) > 0 {
        return 3;
    }
    let dl_dir = format!("{}/{}", home_dir.as_str(), dl);
    if make_dir(dl_dir.clone()) > 0 {
        return 4;
    }
    let music_dir = format!("{}/{}", home_dir.as_str(), music);
    if make_dir(music_dir.clone()) > 0 {
        return 5;
    }
    let pic_dir = format!("{}/{}", home_dir.as_str(), pic);
    if make_dir(pic_dir.clone()) > 0 {
        return 6;
    }
    let vid_dir = format!("{}/{}", home_dir.as_str(), vid);
    if make_dir(vid_dir.clone()) > 0 {
        return 7;
    }
    let rubbish_dir = format!("{}/.local/share/Trash/files", home_dir.as_str());
    if make_dir(rubbish_dir.clone()) > 0 {
        return 8;
    }
    // make the file
    let reload = "\n   <Program icon=\"reload\" label=\"Update Menus\">jwm -reload</Program>";
    let full_menu = format!("<JWM>\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n</JWM>",
                big_menu(home_dir.clone(), home_icon, home),
                big_menu(doc_dir, doc_icon, doc),
                big_menu(dl_dir, dl_icon, dl),
                big_menu(music_dir, music_icon, music),
                big_menu(pic_dir, pic_icon, pic),
                big_menu(vid_dir, vid_icon, vid),
                big_menu(rubbish_dir.to_owned(), rubbish_icon, rubbish),
                reload);
    let gtk_bookmarks = gtk_bookmarks();
    let gtk_menu = format!("<JWM>\n{}\n{}{}{}\n</JWM>",
                big_menu(home_dir, home_icon, home),
                gtk_bookmarks,
                big_menu(rubbish_dir, rubbish_icon, rubbish),
                reload);
    let mut file:File = match File::create(cache_file.as_str()) {
        Ok(file) => file,
        Err(_) => return 9,
    };
    if gtk_bookmarks.is_empty() {
        println!("{}", full_menu.as_str());
        match std::fs::File::write(&mut file,
                                    &full_menu.into_bytes()) {
            Err(e) => {
                println!("Error:{}", e);
                return 10;
            },
            Ok(_) => return 0,
        };
    } else {
        println!("{}", gtk_menu.as_str());
        match std::fs::File::write(&mut file,
                                    &gtk_menu.into_bytes()) {
            Err(e) => {
                println!("Error:{}", e);
                return 11;
            },
            Ok(_) => return 0,
        };
    }
}
