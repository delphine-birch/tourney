use crate::register::*;
use crate::*;

pub fn single_elim_binary_tourney(n: u32) -> Tourney<2, 3> {
    let num = (2 as u32).pow(n);
    let mut tourn = Tourney::<2, 3>::new();
    let mut n = 1;
    let mut prev = Vec::new();
    while n < num {
        match n {
            1 => {
                prev = Vec::new();
                prev.push(tourn.add_match([MatchOutcome::Position(0), MatchOutcome::Position(1)], None));
                prev.push(tourn.add_match([MatchOutcome::Position(2), MatchOutcome::Discard], None));
            },
            2 => {
                let m = prev[0];
                let m0 = prev[1];
                prev = Vec::new();
                prev.push(tourn.add_match([MatchOutcome::Match(m, 0), MatchOutcome::Match(m0, 0)], None));
                prev.push(tourn.add_match([MatchOutcome::Match(m, 1), MatchOutcome::Match(m0, 1)], None));
            }
            _ => {
                let m = prev.clone();
                prev = Vec::new();
                for m0 in m {
                    prev.push(tourn.add_match([MatchOutcome::Match(m0, 0), MatchOutcome::Discard], None));
                    prev.push(tourn.add_match([MatchOutcome::Match(m0, 1), MatchOutcome::Discard], None));
                }
            }
        }
        n *= 2;
    }
    return tourn;
}

pub struct RandomTourneyRunner { prob: f32, }
impl RandomTourneyRunner { pub fn new(p: f32) -> Self { let prob = p; Self { prob } }}
impl TourneyRunner for RandomTourneyRunner {
    fn step<const NP: usize>(&mut self, indices: &[u32], _competitors: &mut Register<Competitor>) -> [u32; NP] {
        let mut ret = [0; NP];
        for i in 0..NP {
            ret[i] = indices[i];
        }
        let r = rand::random::<f32>();
        if r < self.prob { let s = ret[0]; ret[0] = ret[1]; ret[1] = s; }
        ret
    }
}