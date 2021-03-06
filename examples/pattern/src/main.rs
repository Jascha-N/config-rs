extern crate glob;
extern crate config;

use std::path::Path;
use std::collections::HashMap;
use config::*;
use glob::glob;

fn main() {
    // Option 1
    // --------
    // Gather all conf files from conf/ manually
    let settings = Config::default()
        // File::with_name(..) is shorthand for File::from(Path::new(..))
        .merge(File::with_name("conf/00-default.toml"))
        .merge(File::from(Path::new("conf/05-some.yml")))
        .merge(File::from(Path::new("conf/99-extra.json")))
        .unwrap();

    // Print out our settings (as a HashMap)
    println!("\n{:?} \n\n-----------",
             settings.deserialize::<HashMap<String, String>>().unwrap());

    // Option 2
    // --------
    // Gather all conf files from conf/ manually, but put in 1 merge call.
    let settings = Config::default()
        .merge(vec![File::with_name("conf/00-default.toml"),
                    File::from(Path::new("conf/05-some.yml")),
                    File::from(Path::new("conf/99-extra.json"))])
        .unwrap();

    // Print out our settings (as a HashMap)
    println!("\n{:?} \n\n-----------",
             settings.deserialize::<HashMap<String, String>>().unwrap());

    // Option 3
    // --------
    // Gather all conf files from conf/ using glob and put in 1 merge call.
    let settings = Config::default()
        .merge(glob("conf/*")
                   .unwrap()
                   .map(|path| File::from(path.unwrap()))
                   .collect::<Vec<_>>())
        .unwrap();

    // Print out our settings (as a HashMap)
    println!("\n{:?} \n\n-----------",
             settings.deserialize::<HashMap<String, String>>().unwrap());
}
