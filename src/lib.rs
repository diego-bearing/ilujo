
use std::collections::HashMap;
use std::os::unix::process::CommandExt;
use std::str::FromStr;
use std::{env, error, fs};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;

pub trait CreateComponent {
    const PLACEHOLDER_NAME: &'static str;

    const TEMPLATES: &'static[(&'static str, &'static str)];
    
    fn create_ui(&self, component_name: &str) -> Result<(), Box<dyn error::Error>>;
}

#[derive(Debug)]
pub struct Config {
    pub current_exe: PathBuf,
    pub current_dir: PathBuf,
    pub app_dir: String,
    pub args: Vec<String>,
    pub debug_mode: bool,
    pub options: HashMap<String, String>,
}

impl Config {
    pub fn build() -> Result<Self, Box<dyn error::Error>> {
        let args: Vec<String> = env::args().collect();
        let current_exe = env::current_exe()?;
        let current_dir = env::current_dir()?;
        let app_dir = env::var("ILUJO_APP_DIR").unwrap_or({
            let home = env::var("HOME").unwrap();

            format!("{home}/.ilujo")
        });

        let debug_mode = env::var("ILUJO_DEBUG_MODE")
            .map(|v| bool::from_str(&v).unwrap_or_default())
            .unwrap_or(false);

        let options = Self::options();

        Ok(Self { 
            args, 
            app_dir, 
            current_exe, 
            current_dir, 
            debug_mode, 
            options,
        })
    }

    fn options() -> HashMap<String, String> {
        let args = env::args();

        args.filter(|arg| arg.starts_with("--"))
            .map(move |arg| arg.split("=")
                .map(|x| x.to_string())
                .collect::<Vec<String>>())
            .filter_map(|s| {
                match (s.get(0), s.get(1)) {
                    (None, _) => None,
                    (_, None) => None,
                    (Some(key), Some(val)) => Some((String::from(key), String::from(val)))
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum CreateTarget {
    UiComponent(String),
}

impl From<&String> for CreateTarget {
    fn from(_value: &String) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum Command {
    Create(CreateTarget),
    OpenAppDir,
    Help,
}

impl Command {
    pub fn build(args: &[String]) -> Result<Self, Box<dyn error::Error>> {
        match args.get(1) {
            None => Ok(Self::Help),

            Some(cmd) => match cmd.as_str() {
                "ui" => match args.get(2) {
                    None => {
                        println!("Command {cmd}: Component name required");
                        std::process::exit(1);
                    },
                    Some(name) => Ok(Self::Create(CreateTarget::UiComponent(name.to_string())))
                },
                "open-dir" => Ok(Self::OpenAppDir),

                "help" => Ok(Self::Help),

                other => {
                    println!("Command `{other}` not found");
                    std::process::exit(1);
                }
            }
        }
    }
}

pub struct CommandProcessor {
    pub config: Config,
}

impl CommandProcessor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn process(&self, command: Command) -> Result<(), Box<dyn error::Error>> {

        if self.config.debug_mode {
            println!("Config: {:#?}", self.config);
            println!("Command: {:?}", command);
        }

        match command {
            Command::Create(target) => match target {
                CreateTarget::UiComponent(component_name) => self.create_ui(&component_name),
            },
            Command::OpenAppDir => {
                let mut cmd = process::Command::new("open");

                cmd.arg(format!("{}", self.config.app_dir));

                cmd.exec();

                Ok(())
            },
            Command::Help => {
                print_help();

                Ok(())
            },
        }
    }
}

impl CreateComponent for CommandProcessor {
    const TEMPLATES: &'static[(&'static str, &'static str)] = &[
        ("component.tmpl", "{{component_name}}.tsx"),
        ("types.tmpl", "{{component_name}}.types.ts"),
        ("stories.tmpl", "{{component_name}}.stories.tsx"),
        ("index.tmpl", "index.ts"),
    ];

    const PLACEHOLDER_NAME: &'static str = "{{component_name}}";


    fn create_ui(&self, component_name: &str) -> Result<(), Box<dyn error::Error>> {
        let Config {current_dir, app_dir, ..} = &self.config;

        let target_dir = self.config.options.get("--dir")
            .map_or(current_dir.clone(), |s| PathBuf::from_str(s)
                .expect("creating path from --dir"));

        let dir_entries = fs::read_dir(&target_dir).expect("should read the target dir");

        // check if dir exist
        dir_entries.into_iter().for_each(|entry| {
            let path = entry.expect("Failed reading dir entries").path();

            if path.ends_with(component_name) {
                panic!("Directory {component_name} already exists!");
            }        
        });

        // Create component directory

        let component_dir_path = target_dir.join(component_name);

        fs::create_dir(&component_dir_path).expect("Fails creating component dir");

        println!("Directory {component_dir_path:?} created");

        // Write files from templates
        
        for (name, filename_tmpl) in Self::TEMPLATES {
            let path = Path::new(&app_dir).join("templates").join(name);
            let mut file = fs::File::open(&path).expect(&format!("Fails opening template file: {:?}", path));
            let mut text = String::new();
            file.read_to_string(&mut text).expect("Fails reading the template file");
            let replaced = text.replace(Self::PLACEHOLDER_NAME, component_name);
            let file_name = String::from(*filename_tmpl).replace(Self::PLACEHOLDER_NAME, component_name);
            
            let destination_path = &component_dir_path.join(&file_name);
            
            let mut destination_file = fs::File::create(destination_path).expect("Fails creating the destination file");
            destination_file.write_all(replaced.as_bytes()).expect("Fails writing the destination file");
            println!("File {path:?} created");
        }

        // Update index.ts

        let index_path = Path::new(&target_dir).join("index.ts");

        match fs::File::options().append(true).open(&index_path) {
            Err(error) => {
                println!("opening index: {index_path:?}: {error}");
            },
            Ok(mut index_file) => {
                let export_statement = format!("export * from './{component_name}'\n");
                index_file.write(export_statement.as_bytes()).expect("Fails updating the index.ts");
                println!("File {index_path:?} updated");
            }
        };

        Ok(())
    }

}

fn print_help() {
    println!("{}", HELP_MESSAGE);
}

const HELP_MESSAGE: &str = r#"
    Welcome to `ilujo` (the word for toolbox in esperanto)

    Commands:

    ui [name]   Create a UI component boilerplate called [name] in the current directory
                e.g. `ilujo ui HelloWorld`

    app-dir     Open the app directory installation with the default window manager
                e.g. `ilujo open-dir`

    help        Show this message
"#;
