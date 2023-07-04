extern crate clap;

use clap::Parser;
use std::fs::File;
use std::path::PathBuf;
use z3::*;

mod solver;
/// Assigns students to timelots with specific a criteria.
///
/// Current criteria: Every student has to visit exactly "visits" many time slots
#[derive(clap::Parser, Debug)]
#[command(author, version, about)]
struct Opt {
    /// json input file path
    #[arg(short, long)]
    input: PathBuf,

    /// the amount of timeslots every attendee has to visit
    #[arg(short, long, default_value = "1")]
    visits: u64,
}

fn main_opts<'a>(
    opt: &Opt,
    ctx: &'a Context,
) -> Result<(Model<'a>, solver::AssignmentTable<'a>), Box<dyn std::error::Error>> {
    let solver = Solver::new(&ctx);

    let file = File::open(&opt.input).unwrap();
    let reader = std::io::BufReader::new(file);

    let table = solver::AssignmentTable::from_json(&ctx, &solver, reader)?;

    table.eq_visits(&ctx, &solver, opt.visits);
    table.max_attendees(&ctx, &solver);

    solver.check();
    let model = solver.get_model().unwrap();

    Ok((model, table))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let (model, table) = main_opts(&opt, &ctx)?;
    for a in table.attendees.iter() {
        let assignments = table.assignments_per_attendee(&a);
        if let Some((timeslot_id, _)) = assignments
            .enumerate()
            .find(|(_, x)| model.eval(*x, true).unwrap().as_i64().unwrap() == 1)
        {
            println!(" {} -> {}", a.name, table.timeslots[timeslot_id].name);
        } else {
            println!("{} -> Nothing found", a.name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn full_test() {
        use super::*;
        let opt = Opt {
            input: std::path::PathBuf::from("example.json"),
            visits: 1,
        };
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let result = main_opts(&opt, &ctx);
        assert!(result.is_ok());
        let (_model, _table) = result.unwrap();
        //todo: check if correctly assigned
    }
}
