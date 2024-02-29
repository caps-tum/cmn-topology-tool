/* Write-to-disk helper, mainly for writing measurement results */

use std::fs;
use std::path::{PathBuf};
use std::process::Command;

use chrono::{DateTime, Local};

use crate::args::{Cli, Commands};
use crate::event::Event;

pub struct Writer {
    pub basepath: Option<String>,
    pub args: String,
    prefix: String,
    hostname: String,
    kernel: String,
    perf_version: String,
    datetime: DateTime<Local>,
    basedir: String,
    pub additional_args: String
}

impl Writer {
    pub fn new(basepath: Option<String>, args: &Cli) -> Writer {
        let datetime = Local::now();

        let prefix = match args.command {
            Commands::DetermineTopology(_)   => "determine_topology",
            Commands::Launch(_)         => "launch",
            Commands::LaunchMulti(_)    => "launch_multi",
        };
        let mut w = Writer {
            basepath,
            args: format!("{:?}", args),
            hostname: String::from_utf8(Command::new("hostname").output().unwrap().stdout).unwrap().trim().parse().unwrap(),
            kernel: String::from_utf8(Command::new("uname").arg("-a").output().unwrap().stdout).unwrap().trim().parse().unwrap(),
            perf_version: String::from_utf8(Command::new("perf").arg("--version").output().unwrap().stdout).unwrap().trim().parse().unwrap(),
            datetime,
            basedir: "".to_string(),
            prefix: prefix.to_string(),
            additional_args: "".to_string()
        };
        w.basedir = w.gen_basedir();

        w.create_outdir();
        w
    }

    fn gen_basedir(&self) -> String {
        format!("{}_{}", self.hostname, self.datetime.format("%Y-%m-%dT%H%M"))
    }
    pub fn get_outpath(&self) -> PathBuf {
        let mut out_path = PathBuf::from(self.basepath.clone().unwrap().as_str());
        out_path.push(self.prefix.as_str());
        out_path.push(self.basedir.as_str());
        out_path
    }

    fn create_outdir(&mut self) {
        if self.basepath.is_some() {
            let mut out_path = self.get_outpath();
            if out_path.is_dir() { // if already exists (two runs within one minute), then attach "-$i" to dirname until not exists
                let mut i = 1;
                let mut run = true;
                while run {
                    self.basedir = format!("{}-{i}", self.gen_basedir());
                    out_path = self.get_outpath();
                    run = out_path.is_dir();
                    i += 1;
                }
            }
            fs::create_dir_all(out_path.as_path()).expect("Could not create measurement folder!");
        }
    }

    pub fn write_events(&self, events: &Vec<Event>, event_type: &str, folder: Option<&str>) {
        if self.basepath.is_none() {
            log::debug!("Writer nas no basepath, will not write events.");
            return
        }

        let mut out_path = self.get_outpath();
        if let Some(_folder) = folder {
            out_path.push(_folder);

            if !out_path.is_dir() {
                fs::create_dir(out_path.clone()).expect("Could not create directory");
            }
        }

        out_path.push(format!("{event_type}.csv"));
        let mut csv_writer = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_writer(vec![]);
        events.iter().for_each(|e| csv_writer.serialize(e).expect("Could not serialize Event!"));
        fs::write(out_path.as_path(), String::from_utf8(csv_writer.into_inner().unwrap()).unwrap())
            .expect("Could not write events to file!");
    }

    pub fn write_lines(&self, lines: Vec<String>, fname: &str) {
        if self.basepath.is_none() {
            log::debug!("Writer nas no basepath, will not write lines.");
            return
        }

        let mut out_path = self.get_outpath();
        out_path.push(fname);
        fs::write(out_path.as_path(), lines.join("\n"))
            .expect("Could not write lines to file!");
    }

    pub fn write_meta(&self) {
        let now = Local::now();
        if self.basepath.is_none() {
            log::debug!("Writer nas no basepath, will not write meta.");
            return
        }

        let mut out_path = self.get_outpath();
        out_path.push(format!("meta.md"));
        let content = format!("\
Measurement Start: `{}` (epoch: `{}`)
Measurement End  : `{}` (epoch: `{}`)
Host: `{}`
Kernel: `{}`
Perf Version: `{}`
All Args: `{}`
Additional Args: `{}`
",
                              self.datetime.format("%Y-%m-%dT%H%M%S%Z").to_string(),
                              self.datetime.format("%s"),
                              now.format("%Y-%m-%dT%H%M%S%Z").to_string(),
                              now.format("%s"),
                              self.hostname,
                              self.kernel,
                              self.perf_version,
                              self.args,
                              self.additional_args);
        fs::write(out_path.as_path(), content).expect("Could not write meta file!");
    }
}