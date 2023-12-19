mod benchmark;

use clap::Parser;
use core_affinity::CoreId;

#[derive(Clone)]
#[derive(clap::Parser, Debug)]
pub struct CliArgs {
    /// The number of iterations per sample
    #[clap(long, default_value_t = 50000, value_parser)]
    num_iterations: u32,

    /// The number of samples
    #[clap(long, default_value_t = 5000, value_parser)]
    num_samples: u32,

    /// Specify the cores by id that should be used, comma delimited. By default all cores are used.
    #[clap(short, long, value_delimiter=',', value_parser)]
    cores: Vec<usize>,
}
fn main() {
    let args = CliArgs::parse();
    let r = benchmark::run(CoreId { id: args.cores[0] }, CoreId { id: args.cores[1] },
                           args.num_iterations, args.num_samples);
    println!("{r}");
}