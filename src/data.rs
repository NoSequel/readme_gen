use serde::*;

use std::fs;
use std::fs::File;
use std::path::Path;

use std::io::prelude::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    pub header: String,

    pub experiences: Vec<Experience>,
    pub projects: Vec<Project>,
    pub stats_url: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Project {
    pub project_name: String,
    pub project_url: String
}

#[derive(Deserialize, Serialize, Clone)]

pub struct Experience {
    pub experience_id: String,
    pub experience_info: String
}

impl Settings {
    pub fn load_settings() -> Settings {
        let file_path = "./settings.json";
        let file_contents = fs::read_to_string(file_path);
        let path = Path::new(file_path);

        if let Ok(contents) = file_contents {
            return serde_json::from_str(contents.as_str()).expect("Unable to parse from JSON");
        } else {
            let settings = Settings {
                header: String::from("NV6"),
                stats_url: String::from("https://github.com/anuraghazra/github-readme-stats"),
                projects: vec![
                    Project {
                        project_name: String::from("Example"),
                        project_url: String::from("https://github.com/Example")
                    }
                ],
                experiences: vec![
                    Experience {
                        experience_id: String::from("Example"),
                        experience_info: String::from("haha example")
                    }
                ]
            };

            let mut file = match File::create(file_path) {
                Err(why) => panic!("couldn't create {}: {}", path.display(), why),
                Ok(file) => file,
            };

            file.write_all(
                serde_json::to_string_pretty(&settings)
                    .expect("Unable to parse to JSON")
                    .as_bytes(),
            )

            .unwrap();
            return settings;
        }
    }

    pub fn write_settings(&self) {
        let file_path = "./settings.json";
        let path = Path::new(file_path);

        let mut file = match File::create(file_path) {
            Err(why) => panic!("couldn't create {}: {}", path.display(), why),
            Ok(file) => file,
        };

        file.write_all(
            serde_json::to_string_pretty(self)
                .expect("Unable to parse to JSON")
                .as_bytes(),
        )
        .unwrap();
    }
}
