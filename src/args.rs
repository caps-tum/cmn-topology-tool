use std::fmt;
use std::fmt::Formatter;
use clap::{Args, Parser, Subcommand};

#[derive(Args, Debug)]
pub struct DetermineAllArgs {
    pub benchmark_binary_path: String,

    #[arg(default_value_t=String::from(""))]
    pub benchmark_binary_args: String,

    #[arg(default_value_t=2)]
    pub cores_per_dsu: u8,

    #[arg(value_enum)]
    pub numa_config: NUMAConfig
}

#[derive(Args, Debug)]
pub struct DetermineCoresArgs {
    pub benchmark_binary_path: String,

    #[arg(default_value_t=String::from(""))]
    pub benchmark_binary_args: String,

    #[arg(default_value_t=8)]
    pub mesh_x: u16,
    #[arg(default_value_t=6)]
    pub mesh_y: u16,

    #[arg(default_value_t=2)]
    pub cores_per_dsu: u8,
}


#[derive(Clone, Copy, Debug, clap::ValueEnum, Default)]
pub enum NUMAConfig {
    #[default]
    Monolithic=0,
    Hemisphere=1,
    Quadrant=2
}

impl fmt::Display for NUMAConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
#[derive(Args, Debug)]
pub struct DetermineEdgesArgs {
    pub benchmark_binary_path: String,

    #[arg(default_value_t=String::from(""))]
    pub benchmark_binary_args: String,

    #[arg(default_value_t=8)]
    pub mesh_x: u16,
    #[arg(default_value_t=8)]
    pub mesh_y: u16,

    #[arg(default_value_t=2)]
    pub cores_per_dsu: u8,

    #[arg(value_enum, default_value_t=NUMAConfig::Monolithic)]
    pub numa_config: NUMAConfig
}

#[derive(Args, Debug)]
pub struct DetermineNodesArgs {
    #[arg(default_value_t=8)]
    pub mesh_x: u16,
    #[arg(default_value_t=6)]
    pub mesh_y: u16,
}

#[derive(Subcommand,Debug)]
pub enum Commands {
    DetermineMesh,
    DetermineNodes(DetermineNodesArgs),
    DetermineCores(DetermineCoresArgs),
    DetermineEdges(DetermineEdgesArgs),
    DetermineAll(DetermineAllArgs)
}

#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
pub struct Cli {
    #[arg(long, default_value_t=9)]
    pub nodeid_length: u8,

    #[arg(long, default_value_t=String::from(""))]
    pub events: String,

    #[arg(long, default_value_t=String::from("data"))]
    pub outdir: String,

    #[command(subcommand)]
    pub command: Commands
}