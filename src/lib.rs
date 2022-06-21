use core::num;
use std::{error::Error, net::{IpAddr, TcpStream}, sync::mpsc::Sender, io::{self, Write}};
// const MAX: u16 = 65535;


type SBResult<T> = Result<T, Box<dyn Error>>;

const MAX: u16 = 65535;


#[derive(Debug)]
pub struct CLI {
    ip: std::net::IpAddr,
    jobs: u16,

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

pub fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}

        }
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }

}

pub fn run(cli: CLI) -> SBResult<()> {
    // let addr = cli.ip;
    // let num_threads = cli.jobs;
    let (tx, rx) = std::sync::mpsc::channel();
    for i in 0..cli.jobs {
        let tx = tx.clone();

        std::thread::spawn(move || {
            scan(tx, i, cli.ip, cli.jobs)
        });

    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }


    Ok(())
}