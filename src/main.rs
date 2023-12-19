mod args;
mod determine_mesh;
mod determine_nodes;
mod determine_cores;
mod determine_edges;
mod utils;
mod event;
mod writer;

use std::fs;
use std::path::Path;
use clap::Parser;
use crate::writer::Writer;

fn main() {
    env_logger::init();
    let args = args::Cli::parse();

    let basepath = fs::canonicalize(Path::new(args.outdir.as_str()))
        .expect(format!("Folder at {} does not exist!", args.outdir).as_str());

    let writer = Writer::new(String::from(basepath.as_path().to_str().unwrap()), &args);
    writer.write_meta();
    let events = utils::events_map_to_vec();
    writer.write_lines(events, "events.csv");

    match &args.command {
        args::Commands::DetermineMesh => {
             determine_mesh::determine(Some(args.nodeid_length), &writer);
        }
        args::Commands::DetermineNodes(dargs) => {
            determine_nodes::determine(Some(args.nodeid_length), (dargs.mesh_x, dargs.mesh_y),
                                       &writer);
        }
        args::Commands::DetermineCores(dargs) => {
            determine_cores::determine(Some(args.nodeid_length),
                                       (dargs.mesh_x, dargs.mesh_y),
                                       dargs.cores_per_dsu as u16,
                                       Path::new(&dargs.benchmark_binary_path),
                                       dargs.benchmark_binary_args.as_str(),
                                       &writer);
        }

        args::Commands::DetermineEdges(dargs) => {
            determine_edges::determine(Some(args.nodeid_length),
                                       (dargs.mesh_x, dargs.mesh_y),
                                       dargs.cores_per_dsu as u16,
                                       dargs.numa_config,
                                       Path::new(&dargs.benchmark_binary_path),
                                       dargs.benchmark_binary_args.as_str(),
                                       &writer);
        }
        args::Commands::DetermineAll(dargs) => {
            let mesh_size = determine_mesh::determine(Some(args.nodeid_length),
                                                      &writer);
            determine_nodes::determine(Some(args.nodeid_length), mesh_size, &writer);
            determine_cores::determine(Some(args.nodeid_length), mesh_size, dargs.cores_per_dsu as u16,
                                       Path::new(&dargs.benchmark_binary_path), dargs.benchmark_binary_args.as_str(),
                                       &writer);
            determine_edges::determine(Some(args.nodeid_length),
                                       mesh_size,
                                       dargs.cores_per_dsu as u16,
                                       dargs.numa_config,
                                       Path::new(&dargs.benchmark_binary_path),
                                       dargs.benchmark_binary_args.as_str(),
                                       &writer);
        }
    }

}
