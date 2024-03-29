use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::helpers::general::ai_task_request;
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::agents::agent_backend::AgentBackendDeveloper;

#[derive(Debug)]
pub struct ManagingAgent {
    _attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position: String = "Project Manager".to_string();

        let attributes: BasicAgent = BasicAgent {
            objective: "Manage agents who are building an excellent website for the user"
                .to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        let project_description: String = ai_task_request(
            usr_req,
            &position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        let factsheet: FactSheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };
        //Reutrn Self

        Ok(Self {
            _attributes: attributes,
            factsheet,
            agents,
        })
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }
    // Can add as many agents as you want here -- this can be expanded to include more agents such as a frontend developer, a designer, etc.
    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        self.add_agent(Box::new(AgentBackendDeveloper::new()));
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        // This is where Rust really excels, in another language we would pass ina  variable (here the factsheed in agent_res, and then have to pass that back and change it. Here we can pass a mutable reference and the agent can change it directly. This is a huge advantage of Rust.)

        for agent in &mut self.agents {
            let _agent_res: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.factsheet).await;

            // let agent_info: &BasicAgent = agent.get_attributes_from_agent();
            // dbg!(agent_info);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_managing_agent() {
        let usr_request: &str = "need a full stack app that fetches and tracks my fitness progress. Needs to include timezone info from the web.";

        let mut managing_agent: ManagingAgent = ManagingAgent::new(usr_request.to_string())
            .await
            .expect("Error creating Managing Agent");

        managing_agent.execute_project().await;

        dbg!(managing_agent.factsheet);
    }
}
