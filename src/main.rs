use std::collections::HashMap;

use clap::{App, Arg, SubCommand};

use crate::playground::circle::run as circle_run;
use crate::playground::clock::run as clock_run;
use crate::playground::projectile::run as projectile_run;
use crate::playground::sphere::run as sphere_run;

mod tuple;
mod color;
mod canvas;
mod matrix;
mod transformation;
mod ray;
mod light;
mod material;
mod sphere;
mod intersection;
mod util;
mod playground;

fn main() {
    let matches = App::new("Ray Tracer Challenge")
        .version("0.1")
        .author("Tobias S. <tbs.schndlr@gmail.com>")
        .about("Ray tracer implementation by following the book from Jamis Buck")
        .subcommand(SubCommand::with_name("all")
            .about("Renders final results from all chapter results")
            .version("0.1"))
        .subcommand(SubCommand::with_name("projectile")
            .about("Renders final results from all chapter results")
            .version("0.1"))
        .subcommand(SubCommand::with_name("clock")
            .about("Renders final results from all chapter results")
            .version("0.1"))
        .subcommand(SubCommand::with_name("circle")
            .about("Renders final results from all chapter results")
            .version("0.1"))
        .subcommand(SubCommand::with_name("sphere")
            .about("Renders final results from all chapter results")
            .version("0.1"))
        .get_matches();

    let (command_name, command_args) = matches.subcommand();

    match Command::from_string(command_name) {
        Command::All => {
            std::thread::spawn(|| projectile_run()).join();
            std::thread::spawn(|| clock_run()).join();
            std::thread::spawn(|| circle_run()).join();
            std::thread::spawn(|| sphere_run()).join();
        }
        Command::Projectile => projectile_run(),
        Command::Clock => clock_run(),
        Command::Circle => circle_run(),
        Command::Sphere => sphere_run(),
    }

//    projectile_run();
//    println!();
//    clock_run();
//    println!();
//    circle_run();
//    println!();
//    sphere_run();
//    playground::sphere_advanced::run();
}

#[derive(Debug)]
enum Command {
    All,
    Projectile,
    Clock,
    Circle,
    Sphere,
}

impl Command {
    fn from_string(command: &str) -> Command {
        let mut map = HashMap::new();

        map.insert("all", Command::All);
        map.insert("projectile", Command::Projectile);
        map.insert("clock", Command::Clock);
        map.insert("circle", Command::Circle);
        map.insert("sphere", Command::Sphere);

        match map.remove(command) {
            Some(c) => c,
            None => Command::All,
        }
    }
}
