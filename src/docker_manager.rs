//TODO: Add fields to DockerManager

//TODO: replace with Docker Struct from docker_api
#[derive(Clone, Copy, Debug)]
pub struct DockerContainer<'a> {
    pub name: &'a str,
    pub id: &'a str,
}
impl<'a> DockerContainer<'a> {
    pub fn new(name: &'a str, id: &'a str) -> Self {
        DockerContainer { name, id }
    }
}

#[derive(Clone, Debug)]
pub struct DockerManager<'a> {
    pub container: Vec<DockerContainer<'a>>,
    pub options: Vec<&'a str>
}
impl DockerManager<'_> {
    pub fn new() -> Self {
        Self {
            container: vec![
                DockerContainer::new("Container", "2"),
                DockerContainer::new("Container", "3"),
                DockerContainer::new("Container", "4"),
                DockerContainer::new("Container", "5"),
            ],
            options: vec![
                "Start",
                "Delete",
                "Inspect",
                "Restart",
                "Observe",
                "Info"
            ],
        }
    }
    pub fn get_containers(&self) -> Vec<DockerContainer> {
        self.container.clone()
    }
    pub fn get_container_options(&self) -> Vec<&str> {
        self.options.clone()
    }
    pub fn execute_command(&self, states: Vec<u32>) -> String {
        if states.len() != 2 {
            eprintln!("(DockerManager::execute_command) amount of states does not match");
            return String::new();
        }
        let container_index = states[0];
        let container_option_index= states[0];

        let container = self.container[container_index as usize];
        let option= self.options[container_option_index as usize];
        String::from(format!("Container {:?}, Option: {:?}", container, option))
    }
}