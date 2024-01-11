use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct RouteObject{
  pub is_route_dynamic: String,
  pub method: String,
  pub request_body: serde_json::Value,
  pub response: serde_json::Value,
  pub route: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct ProjectScope{
  pub is_crud_required: bool,
  pub is_user_login_and_logout: bool,
  pub is_external_urls_required: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FactSheet{
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

pub trait SpecialFunctions : Debug {


  //Used so that manager can get attributes from agent
  fn get_attributes_from_agent(&self) -> &BasicAgent;


// This function will allow agents to execute their logic

  async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>>;

}
    
