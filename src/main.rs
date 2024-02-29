mod args;

mod commands;
mod utils;
mod event;
mod writer;

use std::path::Path;
use clap::Parser;
use log::info;
use crate::writer::Writer;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = args::Cli::parse();
    let basepath = if args.outdir == "" { None } else { Some(args.outdir.clone()) };

    let mut writer = Writer::new(basepath, &args);
    if args.events.is_some() || matches!(args.command, args::Commands::DetermineTopology(_)) {
        writer.write_lines(utils::events_map_to_vec(), "events.csv");
    }

    if writer.basepath.is_some() {
        info!("Will write data to: {:?}", writer.get_outpath());
    }

    match &args.command {
        args::Commands::DetermineTopology(dargs) => {
            let mesh_size = commands::determine_mesh::determine(args.nodeid_length, &writer);
            commands::determine_nodes::determine(args.nodeid_length, mesh_size, &writer);
            commands::determine_cores::determine(args.nodeid_length, mesh_size, args.cores_per_dsu as u16,
                                                 Path::new(&dargs.benchmark_binary_path),
                                                 dargs.benchmark_binary_args.clone(),  &writer);
        }

        args::Commands::Launch(largs) => {
           commands::launch::launch(largs, (args.mesh_x, args.mesh_y), args.nodeid_length,
                                    args.events, &writer);
        }

        args::Commands::LaunchMulti(largs) => {
            commands::launch_multi::launch_multi(largs, (args.mesh_x, args.mesh_y), args.nodeid_length,
                                                 args.events, &mut writer);
        }
    }
    writer.write_meta();

}
