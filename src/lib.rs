pub mod register;
pub mod testing;
use crate::register::{Register, Identify};
use std::{error::Error, fmt};
use std::collections::HashMap;

//COMPETITOR
pub struct Competitor {
    //ID FOR REGISTER
    pub id: u32,
    //STATS - UPDATED BY TOURNAMENT RUNNER PER MATCH
    pub _stats: Vec<f32>,
}

impl Competitor {
    pub fn new(v: Vec<f32>) -> Self { Self { id: 0, _stats: v } }
}
//IMPLEMENTING REGISTER TRAIT 
impl Identify for Competitor {
    fn get_id(&self) -> u32 { return self.id; }
    fn set_id(&mut self, i: u32) { self.id = i; }
}

//MATCH
#[derive(Clone)]
pub enum MatchOutcome {
    //OUTCOME - (a, b) COMPETITOR CONTINUES TO MATCH a, BECOMES PLAYER b
    Match(u32, u32),
    //OUTCOME - (a) COMPETITOR TAKES FINAL POSITION a
    Position(u32),
    //OUTCOME - COMPETITOR DOES NOT PLACE
    Discard,
}

#[derive(Clone)]
pub struct Match<const N: usize> {
    //ID FOR REGISTER
    pub id: u32,
    //STAGE - OPTIONAL, MATCHES OF STAGE X REQUIRE ALL MATCHES OF STAGE X-1 COMPLETE (IF STAGE IS EMPTY, AUTO COMPLETE)
    pub stage: Option<u32>,
    //MATCH COMPLETED - IMPLEMENTED SO MATCHES COMPETITOR LIST EXISTS AS RECORD OF OUTCOMES
    pub done: bool,
    //COMPETITOR INDICES FOR REGISTER
    pub competitors: [Option<u32>; N],
    //OUTCOMES
    pub outcomes: [MatchOutcome; N],
}
//IMPLEMENTING REGISTER TRAIT
impl<const N: usize> Identify for Match<N> {
    fn get_id(&self) -> u32 { self.id }
    fn set_id(&mut self, i: u32) { self.id = i; }
}

//TOURNEY
//STRUCTURE ERRORS - FOR CLEAR DEBUGGING OF INVALID TOURNAMENT STRUCTURES + RUNNERS
#[derive(Debug, Clone)]
pub enum StructureError {
    MatchOutcomeInvalid([u32; 6]),
    PositionOutcomeInvalid([u32; 4]),
    InputMatchingInvalid([u32; 2]),
}

impl Error for StructureError {}
impl fmt::Display for StructureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StructureError::MatchOutcomeInvalid(n) => {
                write!(f, "StructureError: Match Outcome was invalid: Match index {} Outcome index {} led to Match index {} Player index {} (Total Matches: {}, Total Players: {})", n[0], n[1], n[2], n[3], n[4], n[5])
            },
            StructureError::PositionOutcomeInvalid(n) => {
                write!(f, "StructureError: Position Outcome was invalid: Match index {} Outcome index {} led to Final Position index {} (Total Positions: {})", n[0], n[1], n[2], n[3])
            }
            StructureError::InputMatchingInvalid(n) => {
                write!(f, "StructureError: Number of Competitors and Inputs didn't match: {} Competitors, {} Inputs", n[0], n[1])
            }
        }
    }
}

#[derive(Clone)]
pub struct Tourney<const NP: usize, const NW: usize> {
    //MATCH REGISTER
    pub matches: Register<Match<NP>>,
    //FINAL POSITIONS (PLACEMENTS)
    pub positions: [Option<u32>; NW],
}

impl<const NP: usize, const NW: usize> Tourney<NP, NW>{
    pub fn new() -> Self {
        let matches = Register::new();
        let positions = [None; NW];
        Self { matches, positions }
    }

    //ADD MATCH TO REGISTER - DOES NOT VALIDATE ON ADD
    pub fn add_match(&mut self, outcomes: [MatchOutcome; NP], stage: Option<u32>) -> u32 {
        self.matches.insert(Match { id: 0, stage, done: false, competitors: [None; NP], outcomes })
    }

