//TODO: Add fields to DockerManager

use docker_api::{Container, Docker};
use docker_api::models::{ContainerSummary};

//TODO: replace with Docker Struct from docker_api
#[derive(Clone, Debug)]
pub struct DockerContainer {
    pub name: String,
    pub id:String,
}
impl DockerContainer {
    pub fn new(name: &str, id: &str) -> Self {
        DockerContainer {
            name: String::from(name),
            id: String::from(id)
        }
    }
}

#[derive(Debug)]
pub struct DockerManager {
    pub containers: Vec<ContainerSummary>,
    pub docker_containers: Vec<Container>,
    pub options: Vec<String>
}
impl DockerManager {
    pub async fn new() -> Option<Self> {

        let docker_api = Docker::new(&"tcp://localhost:2375");
        if docker_api.is_err() { return None }

        let containers = docker_api
            .unwrap()
            .containers()
            .list(&Default::default())
            .await;

        if  containers.is_err() { return None }

        Some(
            Self {
                containers: containers.unwrap(),
                docker_containers: vec![],
                options: vec![
                    String::from("Start"),
                    String::from("Delete"),
                    String::from("Inspect"),
                    String::from("Restart"),
                    String::from("Observe"),
                    String::from("Info")
                ],
            }
        )
    }
    pub fn get_containers(&self) -> Vec<ContainerSummary> {
        self.containers.clone()
    }
    pub fn get_container_options(&self) -> Vec<String> {
        self.options.clone()
    }
    pub fn execute_command(&self, states: Vec<u32>) -> String {
        if states.len() != 2 {
            eprintln!("(DockerManager::execute_command) amount of states does not match");
            return String::new();
        }
        let container_index = states[0];
        let container_option_index= states[1];

        let container = &self.containers[container_index as usize];
        let option= &self.options[container_option_index as usize];



        String::from(format!("Container {:?}, Option: {:?}", container, option))
    }
}