mod args;
mod commands;
mod logging;

fn main() {
    logging::init();

    let args = args::parse();

    let result = commands::dispatch(args);

    if let Err(err) = result {
        error::report(err);
        std::process::exit(1);
    }
}
