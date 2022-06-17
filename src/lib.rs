use std::error::Error;
// const MAX: u16 = 65535;


type SBResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct CLI {
    ip: std::net::IpAddr,
    jobs: u8,

}

pub fn get_args() -> SBResult<CLI> {

    let cli = clap::command!()
        .arg_required_else_help(true)
        .arg(
            clap::Arg::new("ip")
                .value_name("IP")
                .takes_value(true)
            
        )
        .arg(
            clap::Arg::new("jobs")
                .short('j')
                .long("jobs")
                .takes_value(true)
                .default_value("1")
                .help("Number of parallel worker threads")
        )
        .get_matches();

    Ok(CLI{
        ip: cli.value_of("ip").unwrap().parse()? ,
        jobs: cli.value_of("jobs").unwrap().parse()? ,
    })

}

pub fn run(cli: CLI) -> SBResult<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    for i in 0..cli.jobs {
        let tx = tx.clone();

        std::thread::spawn(move || {
            scan(tx, i, cli.ip, cli.jobs)
        })

    }


    Ok(())
}