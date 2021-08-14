use clap::{App, Arg};

/// A collection of methods allowed to be called on the database
#[derive(Debug)]
pub enum DBOperation {
    /// Create a new row in the specified table
    NewRowOp(Vec<String>),
    /// List all tables in database
    ListAllTables,
}

/// Parses the command for db operation and the data
pub fn parse_line() -> DBOperation {
    let matches = App::new("schooled-cli")
        .version("0.1")
        .author("Ragnyll <ragnyll@gallowzhumour.dev>")
        .about("A handful of cli methods for interacting with the school_db. Can be used interactively or via a set of command arguments.")
        .subcommand(
            App::new("new-row")
                .arg(
                    Arg::new("new-row-values")
                        .required(true)
                        .multiple(true)
                ),
        )
        .subcommand(App::new("list-tables"))
        .get_matches();

    match matches.subcommand() {
        Some(("new-row", new_row_command_args)) => {
            match new_row_command_args.values_of_lossy("new-row-values") {
                Some(values) => DBOperation::NewRowOp(values),
                // Let the new row handle figure out what to do with the empty array
                _ => DBOperation::NewRowOp(vec![])
            }
        },

        Some(("list-tables", _)) => DBOperation::ListAllTables,

        // This should be some unknown command error
        _ => DBOperation::ListAllTables
    }

    // if matches.is_present("new-row") {
        // return DBOperation::NewRowOp(matches.values_of_lossy("new-row").unwrap());
    // }
    // match matches.expect("subcommand required for db_util") {
    // ("new", _new_matches)  => DBOperation::NewRowOp,
    // _ => panic!("Specified command not found"),
    // }
}
