#![forbid(unsafe_code)]

use xmip_context::{ContextValue, MessageContext};

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentValue {
    Literal(ContextValue),
    Context(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub target_key: String,
    pub value: AssignmentValue,
}

pub fn apply(context: MessageContext, assignments: &[Assignment]) -> MessageContext {
    assignments.iter().fold(context, |current, assignment| {
        let value = match &assignment.value {
            AssignmentValue::Literal(value) => value.clone(),
            AssignmentValue::Context(key) => current.get(key).cloned().unwrap_or(ContextValue::Null),
        };
        current.with_value(assignment.target_key.clone(), value)
    })
}
