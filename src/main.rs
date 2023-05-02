use analyze_xlsx::analyze::analyze;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    template: String,
}

fn main() {
    let args = Args::parse();
    let res = analyze(args.template.as_str());
    match res {
        Ok(res) => println!("{}", res),
        Err(err) => println!("{:?}", err),
    }
}
