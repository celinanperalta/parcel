use clap::{Parser, Subcommand};
use std::{
  fs::{remove_file, OpenOptions},
  io::Write,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ParcelScaffold {
  ///command
  #[command(subcommand)]
  plugin: Plugins,

  ///Name of plugin
  #[arg(global = true)]
  name: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Plugins {
  ///Create namer
  Namer {},
  Optimizer {},
  Packager {},
  Reporter {},
  Resolver {},
  Transformer {},
  Bundler {},
}

fn main() {
  let args = ParcelScaffold::parse();
  let name = args.name;

  if let Some(name) = name {
    println!("{:?}", name);
  };

  match args.plugin {
    Plugins::Namer {} => todo!(),
    Plugins::Optimizer {} => todo!(),
    Plugins::Packager {} => todo!(),
    Plugins::Reporter {} => todo!(),
    Plugins::Resolver {} => todo!(),
    Plugins::Transformer {} => todo!(),
    Plugins::Bundler {} => todo!(),
  }
}
