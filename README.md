# xmip-core-assign

Assignment: setting Message Context values from a literal or from another
context value, in order, producing a new context. `apply` takes a context and
the assignments and hands back the result. A source value that is missing or
`Null` is absent, one thing as a filter reads it (ADR-0046, amendment
2026-09-24), so it assigns nothing and the target is left as it was.

Assignment belongs to an Xmip Process alone; a Receive Port or a Send Port
transforms but does not assign (`runtime-model.md` section 22). Assignment
creates a new Message generation, as transformation does; routing does not.

`doc/architecture/runtime-model.md` sections 3 and 22 govern it;
`architecture.toml` carries the maturity.
