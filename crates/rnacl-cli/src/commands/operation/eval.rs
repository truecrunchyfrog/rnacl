use std::io::{Read, stdin};

use rnacl_core::{registry, sample::batch::Batch};

use crate::args::operation::eval::EvalOperationArgs;

pub(super) fn dispatch(args: EvalOperationArgs) -> anyhow::Result<()> {
    let options = match args.options {
        Some(serialized) => serde_json::from_str::<serde_json::Value>(&serialized)?,
        None => serde_json::Value::Null,
    };

    let operation = registry::resolve_op(&args.operation_id)?;

    let input = if !args.head {
        let mut buf = Vec::new();
        stdin().read_to_end(&mut buf)?;
        serde_json::from_slice::<Batch>(buf.as_slice())?
    } else {
        Batch::default()
    };

    let output = operation.eval(input, &options)?;

    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}
