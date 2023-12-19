use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::{DateTime, Local};

use crate::args::{Cli};
use crate::event::Event;

pub struct Writer {
    pub basepath: String,
    pub args: String,
    hostname: String,
    kernel: String,
    perf_version: String,
    datetime: DateTime<Local>,
    basedir: String,
}

impl Writer {
    pub fn new(basepath: String, args: &Cli, ) -> Writer {
        let datetime = Local::now();

        let mut w = Writer {
            basepath,
            args: format!("{:?}", args),
            hostname: String::from_utf8(Command::new("hostname").output().unwrap().stdout).unwrap().trim().parse().unwrap(),
            kernel: String::from_utf8(Command::new("uname").arg("-a").output().unwrap().stdout).unwrap().trim().parse().unwrap(),
            perf_version: String::from_utf8(Command::new("perf").arg("--version").output().unwrap().stdout).unwrap().trim().parse().unwrap(),
            datetime,
            basedir: "".to_string()
        };
        w.basedir = format!("measurements_{}_{}", w.hostname, w.datetime.format("%Y-%m-%dT%H%M").to_string());
        let mut out_path = PathBuf::from(w.basepath.as_str());
        out_path.push(Path::new(w.basedir.as_str()));
        if !out_path.is_dir() {
            fs::create_dir(out_path.as_path()).expect("Could not create measurement folder!");
        }
        w
    }
    pub fn write_events(&self, events: &Vec<Event>, event_type: &str, folder: Option<&str>) {
        let mut out_path = PathBuf::from(self.basepath.as_str());
        out_path.push(Path::new(self.basedir.as_str()));
        if let Some(_folder) = folder {
            out_path.push(_folder);
        }
        if !out_path.is_dir() {
            fs::create_dir(out_path.clone()).expect("Could not create directory");
        }
        out_path.push(format!("{event_type}.json"));
        fs::write(out_path.as_path(), serde_json::to_string(events).unwrap())
            .expect("Could not write events to file!");
    }

    pub fn write_lines(&self, lines: Vec<String>, fname: &str) {
        let mut out_path = PathBuf::from(self.basepath.as_str());
        out_path.push(Path::new(self.basedir.as_str()));
        if !out_path.is_dir() {
            fs::create_dir(out_path.clone()).expect("Could not create directory");
        }
        out_path.push(fname);
        fs::write(out_path.as_path(), lines.join("\n"))
            .expect("Could not write lines to file!");
    }

    pub fn write_meta(&self) {
        let mut out_path = PathBuf::from(self.basepath.as_str());
        out_path.push(Path::new(self.basedir.as_str()));
        out_path.push(format!("meta.md"));
        let content = format!("\
Measurement Start: `{}` (epoch: `{}`)
Host: `{}`
Kernel: `{}`
Perf Version: `{}`
All Args: `{}`",
                              self.datetime.format("%Y-%m-%dT%H%M%Z").to_string(),
                              self.datetime.format("%s"),
                              self.hostname,
                              self.kernel,
                              self.perf_version,
                              self.args);
        fs::write(out_path.as_path(), content).expect("Could not write meta file!");
    }
}