mod places;
mod menu;
mod apps;
use std::env;
fn usage() {
    println!("jwm-menus [OPTIONS]
    -p || --places || places      Show a places menu
    -a || --apps || apps          Show an application menu
");
}
fn main() {
    let args:Vec<_> = env::args_os().collect();
    if args.len() > 1 {
        for arg in 1..args.len() {
            let check = args[arg].clone();
            if check == "-p" || check == "--places" || check == "places"  {
                places::jwm();
                return;
            } else if check == "-a" || check == "--apps" || check == "apps"  {
                let apps = apps::AppMenu::new("Apps", "");
                let output = apps.jwm();
                println!("{}", output.as_str());
                return;
            } else if check == "-o" || check == "--openbox" || check == "openbox"  {
                let apps = apps::AppMenu::new("Apps", "");
                let output = apps.openbox();
                println!("{}", output.as_str());
                return;
            }
        }
    }
    usage();
}
