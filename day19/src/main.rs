//! day19 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use slab_tree::tree::TreeBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Range;
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

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug)]
struct PartRanges {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    part: PartRanges,
    #[allow(dead_code)]
    sum: usize, // Just used for debugging
}

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

    let mut do_parts = false;
    let mut parts = vec![];
    let mut workflows = HashMap::new();

    for (line_num, line) in lines.iter().enumerate() {
        // A blank line means we're done with rules and now have parts to process.
        if line.is_empty() {
            do_parts = true;
            continue;
        }

        // Parts are simpler to parse. Just take each entry and split it up.
        //
        // e.g. - {x=787,m=2655,a=1222,s=2876}
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
            // Workflows are a bit terser.
            // They have a name and then 1..N parts describing a path to work on and a condition for passing that way.
            //
            // e.g. - px{a<2006:qkq,m>2090:A,rfg}
            //
            // This is the px rule. If a is less than 2006 move to qkq. Otherwise if m is greater than 2090 accept the part.
            // Finally just go to rfg.
            // This can be N long but only one "default" at the end which is "A" (accept), "R" (reject) or a rule name.
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

    // Make sure this has the start key.
    assert!(
        workflows.contains_key("in"),
        "Workflows doesn't contain 'in' key?: - {workflows:?}"
    );

    // Dump out the input for debugging.
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

    // For part1 it's easy. Loop over each part and if it's acceptable add up
    // it's component values to the overall sum.
    let mut sum = 0;
    for p in &parts {
        if acceptable(p, &workflows) {
            sum += p.x + p.m + p.a + p.s;
        }
    }
    println!("part1: {sum}");

    // Part2 gets a lot more complex as we now have a bin of 4000*4000*4000*4000 parts
    // and want to know based just on our workflows which of those would be
    // valid to accept.
    //
    // Numbers alone tell us we can't run acceptable() above as that would take
    // months/years to compute even distributed (or cost a fortune).
    //
    // But...we don't need all of that. In the end create a tree showing all the
    // rule destinations. Along each step we track the valid range of parts that let us progress
    // and adjusting it for each step down.
    //
    // i.e. for the sample rules:
    //
    // px{a<2006:qkq,m>2090:A,rfg}
    // pv{a>1716:R,A}
    // lnx{m>1548:A,A}
    // rfg{s<537:gd,x>2440:R,A}
    // qs{s>3448:A,lnx}
    // qkq{x<1416:A,crn}
    // crn{x>2662:A,R}
    // in{s<1351:px,qqz}
    // qqz{s>2770:qs,m<1801:hdj,R}
    // gd{a>3333:R,R}
    // hdj{m>838:A,pv
    //
    // Moving from in->px->A requires
    // s=1..1351,a=2006..4001,m=2091..4001,x=1..4001
    //
    // The s is obvious at that's the in rule for it. But for a we have to make sure
    // and invert the condition which would have passed to qkq when we continue along to m for testing.

    // Create a tree rooted at in and retain it's id.
    // That tree has a node which has the current name, a part range showing current valid ranges to get there
    // and a sum of those (for debugging).
    // Create a worklist of (parent_id, workflows) where workflows is the vector of all workflows attached to that
    // node (i.e. for "in" we have 2 for destination px and qqz)
    // Then start a loop while the worklist still has items.
    //
    // Pop the top item and make a copy of it's part range as it will shrink for each workflow processed.
    // Then loop over all the workflows and look at the test indicated and adjust the range accordingly
    // (off by one is easy here...that's why sum is in the Node struct).
    // Anything that isn't terminal (A/R) gets new work pushed onto the main work list as we add new nodes to the tree.
    // Effectively building this breadth first.
    let cur_part = PartRanges {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };
    let mut tree = TreeBuilder::new()
        .with_root(Node {
            name: "in",
            sum: (cur_part.x.end - cur_part.x.start)
                * (cur_part.m.end - cur_part.m.start)
                * (cur_part.a.end - cur_part.a.start)
                * (cur_part.s.end - cur_part.s.start),
            part: cur_part.clone(),
        })
        .build();

    let root_id = tree.root_id().unwrap();
    let mut cur_id = root_id;
    let mut work = vec![];
    work.push((cur_id, workflows.get("in").unwrap()));

    while let Some(w) = work.pop() {
        // Get the parent node we need to attach onto.
        let mut e = tree.get_mut(w.0).unwrap();

        // For each workflow compute the new part range needed to get to it
        let mut p = e.data().part.clone();
        for wf in w.1 {
            let mut m = match wf.dimension {
                "x" => p.x.clone(),
                "m" => p.m.clone(),
                "a" => p.a.clone(),
                "s" => p.s.clone(),
                "" => 0..0,
                _ => panic!(),
            };
            let mut n = m.clone();
            match wf.op {
                Op::Greater => {
                    // NOTE: The +1 is hard to miss and will get wrong results.
                    m = wf.test + 1..m.end;
                    // This is only here to shut up the warning about "an inclusive range would be better".
                    // Since these aren't interchangable that's actually BS and wrong.
                    let end = wf.test + 1;
                    n = n.start..end;
                }
                Op::Less => {
                    m = m.start..wf.test;
                    n = wf.test..n.end;
                }
                Op::None => {}
            }
            // For no-ops nothing actually changes so no bother reassigning.
            // This whole thing could likely use a cleanup.
            if wf.op != Op::None {
                match wf.dimension {
                    "x" => p.x = m,
                    "m" => p.m = m,
                    "a" => p.a = m,
                    "s" => p.s = m,
                    _ => panic!(),
                }
            }

            // Make a new node, record it's id.
            cur_id = e
                .append(Node {
                    name: wf.destination,
                    sum: (p.x.end - p.x.start)
                        * (p.m.end - p.m.start)
                        * (p.a.end - p.a.start)
                        * (p.s.end - p.s.start),
                    part: p.clone(),
                })
                .node_id();

            // Now adjust for next loop (assuming non-terminal) and set p now
            // to the state if the test failed to match. This doesn't matter if
            // we're on the last stage of the loop but doesn't hurt.
            if wf.op != Op::None {
                match wf.dimension {
                    "x" => p.x = n,
                    "m" => p.m = n,
                    "a" => p.a = n,
                    "s" => p.s = n,
                    _ => panic!(),
                }
            }

            // These are terminal so no more work to push on. Otherwise it refers
            // to another flow so push that id in as well as all the workflows for that node.
            if wf.destination != "R" && wf.destination != "A" {
                work.push((cur_id, workflows.get(wf.destination).unwrap()));
            }
        }
    }

    // Debug print the whole tree. This is where sum helps since it's each to see
    // the decreases at each level and hand verify if needed (certainly helped me debug).
    if args.debug {
        let mut s = String::new();
        tree.write_formatted(&mut s)?;
        println!("{s}");
    }

    // Traverse the tree from the bottom and find the "A" nodes (nothing else matters).
    // Those nodes will have the values we want to add to our sum.
    let mut sum = 0;
    for node in tree.root().unwrap().traverse_pre_order() {
        let n = node.data();
        // Only find the ends which end in an accept.
        if n.name != "A" {
            continue;
        }
        let p = &n.part;
        let mini = (p.x.end - p.x.start)
            * (p.m.end - p.m.start)
            * (p.a.end - p.a.start)
            * (p.s.end - p.s.start);

        if args.debug {
            // For debugging walk back up, find the parents to make a list of the path to get here.
            // Then reverse it so we can pretty print it out.
            let mut entries = vec!["A"];
            for a in node.ancestors() {
                entries.push(a.data().name);
            }
            entries.reverse();
            for e in entries.iter().take(entries.len() - 1) {
                print!("{e} -> ");
            }
            print!("{} = ", entries.last().unwrap());
            println!("{p:?} - {mini}");
        }

        sum += mini;
    }
    println!("part2: {sum}");
    Ok(())
}

// Return true if the given part after the workflow run is acceptable.
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
