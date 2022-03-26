
/// Menu
#[derive(Debug, Clone)]
pub struct Menu {
    pub name:String,
    pub icon:String,
    pub tooltip:String,
    pub items:Vec<Item>,
    pub menus:Vec<Menu>,
}
impl Default for Menu {
    fn default() -> Self {
        Self::empty()
    }
}
impl Menu {
    /// Empty menu
    pub fn empty() -> Self {
        Self {
            name:String::new(),
            icon:String::new(),
            tooltip:String::new(),
            items:vec![],
            menus:vec![],
        }
    }
    /// make the openbox menu
    pub fn openbox(&self) -> String {
        let mut return_menu:String = format!("<menu id=\"{}-id\" label=\"{}\" icon=\"{}\">\n", self.name.as_str(), self.name.as_str(), self.icon.as_str());
        for item in self.items.clone() {
            return_menu.push_str(item.openbox().as_str());
        }
        return_menu.push_str("</menu>\n");
        return_menu
    }
    /// make the JWM menu
    pub fn jwm(&self) -> String {
        let mut return_menu:String = String::new();
        if self.tooltip.is_empty() {
            return_menu.push_str(
                format!("   <Menu label=\"{}\" icon=\"{}\" height=\"0\">
",
                 self.name.as_str(), self.icon.as_str()).as_str());
        } else {
            return_menu.push_str(
                format!("   <Menu label=\"{}\" icon=\"{}\" tooltip=\"{}\" height=\"0\">
",
                self.name.as_str(),
                self.icon.as_str(),
                self.tooltip.as_str()).as_str());
        }
        for menu in self.menus.clone() {
            return_menu.push_str(menu.jwm().as_str());
        }
        for item in self.items.clone() {
            return_menu.push_str(item.jwm().as_str());
        }
        return_menu.push_str("
    </Menu>
");
        return_menu
    }
}

/// Menu item
#[derive(Debug, Clone)]
pub struct Item {
    pub name:String,
    pub action:String,
    pub icon:String,
    pub tooltip:String,
}

impl Item {
    pub fn new(icon:&str, name:&str, action:&str) -> Self {
        Self {
            name:name.to_string(),
            action:action.to_string(),
            icon:icon.to_string(),
            tooltip:String::new(),
        }
    }
    /// make the openbox item
    pub fn openbox(&self) -> String {
        let mut action = self.action.clone();
        let position = match action.rfind(" %"){
            Some(pos) => {
                let _excess = action.split_off(pos);
                pos
            },
            None => 0,
        };
        format!("\t<item label=\"{}\" icon=\"{}\">
\t\t<action name=\"Execute\">
\t\t\t<command>{}</command>
\t\t\t<startupnotify>
\t\t\t\t<enabled>yes</enabled>
\t\t\t</startupnotify>
\t\t</action>
\t</item>\n", self.name.as_str(), self.icon.as_str(), action.as_str())
    }
    /// make the JWM item
    pub fn jwm(&self) -> String {
        let mut action = self.action.clone();
        let position = match action.rfind(" %"){
            Some(pos) => {
                let _excess = action.split_off(pos);
                pos
            },
            None => 0,
        };
        if self.tooltip.is_empty() {
            return format!("        <Program icon=\"{}\" label=\"{}\">{}</Program>
",
                self.icon.as_str(), self.name.as_str(), action.as_str())
        }
        format!("      <Program icon=\"{}\" label=\"{}\" tooltip=\"{}\">{}</Program>
",
                self.icon.as_str(),
                self.name.as_str(),
                self.tooltip.as_str(),
                action.as_str())
    }
}
