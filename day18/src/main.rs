//! day18 advent 20XX
use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use egui::{TextureHandle, TextureOptions};
use grid::Location;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = false)]
    debug_full: bool,

    #[arg(long, default_value_t = 3.0)]
    magnify: f32,
}

struct MyApp {
    texture: TextureHandle,
    magnify: f32,
}

// A color we know isn't an edge color by checking the input we can use as the
// flood fill instead.
const INTERIOR: &str = "#9F0000";

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    let mut locs = HashMap::new();
    let mut cur = Location(0, 0);
    let mut part_loc = Location(0, 0);
    locs.insert(cur.clone(), "#000000");
    let mut testlocs = vec![];

    let mut vertices = vec![part_loc.clone()];

    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 3, "Bad line {} - {line}", line_num + 1);

        let steps = parts[1].parse::<u32>().unwrap();
        let color = &parts[2][1..8];
        match *parts.first().unwrap() {
            "R" => {
                for _ in 0..steps {
                    cur = Location(cur.0 + 1, cur.1);
                    locs.insert(cur.clone(), color);
                }
            }
            "D" => {
                for _ in 0..steps {
                    cur = Location(cur.0, cur.1 + 1);
                    locs.insert(cur.clone(), color);
                }
            }
            "L" => {
                for _ in 0..steps {
                    cur = Location(cur.0 - 1, cur.1);
                    locs.insert(cur.clone(), color);
                }
            }
            "U" => {
                for _ in 0..steps {
                    cur = Location(cur.0, cur.1 - 1);
                    locs.insert(cur.clone(), color);
                }
            }
            _ => panic!("Bad line {} - {line}", line_num + 1),
        }

        let dist = isize::from_str_radix(&parts[2][2..7], 16).unwrap();
        match &parts[2][7..8] {
            // R
            "0" => {
                if args.debug {
                    println!("R {dist}");
                }
                part_loc.0 += dist;
            }
            // D
            "1" => {
                if args.debug {
                    println!("D {dist}");
                }
                part_loc.1 += dist;
            }
            // L
            "2" => {
                if args.debug {
                    println!("L {dist}");
                }
                part_loc.0 -= dist;
            }
            // U
            "3" => {
                if args.debug {
                    println!("U {dist}");
                }
                part_loc.1 -= dist;
            }
            _ => panic!("Bad line {} - {line}", line_num + 1),
        }
        vertices.push(part_loc.clone());
    }

    // Make sure it came back to the start.
    assert!(
        *vertices.last().unwrap() == Location(0, 0),
        "Polygon didn't go back to start? - {vertices:?}"
    );

    // Some easy constants we need later for various things and debugging.
    let min_x = locs.iter().map(|l| l.0 .0).min().unwrap();
    let max_x = locs.iter().map(|l| l.0 .0).max().unwrap();
    let min_y = locs.iter().map(|l| l.0 .1).min().unwrap();
    let max_y = locs.iter().map(|l| l.0 .1).max().unwrap();

    // Take all the neighbors of 0,0 and for each one cast to see if it's inside or outside.
    // We'll take all these and then just run a flood fill against that.
    for n in Location(0, 0).neighbors_all() {
        if raycast(&locs, min_x, max_x, min_y, max_y, &n, args.debug) {
            testlocs.push(n);
        }
    }
    if args.debug {
        display_map(&locs, args.magnify)?;
    }
    while let Some(t) = testlocs.pop() {
        for n in t.neighbors() {
            if !locs.contains_key(&n) {
                testlocs.push(n);
            }
        }
        locs.insert(t, INTERIOR);
        if args.debug_full {
            display_map(&locs, args.magnify)?;
        }
    }
    if args.debug {
        display_map(&locs, args.magnify)?;
    }
    println!("part1: {}", locs.len());

    if args.debug {
        println!("vertices:\n{vertices:?}");
    }

    println!("part2: {}", picks_theorem(&vertices, args.debug));
    Ok(())
}

// This uses the shoelace theorem to calculate the area inside the polygon.
// NOTE: This won't include the polygon itself. For that take this answer
//       and plug it into Pick's theorem.
fn shoelace_area(vertices: &[Location], debug: bool) -> i128 {
    let mut sum: i128 = 0;
    for i in 0..vertices.len() - 1 {
        let p0 = &vertices[i];
        let p1 = &vertices[i + 1];
        let area: i128 = (p0.0 * p1.1 - p0.1 * p1.0).try_into().unwrap();
        sum += area;
        if debug {
            println!("({p0},{p1}) = {area}");
        }
    }
    if debug {
        println!("Sum = {sum}");
    }
    sum /= 2;
    sum
}

