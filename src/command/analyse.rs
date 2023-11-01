pub fn run() {
    let project_types = crate::service::detect_project::all().unwrap();
    for project_type in project_types {
        println!("Detected project type: {:?}", project_type);
    }
}
