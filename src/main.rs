mod data;

use data::Settings;

use std::error::Error;
use std::{fs::File, io::{ErrorKind, Write}};

fn main() {
    let settings = Settings::load_settings();
    let file = &mut create_file();

    write_to_file(file, settings);
}

fn create_file() -> File {
    File::create("README.md").expect("Unable to create README.md file")
}

fn write_to_file(file: &mut File, settings: Settings) -> Result<(), Box<dyn Error>>{
    file.write_all(format!("# {}\n\n", settings.header).as_bytes())?;

    file.write_all(b"## Previous Experiences\n")?;

    for element in settings.clone().experiences {
        file.write_all(format!("### {}\n", element.experience_id).as_bytes())?;
        file.write_all(format!("{}\n\n", element.experience_info).as_bytes())?;
    }

    file.write_all(b"## Projects\n")?;

    for project in settings.projects {
        file.write_all(format!("[{}]({})\n", project.project_name, project.project_url).as_bytes())?;
    }

    file.write_all(format!("\n## Stats\n[![NV6's stats]({})](https://github.com/anuraghazra/github-readme-stats)", settings.stats_url).as_bytes())?;

    Ok(())
}