    //GETS POTENTIAL STARTING MATCHES, VALIDATES STRUCTURE
    pub fn get_inputs(&self) -> Result<Vec<(u32, u32)>, StructureError> {
        let mut v = Vec::new();
        let mut r = Vec::new();
        for m in &self.matches.data {
            for i in 0..NP {
                v.push((m.id.clone(), i as u32));
            }
            for o in &m.outcomes {
                match o {
                    MatchOutcome::Match(i, j) => { r.push((i.clone(), j.clone())); },
                    _ => {},
                }
            }
        }
        let v0 = v.iter().filter(|x| { !r.contains(x) }).map(|x| { *x }).collect::<Vec<_>>();
        match self.validate_structure() {
            Ok(()) => Ok(v0),
            Err(e) => Err(e),
        }
    }

    //STRUCTURE VALIDATION
    pub fn validate_structure(&self) -> Result<(), StructureError> {
        let mut match_check = vec![false; NP*self.matches.data.len()];
        let mut position_check = vec![false; NW];
        for m in &self.matches.data {
            let mut no = 0;
            for o in &m.outcomes {
                match o {
                    //VALIDATING MATCH OUTCOMES - MAKE SURE THEY LEAD TO VALID PLAYER ON VALID MATCH
                    MatchOutcome::Match(i, j) => {
                        if *j as usize >= NP { 
                            return Err(StructureError::MatchOutcomeInvalid(
                                [m.id.clone(), no, i.clone(), j.clone(), self.matches.data.len() as u32, NP as u32] 
                            )); 
                        }
                        match self.matches.index.get(i) {
                            None => {
                                return Err(StructureError::MatchOutcomeInvalid(
                                    [m.id.clone(), no, i.clone(), j.clone(), self.matches.data.len() as u32, NP as u32]
                                ))
                            }
                            Some(_) => {
                                match match_check[(*i as usize)*NP + (*j as usize)] {
                                    true => {
                                        return Err(StructureError::MatchOutcomeInvalid(
                                            [m.id.clone(), no, i.clone(), j.clone(), self.matches.data.len() as u32, NP as u32]
                                        ))
                                    },
                                    false => { match_check[(*i as usize)*NP + (*j as usize)] = true; },
                                }
                            }, 
                        }
                    },
                    //VALIDATING POSITION OUTCOMES - MAKE SURE THEY LEAD TO VALID FINAL POSITION
                    MatchOutcome::Position(i) => {
                        match position_check.get(*i as usize) {
                            None => {
                                return Err(StructureError::PositionOutcomeInvalid(
                                    [m.id.clone(), no, i.clone(), NW as u32]
                                ))
                            },
                            Some(check) => {
                                match check {
                                    true => {
                                        return Err(StructureError::PositionOutcomeInvalid(
                                            [m.id.clone(), no, i.clone(), NW as u32]
                                        ))
                                    }
                                    false => { position_check[*i as usize] = true; }
                                }
                            },
                        }
                    },
                    MatchOutcome::Discard => {},
                }
                no += 1;
            }
        }
        Ok(())
    }
}

//INSTANCE - ALLOWS TOURNEY TO BE USED AS TEMPLATE, HANDLES RUNNING THROUGH AND GETTING STATES
pub struct TourneyInstance<'a, const NP: usize, const NW: usize> {
    //TOURNEY CLONE
    pub tourney: Tourney<NP, NW>,
    //COMPETITOR REGISTER REF
    pub competitors: &'a mut Register<Competitor>,
    //CHECK INITIALISED, COMPETITORS LOADED
    pub initialised: bool,
    pub stages: HashMap<u32, bool>,
}

impl<'a, const NP: usize, const NW: usize> TourneyInstance<'a, NP, NW> {
    pub fn new(t: &Tourney<NP, NW>, c: &'a mut Register<Competitor>) -> Self {
        let tourney = t.clone();
        let competitors = c;
        Self { tourney, competitors, initialised: false, stages: HashMap::new() }
    }

    //LOAD COMPETITORS INTO TOURNEY, LOAD STAGES, CHECK NUMBER OF COMPETITORS == NUMBER OF STARTING SPOTS
    pub fn initialise(&mut self, ids: Vec<u32>) -> Result<(), StructureError> {
        for m in &self.tourney.matches.data {
            match m.stage {
                Some(s) => { self.stages.insert(s, false); },
                None => {},
            }
        }
        let competitors = ids.iter().filter(|x| { self.competitors.get(*x).is_some() }).collect::<Vec<_>>();
        match self.tourney.get_inputs() {
            Err(e) => Err(e),
            Ok(inp) => {
                if competitors.len() != inp.len() { return Err(StructureError::InputMatchingInvalid([competitors.len() as u32, inp.len() as u32])); }
                for i in 0..inp.len() {
                    let c = *competitors.get(i).unwrap();
                    let slot = *inp.get(i).unwrap();
                    let m = self.tourney.matches.get_mut(&slot.0).unwrap();
                    m.competitors[slot.1 as usize] = Some(c.clone());
                }
                Ok(())
            }
        }
    }

