// TODO: check clippy lints that can get cleaned up
// TODO: improve error handling
// TODO: add ability to pass multiple dxf files insead of just a single one
// TODO: add in logging?
// TODO: add support for missing entities
// TODO: Add in some unit tests
// TODO: See if I can find some open licences DXF files I could use to test against
//      * https://github.com/GSStnb/dxfBlocks
//      * https://people.math.sc.edu/Burkardt/data/dxf/dxf.html
//      * https://github.com/jscad/sample-files/tree/master
// TODO: See if i can add any parallelization.
// NOTE: The dxf crate hasn't had an update in 3 years, I was wondering if there were any other crates worth updating to.
//       it looks like the answer is no...but the crate isn't completely dead. it hasn't had a release in 3 years, but had
//       some commits about 5 months ago.
// NOTE: simple-xml-builder also hasn't had any updates in 3 years (or commits) but a straight forward api. Not sure if this is worth
//       swapping out for a more maintained crate, but it could be worth looking into
// NOTE: the bspline crate hasn't had a commit in 2 years, but once again, might not be worth trying to swap. There is stroke which is
//       slightly more up to date, but has a bunch of extra features that I'm not sure are needed. There is a bsplines crate built on
//       nalgebra, but looks to be very early stages. What about Kurbo? Part of the Xilem project, activly maintained, but i'm not sure
//       how stable it is, or if it can actually do what's needed. could be worth looking into.
//TODO: looking more closely at the way this is written, I don't feel passing in mutable int's for counts and a mutable XMLElement for updating
//      is really the best way to do this....probably better to have the add function return a Result<(XMLElement, count), Err, So it's either
//      returning an error (because there are none or some other failure?) or a count of how many elements plus the element....wait the count
//      is only ever incremented by 1...why is it inside these functions in the first place.....just return a Result<XMLElement, Err> if it
//      returns an element increment the count...

#![warn(
    clippy::all,
    clippy::pedantic,
    //clippy::cargo,
    rust_2024_compatibility,
)]

extern crate dxf;
extern crate simple_xml_builder;
extern crate unicode_segmentation;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use dxf::entities::EntityType;
use dxf::Drawing;
use qelmt::Definition;
use simple_xml_builder::XMLElement;
use std::path::PathBuf;
use std::time::Instant;
//use rayon::prelude::*;
mod qelmt;

#[derive(Parser, Debug)]
#[command(name = "dxf2elmt")]
#[command(author, version, about = "A CLI program to convert .dxf files into .elmt files", long_about = None)]
struct Args {
    /// The .dxf file to convert
    //#[clap(short, long, value_parser)]
    file_names: Vec<PathBuf>,

    /// Activates verbose output, eliminates .elmt file writing
    #[clap(short, long, value_parser, default_value_t = false)]
    verbose: bool,

    /// Converts text entities into dynamic text instead of the default text box
    #[clap(short, long, value_parser, default_value_t = false)]
    dtext: bool,

    /// Determine the number of lines you want each spline to have (more lines = greater resolution)
    #[clap(short, long, value_parser, default_value_t = 20)]
    spline_step: u32,

    /// Toggles information output... defaults to off
    #[clap(short, long, value_parser, default_value_t = false)]
    info: bool,
}

pub mod file_writer;

fn main() -> Result<()> {
    // Start recording time
    let now: Instant = Instant::now();

    // Collect arguments
    let args: Args = Args::parse_from(wild::args());

    // Load dxf file
    for file_name in args.file_names {
        let friendly_file_name = file_name.file_stem().unwrap().to_string_lossy();
        let drawing: Drawing = Drawing::load_file(&file_name).context(format!(
            "Failed to load {friendly_file_name}...\n\tMake sure the file is a valid .dxf file.",
        ))?;
        let q_elmt = Definition::new(friendly_file_name.clone(), args.spline_step, &drawing);
        if !args.verbose && args.info {
            println!("{friendly_file_name} loaded...");
        }

        // Intialize counts
        let mut circle_count: u32 = 0;
        let mut line_count: u32 = 0;
        let mut arc_count: u32 = 0;
        let mut spline_count: u32 = 0;
        let mut text_count: u32 = 0;
        let mut ellipse_count: u32 = 0;
        let mut polyline_count: u32 = 0;
        let mut lwpolyline_count: u32 = 0;
        let mut solid_count: u32 = 0;
        let mut block_count: u32 = 0;
        let mut other_count: u32 = 0;

        // Loop through all entities, counting the element types
        drawing.entities().for_each(|e| match e.specific {
            EntityType::Circle(ref _circle) => {
                circle_count += 1;
            }
            EntityType::Line(ref _line) => {
                line_count += 1;
            }
            EntityType::Arc(ref _arc) => {
                arc_count += 1;
            }
            EntityType::Spline(ref _spline) => {
                spline_count += 1;
            }
            EntityType::Text(ref _text) => {
                text_count += 1;
            }
            EntityType::Ellipse(ref _ellipse) => {
                ellipse_count += 1;
            }
            EntityType::Polyline(ref _polyline) => {
                polyline_count += 1;
            }
            EntityType::LwPolyline(ref _lwpolyline) => {
                lwpolyline_count += 1;
            }
            EntityType::Solid(ref _solid) => {
                solid_count += 1;
            }
            EntityType::Insert(ref _insert) => {
                block_count += 1;
            }
            _ => {
                other_count += 1;
            }
        });

        // Create output file for .elmt
        let out_file = file_writer::create_file(args.verbose, args.info, &file_name);

        // Write to output file
        XMLElement::from(&q_elmt)
            .write(&out_file)
            .context("Failed to write output file.")?;

        if args.info {
            println!("Conversion complete!\n");

            // Print stats
            println!("STATS");
            println!("~~~~~~~~~~~~~~~");
            println!("Circles: {circle_count}");
            println!("Lines: {line_count}");
            println!("Arcs: {arc_count}");
            println!("Splines: {spline_count}");
            println!("Texts: {text_count}");
            println!("Ellipses: {ellipse_count}");
            println!("Polylines: {polyline_count}");
            println!("LwPolylines: {lwpolyline_count}");
            println!("Solids: {solid_count}");
            println!("Blocks: {block_count}");
            println!("Currently Unsupported: {other_count}");

            println!("\nTime Elapsed: {} ms", now.elapsed().as_millis());
        }

        if args.verbose {
            file_writer::verbose_print(out_file);
        }
    }

    Ok(())
}
