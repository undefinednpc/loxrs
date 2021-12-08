use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    mode: Option<String>,
    #[structopt(required_if("mode", "run"))]
    file: Option<String>,
}

fn run_file(name: String) {
    println!("{}", name);
}

fn run_prompt() {}

fn main() {
    let args = Opt::from_args();
    match args.mode {
        Some(x) => { if x == "interactive" {
            run_prompt();
        } else if x == "run" {
            match args.file {
                Some(x) => { run_file(x) }
                None => { println!("no file") }
            }
        }
        }
        None => { println!("no mode") }
    };
}
