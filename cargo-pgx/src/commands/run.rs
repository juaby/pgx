use crate::commands::install::install_extension;
use crate::commands::start::start_postgres;
use crate::commands::stop::stop_postgres;
use colored::Colorize;
use pgx_utils::{createdb, get_pg_config, get_psql_path, handle_result, BASE_POSTGRES_PORT_NO};
use std::process::Stdio;

pub(crate) fn run_psql(major_version: u16, dbname: &str) {
    let pg_config = get_pg_config(major_version);

    // install the extension
    install_extension(&pg_config, false, None);

    // restart postgres
    stop_postgres(major_version);
    start_postgres(major_version);

    // create the named database
    if !createdb(
        major_version,
        "localhost",
        BASE_POSTGRES_PORT_NO + major_version,
        dbname,
        true,
    ) {
        println!(
            "{} existing database {}",
            "    Re-using".bold().cyan(),
            dbname
        );
    }

    // run psql
    exec_psql(major_version, dbname);
}

fn exec_psql(major_version: u16, dbname: &str) {
    let mut command = std::process::Command::new(get_psql_path(major_version));
    command
        .arg("-h")
        .arg("localhost")
        .arg("-p")
        .arg((BASE_POSTGRES_PORT_NO + major_version).to_string())
        .arg(dbname)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());

    let status = handle_result!("problem starting psql", command.status());

    // as soon as psql exists, exit our process and return its status code
    std::process::exit(status.code().expect("no return code from psql"))
}