fn picks_theorem(vertices: &[Location], debug: bool) -> i128 {
    let inside = shoelace_area(vertices, debug);
    let mut b: i128 = 0;
    for i in 0..vertices.len() - 1 {
        let p0 = &vertices[i];
        let p1 = &vertices[i + 1];
        let dist: i128 = ((p0.0 - p1.0) + (p0.1 - p1.1))
            .unsigned_abs()
            .try_into()
            .unwrap();
        if debug {
            println!("({p0},{p1}) = {dist}");
        }
        b += dist;
    }
    // The above never counts 2 points so add 4 here so the math works out.
    // Can't just add one at every intersection or you double count each one.
    b += 4;
    if debug {
        println!("inside: {inside} b: {b}");
    }
    inside + b / 2 - 1
}

// Visualize the map data into an image and use egui to toss up a window so we can see it.
fn display_map(locs: &HashMap<Location, &str>, mult: f32) -> Result<()> {
    let min_x = locs.iter().map(|l| l.0 .0).min().unwrap();
    let max_x = locs.iter().map(|l| l.0 .0).max().unwrap();
    let min_y = locs.iter().map(|l| l.0 .1).min().unwrap();
    let max_y = locs.iter().map(|l| l.0 .1).max().unwrap();

    #[allow(clippy::cast_sign_loss)]
    let width = Box::leak(Box::new((max_x - min_x + 1) as usize));
    #[allow(clippy::cast_sign_loss)]
    let height = Box::leak(Box::new((max_y - min_y + 1) as usize));
    let mult = Box::leak(Box::new(mult));

    let width_i = max_x - min_x + 1;
    println!(
        "Size: {}\n{}x{} ({},{}),({},{})",
        locs.len(),
        width,
        height,
        min_x,
        min_y,
        max_x,
        max_y,
    );

    // Set everything to grey
    let mut data = vec![0x3F_u8; *width * *height * 3];
    for l in locs {
        // Take each point, adjust to make them all positive and then paint those locations white.
        #[allow(clippy::cast_sign_loss)]
        let loc = (((l.0 .0 - min_x) + ((l.0 .1 - min_y) * width_i)) * 3) as usize;
        let r = u8::from_str_radix(&l.1[1..3], 16).unwrap();
        let g = u8::from_str_radix(&l.1[3..5], 16).unwrap();
        let b = u8::from_str_radix(&l.1[5..7], 16).unwrap();
        data[loc] = r;
        data[loc + 1] = g;
        data[loc + 2] = b;
    }

    let res = eframe::run_native(
        "Debug path",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(MyApp::new(cc, data, *width, *height, *mult))),
    );

    if let Err(e) = res {
        return Err(eyre!("EGUI error: {e:?}"));
    }
    Ok(())
}

impl MyApp {
    #[allow(clippy::needless_pass_by_value)]
    fn new(
        cc: &eframe::CreationContext<'_>,
        map: Vec<u8>,
        width: usize,
        height: usize,
        magnify: f32,
    ) -> Self {
        let im = egui::ColorImage::from_rgb([width, height], &map);
        let text = cc.egui_ctx.load_texture(
            "debug map",
            egui::ImageData::Color(im.into()),
            TextureOptions::default(),
        );

        MyApp {
            texture: text,
            magnify,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::GRAY))
            .show(ctx, |ui| {
                ui.add_space(10.0);

                // Make a direct image allows us to scale it up vs using ui.image()
                let mut s = self.texture.size_vec2();
                s[0] *= self.magnify;
                s[1] *= self.magnify;
                ui.add(egui::Image::from_texture(&self.texture).fit_to_exact_size(s));
            });
        frame.set_window_size(ctx.used_size());
    }
}

// Test whether a given point is inside or outside the enclousure
fn raycast(
    locs: &HashMap<Location, &str>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    test: &Location,
    debug: bool,
) -> bool {
    if !locs.contains_key(test) {
        if debug {
            println!("Testing {test}");
        }
        let mut c = 0;
        let mut cur = test.clone();
        let mut old;
        loop {
            // Don't go outside the overall bounds.
            if cur.0 < min_x || cur.0 > max_x || cur.1 < min_y || cur.1 > max_y {
                break;
            }
            // Just go one direction but a string of the same is one point, not N.
            old = cur.clone();
            cur = Location(cur.0 + 1, cur.1);
            if locs.contains_key(&cur) && !locs.contains_key(&old) {
                if debug {
                    println!("Intersected at {cur}");
                }
                c += 1;
            }
        }

        // If odd we're inside
        if c % 2 != 0 {
            if debug {
                println!("inside");
            }
            return true;
        }
    }
    false
}
