#![forbid(unsafe_code)]

use context::MessageContext;

use xcore::ScalarValue;

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentValue {
    Literal(ScalarValue),
    Context(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub target_key: String,
    pub value: AssignmentValue,
}

/// Apply `assignments` in order: a literal is set, and a context value is
/// copied. A source that is missing, or holds `Null`, is absent — one thing,
/// as a filter reads it (ADR-0046, amendment 2026-09-24) — so it assigns
/// nothing and the target is left as it was.
#[must_use]
pub fn apply(context: MessageContext, assignments: &[Assignment]) -> MessageContext {
    assignments.iter().fold(context, |current, assignment| {
        let value = match &assignment.value {
            AssignmentValue::Literal(value) => Some(value.clone()),
            AssignmentValue::Context(key) => current
                .get(key)
                .filter(|value| **value != ScalarValue::Null)
                .cloned(),
        };
        match value {
            Some(value) => current.with_value(assignment.target_key.clone(), value),
            None => current,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_literal_is_set_and_a_context_value_is_copied() {
        let context =
            MessageContext::new().with_value("source", ScalarValue::Text("edi".to_string()));
        let assigned = apply(
            context,
            &[
                Assignment {
                    target_key: "priority".to_string(),
                    value: AssignmentValue::Literal(ScalarValue::Integer(1)),
                },
                Assignment {
                    target_key: "origin".to_string(),
                    value: AssignmentValue::Context("source".to_string()),
                },
            ],
        );
        assert_eq!(assigned.get("priority"), Some(&ScalarValue::Integer(1)));
        assert_eq!(
            assigned.get("origin"),
            Some(&ScalarValue::Text("edi".to_string()))
        );
    }

    #[test]
    fn a_missing_or_null_source_assigns_nothing() {
        let context = MessageContext::new()
            .with_value("note", ScalarValue::Null)
            .with_value("kept", ScalarValue::Text("as it was".to_string()));
        let copy = |target: &str, source: &str| Assignment {
            target_key: target.to_string(),
            value: AssignmentValue::Context(source.to_string()),
        };

        let assigned = apply(
            context,
            &[
                copy("copy", "absent"),
                copy("noted", "note"),
                copy("kept", "absent"),
            ],
        );

        assert!(!assigned.contains_key("copy"), "missing is absent");
        assert!(!assigned.contains_key("noted"), "Null is absent");
        assert_eq!(
            assigned.get("kept"),
            Some(&ScalarValue::Text("as it was".to_string()))
        );
    }
}
