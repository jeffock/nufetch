use nu_protocol::ast::Call;
use nu_protocol::engine::{CommandRegistry,EvaluationContext,ShellError};
use nu_protocol::pipeline::Pipeline;
use nu_protocol::Value;
use nu_source::Tag;

use std::sync::Arc;

//clear, ascii + sys info, compact (c) or normal (.)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //empty command registry
    let mut registry = CommandRegistry::new();

    //evalution context
    let context = EvaluationContext::basic()?;

    //parese commands
    let mut pipeline = Pipeline::new();
    let call = Call {
        head: nu_protocol::hir::Expression::external_command("clear"),
        positional: vec![],
        named: vec![],
        span: Tag::unknown(),
    };

    //execute pipeline
    let result = context.run_pipeline(
        &mut registry,
        Arc::new(pipeline),
        Value::nothing(),
    );

    //handle result
    match result {
        Ok(Value::Stream(stream)) => {
            for item in stream {
                println!("{}",item.format(None));
            }
        }
        Err(ShellError::UntaggedRuntimeError(msg)) => {
            eprintln!("Error: {}",msg);
        }
        _ => {
            eprintln!("An error occurred");
        }
    }

    Ok(())
}
