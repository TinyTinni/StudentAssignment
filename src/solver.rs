extern crate serde;
extern crate serde_json;
extern crate z3;

use z3::ast::*;
use z3::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Attendee {
    #[serde(skip)]
    pub id: usize,
    pub name: std::string::String,
    pub wishlist: Vec<usize>,
}
#[derive(Deserialize)]
pub struct Timeslot {
    #[serde(skip)]
    pub id: usize,
    pub name: std::string::String,
    pub max_capacity: usize,
}

pub struct AssignmentTable<'a> {
    pub attendees: Vec<Attendee>,
    pub timeslots: Vec<Timeslot>,
    pub assignments: Vec<Int<'a>>,
}

impl<'a> AssignmentTable<'a> {
    pub fn assignments_per_timeslot(
        &'a self,
        timeslot: &Timeslot,
    ) -> impl Iterator<Item = &'a Int<'a>> {
        self.assignments
            .iter()
            .skip(timeslot.id)
            .step_by(self.attendees.len())
    }
    pub fn assignments_per_attendee(
        &'a self,
        attendee: &Attendee,
    ) -> impl Iterator<Item = &'a Int<'a>> {
        self.assignments
            .iter()
            .skip(attendee.id * self.attendees.len())
            .take(self.timeslots.len())
    }

    pub fn from_json<Buffer: std::io::Read>(
        ctx: &'a Context,
        solver: &Solver,
        reader: Buffer,
    ) -> Result<AssignmentTable<'a>, serde_json::Error> {
        let mut json: serde_json::Value = serde_json::from_reader(reader)?;

        let mut timeslots: Vec<Timeslot> = serde_json::from_value(json["timeslots"].take())?;
        for (i, t) in timeslots.iter_mut().enumerate() {
            t.id = i;
        }

        let mut attendees: Vec<Attendee> = serde_json::from_value(json["students"].take())?;
        let mut assignments = Vec::with_capacity(attendees.len() * timeslots.len());

        //adding a rules here already
        let bool_constrainer = BoolConstrainer::new(&ctx);
        for (a_i, a) in attendees.iter_mut().enumerate() {
            for t in timeslots.iter() {
                a.id = a_i;
                let expr = Int::new_const(&ctx, format!("a_{}_{}", a_i, t.id));

                if !a.wishlist.contains(&t.id) {
                    bool_constrainer.zero(&solver, &expr);
                } else {
                    bool_constrainer.zero_or_one(&solver, &expr);
                }
                assignments.push(expr);
            }
        }

        Ok(AssignmentTable {
            attendees,
            timeslots,
            assignments,
        })
    }

    pub fn eq_visits(&self, ctx: &'a Context, solver: &Solver, min_visits: u64) {
        let min = Int::from_u64(&ctx, min_visits);

        for a in self.attendees.iter() {
            let row: Vec<_> = self.assignments_per_attendee(a).collect();
            let sum = Int::add(&ctx, &row);
            solver.assert(&sum._eq(&min));
        }
    }

    pub fn max_attendees(&self, ctx: &'a Context, solver: &Solver) {
        for t in &self.timeslots {
            let timeslot_capacity = Int::from_u64(&ctx, t.max_capacity as u64);
            let row: Vec<_> = self.assignments_per_timeslot(t).collect();
            let sum = Int::add(&ctx, &row);
            solver.assert(&sum._eq(&timeslot_capacity));
        }
    }
}

struct BoolConstrainer<'a> {
    one: Int<'a>,
    zero: Int<'a>,
}

impl<'a> BoolConstrainer<'a> {
    fn new(ctx: &Context) -> BoolConstrainer {
        BoolConstrainer {
            one: Int::from_u64(&ctx, 1),
            zero: Int::from_u64(&ctx, 0),
        }
    }
    fn zero_or_one(&self, solver: &Solver, ast: &Int) {
        solver.assert(&ast.ge(&self.zero));
        solver.assert(&ast.le(&self.one));
    }
    fn zero(&self, solver: &Solver, ast: &Int) {
        solver.assert(&ast._eq(&self.zero));
    }
    fn _one(&self, solver: &Solver, ast: &Int) {
        solver.assert(&ast._eq(&self.one));
    }
}
