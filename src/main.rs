mod create_tx;

use structopt::StructOpt;
use crate::create_tx::create_tx;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    descriptor: String,

    #[structopt(long)]
    network: String,

    #[structopt(long)]
    output_script: String,

    #[structopt(long)]
    channel_value_satoshis: u64,
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);

    let funding_transaction: String = create_tx(
        opt.descriptor,
        opt.network,
        opt.output_script,
        opt.channel_value_satoshis
    ).unwrap();

    println!("{}", funding_transaction)
}
