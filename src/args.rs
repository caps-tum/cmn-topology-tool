/* Structs for CLI parameter parsing */

use std::fmt;
use std::fmt::Formatter;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

static DEFAULT_MESH_X: u16 = 8;
static DEFAULT_MESH_Y: u16 = 8;
static DEFAULT_CORES_PER_DSU: u8 = 2;
static DEFAULT_NODEID_LENGTH: u8 = 9;

/* Topology Parameters */
#[derive(Args, Debug)]
pub struct DetermineTopologyArgs {
    #[arg(long, value_enum)]
    pub numa_config: NUMAConfig,

    #[arg(long)]
    pub benchmark_binary_path: String,

    #[arg(long, trailing_var_arg = true, value_delimiter = ' ', allow_hyphen_values = true)]
    pub benchmark_binary_args: Option<Vec<String>>,
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

/* Launch* parameters */
#[derive(Args, Debug)]
pub struct LaunchArgs {

    #[arg(long)]
    pub core_map: Option<String>,

    #[arg(long)]
    pub shell: Option<String>,

    #[arg(long, value_delimiter=' ')]
    pub env: Option<Vec<String>>,

    #[arg(long)]
    pub pwd: Option<String>,

    #[arg(long)]
    pub binary: String,

    #[arg(long, trailing_var_arg = true, value_delimiter = ' ', allow_hyphen_values = true)]
    pub args: Option<Vec<String>>,
}

#[derive(Args,Debug)]
pub struct LaunchMultiArgs {
    #[arg(long)]
    pub config: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct LaunchMultiConfig {
    pub executables: Vec<LaunchMultiExecutableConfig>
}

#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct LaunchMultiExecutableConfig {
    pub name: Option<String>,
    pub shell: Option<String>,
    pub binary: String,
    pub args: Option<String>,
    pub core_map: Option<String>,
    pub env: Option<Map<String,Value>>
}


/* Main CLI */
#[derive(Subcommand,Debug)]
pub enum Commands {
    DetermineTopology(DetermineTopologyArgs),

    Launch(LaunchArgs),
    LaunchMulti(LaunchMultiArgs),
}

#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
pub struct Cli {
    #[arg(long, default_value_t=DEFAULT_NODEID_LENGTH)]
    pub nodeid_length: u8,

    #[arg(long, default_value_t=DEFAULT_MESH_X)]
    pub mesh_x: u16,
    #[arg(long, default_value_t=DEFAULT_MESH_Y)]
    pub mesh_y: u16,

    #[arg(long, default_value_t=DEFAULT_CORES_PER_DSU)]
    pub cores_per_dsu: u8,

    #[arg(long, value_delimiter = ',')]
    pub events: Option<Vec<String>>,

    #[arg(long, default_value_t=String::from("data"))]
    pub outdir: String,

    #[command(subcommand)]
    pub command: Commands
}