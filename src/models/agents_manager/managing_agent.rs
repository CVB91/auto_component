use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

#[derive(Debug)]
pub struct ManagingAgent{
  attributes: BasicAgent,
  factsheet: FactSheet,
  agents: Vec<Box<dyn SpecialFunctions>>,
}