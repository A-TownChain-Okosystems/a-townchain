// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! R1CS-lite: Constraint-Check gegen Witness-Belegung (MVP).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    pub a: Vec<(usize, u64)>,
    pub b: Vec<(usize, u64)>,
    pub c: Vec<(usize, u64)>,
}

fn eval(lc: &[(usize, u64)], witness: &[u64]) -> u64 {
    lc.iter()
        .map(|(i, w)| w.wrapping_mul(witness[*i]))
        .fold(0u64, |acc, x| acc.wrapping_add(x))
}

pub fn satisfied(con: &Constraint, witness: &[u64]) -> bool {
    eval(&con.a, witness).wrapping_mul(eval(&con.b, witness)) == eval(&con.c, witness)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quadrat_constraint() {
        // x * x == x  (fuer x in {0,1})
        let con = Constraint { a: vec![(0, 1)], b: vec![(0, 1)], c: vec![(0, 1)] };
        assert!(satisfied(&con, &[1]));
        assert!(satisfied(&con, &[0]));
        assert!(!satisfied(&con, &[2]));
    }
}
