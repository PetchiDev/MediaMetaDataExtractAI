// AWS Step Functions integration
// I-FR-32: Workflow creation
// I-FR-33: Preprocessing workflow execution

use aws_sdk_sfn::Client as StepFunctionsClient;
use aws_config::SdkConfig;
use anyhow::Result;
use serde_json::Value;
use uuid::Uuid;

pub struct StepFunctionsService {
    client: StepFunctionsClient,
}

impl StepFunctionsService {
    pub fn new(config: &SdkConfig) -> Self {
        let client = StepFunctionsClient::new(config);
        Self { client }
    }

    // I-FR-32, I-FR-33: Start workflow execution
    pub async fn start_execution(
        &self,
        state_machine_arn: &str,
        input: Value,
        name: Option<String>,
    ) -> Result<String> {
        let execution_name = name.unwrap_or_else(|| format!("exec-{}", Uuid::new_v4()));
        
        let input_json = serde_json::to_string(&input)?;
        
        let response = self.client
            .start_execution()
            .state_machine_arn(state_machine_arn)
            .name(&execution_name)
            .input(input_json)
            .send()
            .await?;

        Ok(response.execution_arn().to_string())
    }

    pub async fn get_execution_status(
        &self,
        execution_arn: &str,
    ) -> Result<Value> {
        let response = self.client
            .describe_execution()
            .execution_arn(execution_arn)
            .send()
            .await?;

        // Handle start_date and stop_date
        // Note: These fields may not always be available, so we handle them as optional
        let start_date_secs: Option<i64> = None; // TODO: Fix DateTime handling for AWS SDK version
        let stop_date_secs: Option<i64> = None; // TODO: Fix DateTime handling for AWS SDK version
        
        let status = serde_json::json!({
            "status": response.status().as_str().to_string(),
            "start_date": start_date_secs,
            "stop_date": stop_date_secs,
            "output": response.output().and_then(|o| serde_json::from_str::<Value>(o.as_ref()).ok()),
        });

        Ok(status)
    }
}
