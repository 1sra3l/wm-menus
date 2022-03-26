use xdgkit::categories::Categories;
use xdgkit::icon_theme::IconTheme;
use xdgkit::icon_finder::{multiple_find_icon, generate_dir_list, user_theme, DirList};
use xdgkit::basedir::{applications, convert_to_vec, cache_home, config_home, home};
use xdgkit::desktop_entry::DesktopEntry;
use std::path::Path;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use crate::menu::{Menu, Item};

fn normal_categories() -> Vec<Categories> {
    vec![
        Categories::AudioVideo,
        Categories::Education,
        Categories::Development,
        Categories::Game,
        Categories::Graphics,
        Categories::Network,
        Categories::Office,
        Categories::Science,
        Categories::Settings,
        Categories::System,
        Categories::Utility,
    ]
}
fn icon_getter(name:&str, size:i32, scale:i32, dir_list_vector:Vec<DirList>, theme:IconTheme) -> String {
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
fn icon(category:Categories, size:i32, scale:i32, dir_list_vector:Vec<DirList>, theme:IconTheme) -> String {
    let mut cat_name = category.to_string().to_lowercase();
    cat_name.retain(|c| c != '"');
    if cat_name == "game" {
        cat_name = "games".to_string();
    } else if cat_name == "utility" {
        cat_name = "accessories".to_string();
    } else if cat_name == "settings" {
        cat_name = "utilities".to_string();
    } else if cat_name == "audiovideo" {
        cat_name = "multimedia".to_string();
    } else if cat_name == "education" {
        cat_name = "engineering".to_string();
    } else if cat_name == "network" {
        cat_name = "internet".to_string();
    }
    let cat = format!("applications-{}", cat_name.as_str());
    let icon = icon_getter(cat.as_str(), size, scale, dir_list_vector.clone(), theme.clone());
    if icon.is_empty() {
        return icon_getter("applications-other", size, scale, dir_list_vector.clone(), theme.clone());
    }
    icon
}
pub struct AppMenu {
    pub name:String,
    pub icon:String,
    pub menus:Vec<Menu>,
}
impl AppMenu {
    /// create the app menu items sorted by category
    pub fn make_menu(size:i32, scale:i32) -> Vec<Menu> {
        let appdirs =  convert_to_vec(applications());
        let mut menus:Vec<Menu> = vec![];
        let dir_list_vector = generate_dir_list();
        let mut theme = user_theme(dir_list_vector.clone());
        if theme.is_none() {
            //println!("No user theme");
            theme = Some(IconTheme::empty());
        }
        let theme:IconTheme = theme.unwrap();
        for category in normal_categories() {
            let mut menu = Menu::empty();
            let mut cat = category.to_string();
            cat.retain(|c| c != '"');
            menu.name = cat.clone();
            menu.tooltip = cat.clone();
            menu.icon = icon(category, size, scale, dir_list_vector.clone(), theme.clone());
            menus.push(menu.clone());
        }

        for dir in appdirs {
            let dir_path = Path::new(dir.as_str());
            if !dir_path.is_dir() {
                continue;
            }
            let dir_path = match std::fs::read_dir(dir_path) {
                Ok(dir_path) => dir_path,
                Err(_) => continue,
            };
            for entry in dir_path.flatten() {
                let path = entry.path();
                if path.is_file() {
                    // get a useable path
                    let file = match path.to_str() {
                        Some(file) => file,
                        None => continue,
                    };
                    let desktop_file = DesktopEntry::new(file.to_string());
                    if desktop_file.name.is_none() {
                        continue;
                    }
                    //println!("entry:{:?}", file.to_string());
                    if let Some(categories) = desktop_file.categories {
                        for cat in categories.clone() {
                            //println!("category:{:?}", cat.clone());
                            let mut cat_name = cat.to_string();
                            cat_name.retain(|c| c != '"');
                            for mut menu in &mut menus {
                                //println!("menu:{:?}", menu.name.clone());
                                if cat_name == menu.name {
                                    let action = match desktop_file.exec.clone() {
                                        Some(e) => e,
                                        None => continue,
                                    };
                                    let name = match desktop_file.name .clone(){
                                        Some(n) => n,
                                        None => action.clone(),
                                    };
                                    let tooltip = match desktop_file.comment.clone() {
                                        Some(c) => c,
                                        None => name.clone(),
                                    };
                                    let mut icon = match desktop_file.icon.clone() {
                                        Some(c) => c,
                                        None => "".to_string(),
                                    };
                                    icon = icon_getter(icon.as_str(),
                                            size,
                                            scale,
                                            dir_list_vector.clone(),
                                            theme.clone());
                                    if icon.is_empty() {
                                        icon = icon_getter(
                                                "application-default-icon",
                                                size,
                                                scale,
                                                dir_list_vector.clone(),
                                                theme.clone());
                                    }
                                    //println!("item:{:?}", name.clone());
                                    let item = Item {
                                        name,
                                        icon,
                                        tooltip,
                                        action,
                                    };
                                    menu.items.push(item.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut return_menus:Vec<Menu> = vec![];
        for menu in menus {
            if !menu.items.is_empty() {
                return_menus.push(menu.clone());
            }
        }
        // return the menu vector
        return_menus
    }
    /// Create a new AppMenu
    pub fn new(name:&str, icon:&str) -> Self {
        let menus = Self::make_menu(48, 1);
        Self {
            name:name.to_string(),
            icon:icon.to_string(),
            menus,
        }
    }
    fn jwm_cache_filename() -> String {
        let dir = match xdgkit::basedir::cache_home() {
            Ok(ch) => ch,
            Err(_)=> "".to_string(),
        };
        format!("{}/apps-menu", dir.as_str())
    }
    fn openbox_cache_filename() -> String {
        let dir = match xdgkit::basedir::config_home() {
            Ok(ch) => ch,
            Err(_)=> "".to_string(),
        };
        format!("{}/openbox/menu.xml", dir.as_str())
    }
    /// make the openbox menu
    pub fn openbox(&self) -> String {
        let mut return_menu:String = String::new();
        let mut return_menu_list:String = String::new();
        let cache_file = Self::openbox_cache_filename();
        let dir_list_vector = generate_dir_list();
        let mut theme = user_theme(dir_list_vector.clone());
        if theme.is_none() {
            //println!("No user theme");
            theme = Some(IconTheme::empty());
        }
        let theme:IconTheme = theme.unwrap();
        for menu in &mut self.menus.clone() {
            //return_menu.push_str(format!("<!--{}-->", menu.icon.as_str()).as_str());
            return_menu.push_str(menu.openbox().as_str());
            let id = format!("\t<menu id=\"{}-id\" />\n", menu.name.clone().as_str());
            return_menu_list.push_str(id.as_str());
        }
        let exit = format!("\t<separator />\n\t<item label=\"Log Out\">\n\t\t<action name=\"Exit\">\n\t\t\t<prompt>yes</prompt>\n\t\t</action>\n\t</item>");
        let menu = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<openbox_menu xmlns=\"http://openbox.org/3.4/menu\">\n{}\n<menu id=\"root-menu\" label=\"Openbox\">\n{}\n{}</menu>\n</openbox_menu>", return_menu.as_str(), return_menu_list.as_str(), exit.as_str());
        let mut file:File = match File::create(cache_file.as_str()) {
            Ok(file) => file,
            Err(_) => return menu,
        };
        match std::fs::File::write(&mut file,
                                    &menu.clone().into_bytes()) {
            Err(e) => {
                println!("Error:{}", e);
                return menu;
            },
            Ok(_) => return menu,
        };
        menu

    }
    /// make the JWM menu
    pub fn jwm(&self) -> String {
        let mut return_menu:String = String::new();
        let cache_file = Self::jwm_cache_filename();
        let reload = "\n   <Program icon=\"reload\" label=\"Update Menus\">jwm -reload</Program>";
        for menu in self.menus.clone() {
            return_menu.push_str(menu.jwm().as_str());
        }
        let menu = format!("<JWM>\n{}\n{}\n</JWM>", return_menu.as_str(), reload);
        let mut file:File = match File::create(cache_file.as_str()) {
            Ok(file) => file,
            Err(_) => return menu,
        };
        match std::fs::File::write(&mut file,
                                    &menu.clone().into_bytes()) {
            Err(e) => {
                println!("Error:{}", e);
                return menu;
            },
            Ok(_) => return menu,
        };
        menu
    }
}