    //GET NUMBER OF AVAILABLE STARTING SPOTS
    pub fn num_spots(&self) -> usize { match self.tourney.get_inputs() { Ok(inputs) => inputs.len(), Err(e) => { eprintln!("{}", e); 0 } } }

    //GET ACTIVE MATCH REGISTER IDS - NOT USED INTERNALLY, FOR DISPLAY FUNCTIONS
    pub fn get_active(&self) -> Vec<u32> {
        let mut active = Vec::new();
        for m in &self.tourney.matches.data {
            if m.competitors.iter().filter(|x| { x.is_none() })
                                    .collect::<Vec<_>>().len() == 0 && !m.done {
                active.push(m.id.clone());
            }
        }
        active
    }

    //GET MATCH REGISTER IDS FOR STAGE - NOT USED INTERNALLY, FOR DISPLAY FUNCTIONS 
    pub fn get_stage(&self, s: u32) -> Vec<u32> {
        let mut stage = Vec::new();
        for m in &self.tourney.matches.data {
            match m.stage {
                Some(s0) => { if s0 == s { stage.push(m.id.clone()); } }
                None => {},
            }
        }
        stage
    }

    //IS STAGE COMPLETE?
    pub fn stage_complete(&self, s: u32) -> bool {
        let mut complete = true;
        for m in &self.tourney.matches.data {
            match m.stage {
                Some(s0) => { if s0 == s { if !m.done { complete = false; } } },
                None => {},
            }
        }
        complete
    }

    //STEP FORWARD
    pub fn step<T: TourneyRunner>(&mut self, runner: &mut T) -> bool {
        let mut assignments: Vec<(u32, u32, u32)> = Vec::new();
        let mut num_active = 0;
        //UPDATE STAGE COMPLETION
        let mut completed = Vec::new();
        for s in self.stages.keys() {
            if self.stage_complete(s.clone()) { completed.push(s.clone()); }
        }
        for s in completed { self.stages.insert(s, true); }
        for m in &mut self.tourney.matches.data {
            //CHECK STAGE, IF PRESENT, IS ACTIVE (STAGE BEFORE COMPLETE)
            let mut stage_active = true;
            match m.stage {
                Some(s) => { stage_active = match self.stages.get(&(s - 1)) {
                    Some(complete) => { if *complete { true } else { false }}
                    None => { true }
                } },
                None => {},
            }
            if m.competitors.iter().filter(|x| { x.is_none() })
                                    .collect::<Vec<_>>().len() == 0 && !m.done && stage_active {
                //LOOP OVER ACTIVE MATCHES
                num_active += 1;
                let result = runner.step::<NP>(
                    &m.competitors.iter().map(|x| { x.unwrap() }).collect::<Vec<_>>(),
                    &mut self.competitors,
                );
                for i in 0..NP {
                    let c = result[i];
                    let outcome = &m.outcomes[i as usize];
                    //CARRY OUT OUTCOMES
                    match outcome {
                        MatchOutcome::Match(m0, j) => {
                            assignments.push((c.clone() as u32, m0.clone(), j.clone()));
                        },
                        MatchOutcome::Position(p) => { 
                            self.tourney.positions[*p as usize] = Some(c as u32); 
                        },
                        MatchOutcome::Discard => {}, 
                    }
                }
                for i in 0..NP { m.competitors[i] = Some(result[i]); }
                m.done = true;
                //LOOPED OVER ACTIVE MATCHES
            }
        }
        //ASSIGN COMPETITORS TO NEW MATCHES 
        for a in assignments {
            self.tourney.matches.get_mut(&a.1).unwrap().competitors[a.2 as usize] = Some(a.0);
        }
        //RETURN TRUE IF MATCHES WERE COMPLETED THIS ROUND
        if num_active > 0 { return true; } else { return false; }
    }
}

//TRAIT FOR RUNNERS - JUST HAS TO BE ABLE TO IMPLEMENT STEP, SHOULD ALSO UPDATE STATS OF COMPETITORS
pub trait TourneyRunner {
    fn step<const NP: usize>(&mut self, indices: &[u32], competitors: &mut Register<Competitor>) -> [u32; NP];
}