//! day19 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::Display;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Debug, Default)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Default, Display, PartialEq)]
enum Op {
    #[default]
    None,
    Greater,
    Less,
}

#[derive(Debug, Default)]
struct Workflow<'a> {
    dimension: &'a str,
    op: Op,
    test: usize,
    destination: &'a str,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    let mut do_parts = false;
    let mut parts = vec![];
    let mut workflows = HashMap::new();

    for (line_num, line) in lines.iter().enumerate() {
        if line.is_empty() {
            do_parts = true;
            continue;
        }
        if do_parts {
            let l = line.as_str();
            let l = &l[1..l.len() - 1];
            let ps = l.split(',').collect::<Vec<_>>();
            assert!(ps.len() == 4, "bad parts line {} - {line}", line_num + 1);
            let mut part = Part::default();
            for pt in ps {
                let v = pt[2..].parse::<usize>().unwrap();
                match &pt[0..1] {
                    "x" => part.x = v,
                    "m" => part.m = v,
                    "a" => part.a = v,
                    "s" => part.s = v,
                    _ => panic!("bad parts line {} - {line}", line_num + 1),
                }
            }
            parts.push(part);
        } else {
            let parts = line.split('{').collect::<Vec<_>>();

            let rules = parts[1][0..parts[1].len() - 1]
                .split(',')
                .collect::<Vec<_>>();
            let mut flows = vec![];
            for r in rules {
                let mut workflow = Workflow::default();
                // No : means it's just a destination rule
                if !r.contains(':') {
                    workflow.destination = r;
                    flows.push(workflow);
                    continue;
                }
                // a<2006:qkq
                let rl = r.split(':').collect::<Vec<_>>();
                assert!(rl.len() == 2, "bad rules line {} - {line}", line_num + 1);
                workflow.dimension = &rl[0][0..1];
                workflow.destination = rl[1];
                workflow.test = rl[0][2..].parse::<usize>().unwrap();
                match &rl[0][1..2] {
                    "<" => workflow.op = Op::Less,
                    ">" => workflow.op = Op::Greater,
                    _ => panic!("bad rules line {} - {line}", line_num + 1),
                }
                flows.push(workflow);
            }
            assert!(
                workflows.insert(parts[0], flows).is_none(),
                "Key {} already exists for line {} - {line}",
                parts[0],
                line_num + 1
            );
        }
    }

    assert!(
        workflows.contains_key("in"),
        "Workflows doesn't contain 'in' key?: - {workflows:?}"
    );
    if args.debug {
        println!("Workflows:");
        for (k, v) in &workflows {
            println!("{k} -> {v:?}");
        }
        println!("\nParts:");
        for p in &parts {
            println!("{p:?}");
        }
    }

    let mut sum = 0;
    for p in &parts {
        if acceptable(p, &workflows) {
            sum += p.x + p.m + p.a + p.s;
        }
    }
    println!("part1: {sum}");
    Ok(())
}

// Return true if the given part after the workflow run is acceptable.
#[allow(clippy::too_many_lines)]
fn acceptable(part: &Part, workflows: &HashMap<&str, Vec<Workflow>>) -> bool {
    let mut cur = workflows.get("in").unwrap();
    'outer: loop {
        for w in cur {
            let mut check_dest = false;
            match w.op {
                Op::None => {
                    // No op means this is just a destination which could be reject, accept or a new flow to start.
                    check_dest = true;
                }
                Op::Greater | Op::Less => {
                    let p = match w.dimension {
                        "x" => part.x,
                        "m" => part.m,
                        "a" => part.a,
                        "s" => part.s,
                        _ => panic!("invalid dimension {}", w.dimension),
                    };
                    if w.op == Op::Greater && p > w.test {
                        check_dest = true;
                    }
                    if w.op == Op::Less && p < w.test {
                        check_dest = true;
                    }
                }
            }
            if check_dest {
                match w.destination {
                    "R" => return false,
                    "A" => return true,
                    _ => {
                        cur = workflows.get(w.destination).unwrap();
                        continue 'outer;
                    }
                }
            }
        }
        break;
    }
    panic!("Workflows fell off the end?");
}
