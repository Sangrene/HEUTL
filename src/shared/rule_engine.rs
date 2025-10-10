use crate::shared::errors::Error;
use serde_json::Value;
use zen_engine::{DecisionEngine, Variable, model::DecisionContent};

pub async fn evaluate(content: &Value) -> Result<Value, Error> {
    let content: DecisionContent = serde_json::from_value(content.clone())?;
    let engine = DecisionEngine::default();
    let decision = engine.create_decision(content.into());
    let context = Variable::Bool(false);
    let result = decision
        .evaluate(context)
        .await
        .map_err(|e| Error::RuleEngineError(e.to_string()))?;

    return Ok(result.result.to_value());
}
