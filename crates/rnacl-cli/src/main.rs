use std::{collections::HashMap, process::ExitCode};

use rnacl_core::{
    operation::Operation,
    registry,
    sample::{Sample, Trace, batch::Batch},
};
use serde_json::json;

mod args;
mod commands;
pub(crate) mod helper;
mod logging;
pub(crate) mod reuse_node;
pub(crate) mod ui;

fn main() -> ExitCode {
    let cli = args::parse();
    logging::init(&cli);

    registry::register_op(
        "heads.nums".to_string(),
        Operation::new(
            "two numbers, 'a' and 'b'".to_string(),
            Box::new(|_, opts| {
                Ok(Batch::new(vec![
                    Sample::new(
                        Trace::new(HashMap::from_iter(vec![
                            ("_op".to_string(), json!("heads.nums")),
                            ("src".to_string(), json!("a")),
                        ])),
                        opts.get("a").unwrap().to_string(),
                    ),
                    Sample::new(
                        Trace::new(HashMap::from_iter(vec![
                            ("_op".to_string(), json!("heads.nums")),
                            ("src".to_string(), json!("b")),
                        ])),
                        opts.get("b").unwrap().to_string(),
                    ),
                ]))
            }),
        ),
    )
    .unwrap();

    registry::register_op(
        "increment".to_string(),
        Operation::new(
            "increment all inputs by 'amount'".to_string(),
            Box::new(|b, opts| {
                Ok(b.transform(|s| {
                    Some(s.transform(|value| {
                        (
                            Trace::new(HashMap::from_iter(vec![(
                                "_op".to_string(),
                                json!("increment"),
                            )])),
                            (value.parse::<i64>().unwrap()
                                + opts.get("amount").unwrap().as_i64().unwrap())
                            .to_string(),
                        )
                    }))
                }))
            }),
        ),
    )
    .unwrap();

    match commands::dispatch(cli) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            ui::error(format!("{}", err));
            ExitCode::FAILURE
        }
    }
}
