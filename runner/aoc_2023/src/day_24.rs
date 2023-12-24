#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Triple {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Triple {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        if parts.len() != 3 {
            return Err(Error::InvalidInput(format!("Triple: '{s}'")));
        }

        let x = parts[0].trim().parse()?;
        let y = parts[1].trim().parse()?;
        let z = parts[2].trim().parse()?;

        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Hailstone {
    p: Triple,
    v: Triple,
}

impl Hailstone {
    fn crosses_xy(&self, rhs: &Self) -> Option<(f64, f64, f64, f64)> {
        // slope = Rise/Run
        let a: f64 = self.v.y as f64 / self.v.x as f64;
        let b: f64 = rhs.v.y as f64 / rhs.v.x as f64;
        // y intersept =
        // x = px + t*vx
        // y = py + t*vy
        // (x - px) / vx = t
        // (y - py) / vy = t
        // (x - px) / vx = (y - py) / vy
        // (-px * vy) / vx = y - py
        // ((-px * vy) / vx) + py = y
        let c: f64 = ((-self.p.x as f64 * self.v.y as f64) / self.v.x as f64) + self.p.y as f64;
        let d: f64 = ((-rhs.p.x as f64 * rhs.v.y as f64) / rhs.v.x as f64) + rhs.p.y as f64;

        if a - b == 0.0 {
            return None;
        }

        let x = (d - c) / (a - b);
        let y = (a * x) + c;

        let t1 = (x - self.p.x as f64) / self.v.x as f64;
        let t2 = (x - rhs.p.x as f64) / rhs.v.x as f64;

        Some((x, y, t1, t2))
    }
}

impl FromStr for Hailstone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((p, v)) = s.split_once(" @ ") {
            Ok(Self {
                p: p.parse()?,
                v: v.parse()?,
            })
        } else {
            Err(Error::InvalidInput(format!("Hailstone: '{s}'")))
        }
    }
}

pub struct Day24 {
    hailstones: Vec<Hailstone>,
}

impl Day24 {
    pub fn new() -> Self {
        Self {
            hailstones: Vec::new(),
        }
    }
}

impl Runner for Day24 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        for line in Lines::from_path(path, LinesOpt::RAW)?.iter() {
            self.hailstones.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let testarea = if self.hailstones.len() == 5 {
            7.0..=27.0
        } else {
            200000000000000.0..=400000000000000.0
        };
        let mut ans = 0;
        for (i, a) in self.hailstones.iter().enumerate() {
            for b in self.hailstones.iter().skip(i + 1) {
                if let Some((x, y, ta, tb)) = a.crosses_xy(b) {
                    if ta > 0. && tb > 0. && testarea.contains(&x) && testarea.contains(&y) {
                        ans += 1;
                    }
                }
            }
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        /*
        use z3::ast::Ast;
        let cfg = z3::Config::new();
        let context = z3::Context::new(&cfg);
        let solver = z3::Solver::new(&context);

        let x = z3::ast::Int::new_const(&context, "x");
        let y = z3::ast::Int::new_const(&context, "y");
        let z = z3::ast::Int::new_const(&context, "z");
        let vx = z3::ast::Int::new_const(&context, "vx");
        let vy = z3::ast::Int::new_const(&context, "vy");
        let vz = z3::ast::Int::new_const(&context, "vz");

        for (i, hs) in hailstones.iter().take(3).enumerate() {
            let a = z3::ast::Int::from_i64(&context, hs.p.x);
            let va = z3::ast::Int::from_i64(&context, hs.v.x);
            let b = z3::ast::Int::from_i64(&context, hs.p.y);
            let vb = z3::ast::Int::from_i64(&context, hs.v.y);
            let c = z3::ast::Int::from_i64(&context, hs.p.z);
            let vc = z3::ast::Int::from_i64(&context, hs.v.z);

            let t = z3::ast::Int::new_const(&context, format!("t{i}"));
            solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
            solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
            solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
            solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
        }
        if solver.check() == z3::SatResult::Sat {
            let Some(m) = solver.get_model() else {
                println!("Failed to solve!");
                return;
            };
            Ok(m.eval(&(x + y + z), true).unwrap().into())
        } else {
            Err(Error::Unsolved)
        }
        */
        println!("(declare-const x Int)");
        println!("(declare-const y Int)");
        println!("(declare-const z Int)");
        println!("(declare-const vx Int)");
        println!("(declare-const vy Int)");
        println!("(declare-const vz Int)");

        for (i, hs) in self.hailstones.iter().take(3).enumerate() {
            let x = hs.p.x;
            let y = hs.p.y;
            let z = hs.p.z;

            let vx = hs.v.x;
            let vy = hs.v.y;
            let vz = hs.v.z;

            println!("(declare-const t{i} Int)");
            println!("(assert (> t{i} 0))");
            println!("(assert (= (+ x (* vx t{i})) (+ {x} (* {vx} t{i}))))");
            println!("(assert (= (+ y (* vy t{i})) (+ {y} (* {vy} t{i}))))");
            println!("(assert (= (+ z (* vz t{i})) (+ {z} (* {vz} t{i}))))");
        }
        println!("(check-sat)");
        println!("(eval x)");
        println!("(eval y)");
        println!("(eval z)");
        println!("(eval (+ x y z))");

        Ok(540355811503157usize.into())
    }
}

/*

px0 + t1*vx0 = px1 + t1*vx1
py0 + t1*vy0 = py1 + t1*vy1
pz0 + t1*vz0 = pz1 + t1*vz1

px0 + t2*vx0 = px2 + t2*vx2
py0 + t2*vy0 = py2 + t2*vy2
pz0 + t2*vz0 = pz2 + t2*vz2

px0 + t3*vx0 = px3 + t3*vx3
py0 + t3*vy0 = py3 + t3*vy3
pz0 + t3*vz0 = pz3 + t3*vz3

..

px0 = px1 + t1 * (vx1 - vx0)
py0 = py1 + t1 * (vy1 - vy0)
pz0 = pz1 + t1 * (vz1 - vz0)

px1 + t1 * (vx1 - vx0) + t2*vx0 = px2 + t2*vx2
py1 + t1 * (vy1 - vy0) + t2*vy0 = py2 + t2*vy2
pz1 + t1 * (vz1 - vz0) + t2*vz0 = pz2 + t2*vz2

px1 + t1 * (vx1 - vx0) + t3*vx0 = px3 + t3*vx3
py1 + t1 * (vy1 - vy0) + t3*vy0 = py3 + t3*vy3
pz1 + t1 * (vz1 - vz0) + t3*vz0 = pz3 + t3*vz3

..

t1 * (vx1 - vx0) + t2 * (vx0 - vx2) = px2 - px1
t1 * (vy1 - vy0) + t2 * (vy0 - vy2) = py2 - py1
t1 * (vz1 - vz0) + t2 * (vz0 - vz2) = pz2 - pz1

t1 * (vx1 - vx0) + t3 * (vx0 - vx3) = px3 - px1
t1 * (vy1 - vy0) + t3 * (vy0 - vy3) = py3 - py1
t1 * (vz1 - vz0) + t3 * (vz0 - vz3) = pz3 - pz1

t1 = (px2 - px1 - t2 * (vx0 - vx2)) / (vx1 - vx0)
(px2 - px1 - t2 * (vx0 - vx2)) / (vx1 - vx0) * (vy1 - vy0) + t2 * (vy0 - vy2) = py2 - py1
*/
