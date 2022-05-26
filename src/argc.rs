use clap::{crate_version, App, Arg, crate_name};

pub fn argc_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(&crate_version!()[..])
        .about("Which Is Better")
        .arg(Arg::with_name("nums").help("Number of biggest files to display").short("n").takes_value(true))
        .version_short("v")
}