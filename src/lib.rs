#![forbid(unsafe_code)]

use context::{ContextValue, MessageContext};

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
            AssignmentValue::Context(key) => {
                current.get(key).cloned().unwrap_or(ContextValue::Null)
            }
        };
        current.with_value(assignment.target_key.clone(), value)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_literal_is_set_and_a_context_value_is_copied() {
        let context =
            MessageContext::new().with_value("source", ContextValue::Text("edi".to_string()));
        let assigned = apply(
            context,
            &[
                Assignment {
                    target_key: "priority".to_string(),
                    value: AssignmentValue::Literal(ContextValue::Integer(1)),
                },
                Assignment {
                    target_key: "origin".to_string(),
                    value: AssignmentValue::Context("source".to_string()),
                },
            ],
        );
        assert_eq!(assigned.get("priority"), Some(&ContextValue::Integer(1)));
        assert_eq!(
            assigned.get("origin"),
            Some(&ContextValue::Text("edi".to_string()))
        );
    }

    #[test]
    fn a_missing_context_key_assigns_null_rather_than_nothing() {
        let assigned = apply(
            MessageContext::new(),
            &[Assignment {
                target_key: "copy".to_string(),
                value: AssignmentValue::Context("absent".to_string()),
            }],
        );
        assert_eq!(assigned.get("copy"), Some(&ContextValue::Null));
    }
}
