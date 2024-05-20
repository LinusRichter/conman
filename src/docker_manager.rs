//TODO: Add fields to DockerManager
#[derive(Clone, Debug)]
pub struct DockerManager {}

//TODO: replace with Docker Struct from docker_api
#[derive(Clone, Copy)]
pub struct DockerContainer<'a> {
    pub name: &'a str,
    pub id: &'a str,
}
impl<'a> DockerContainer<'a> {
    pub fn new(name: &'a str, id: &'a str) -> Self {
        DockerContainer { name, id }
    }
}

//TODO: Add implementation to DockerManager
impl DockerManager {
    pub fn new() -> Self {
        Self{}
    }
    pub fn get_containers(&self) -> Vec<DockerContainer> {
        vec![
            DockerContainer::new("Container", "2"),
            DockerContainer::new("Container", "3"),
            DockerContainer::new("Container", "4"),
            DockerContainer::new("Container", "5"),
        ]
    }
    pub fn get_container_options(&self) -> Vec<&str> {
        vec![
            "Start",
            "Delete",
            "Inspect",
            "Restart",
            "Observe",
            "Info"
        ]
    }
    pub fn execute_command(&self, states: Vec<u32>) -> String {
        String::from(format!("RESULT: {:?}", states))
    }
}