use std::collections::HashMap;
use std::fs;
use crate::data::project::{PROJECT_FILES, ProjectType};

pub fn all() -> std::io::Result<Vec<ProjectType>> {
    let mut project_type = HashMap::new();

    for (file, project) in PROJECT_FILES.iter() {
        project_type.insert(*file, *project);
    }

    let entries = fs::read_dir(".")?;
    let mut detected_projects = Vec::<ProjectType>::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    if let Some(project) = project_type.get(name_str) {
                        detected_projects.push(*project);
                    }
                }
            }
        }
    }

    Ok(detected_projects)
}
