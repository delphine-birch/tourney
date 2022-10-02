# Tourney

## Overview
Tourney is a small rust package designed to help deal with a specific, niche issue that I've come up against a few times - generically programming and formatting tournaments.

Tourney can't quite create any format of tournament - the limitations currently are that each match within the tournament must have the same number of contestants, and that each match must provide a specific ordering of each of its contestants once done (though this doesn't need to be an ordering from winner to loser).

The package is currently very barebones, but in future it will hopefully contain some functions both for visualising tournament structures in the command line and for creating default versions of some common tournament types (single/double elimination, round robin, etc.)

## Basic Example
To get started with Tourney, you're first going to want to design your tournament structure - here we'll design a simple 8 person single elimination tournament with only 1 recognised winner (no 2nd, 3rd places).

```rust
//Tournaments need two generic usize parameters - first, the number of contestants
    //in each match (Two, in our case), and the number of final recognised placement
    //positions.
    let mut tournament_template = Tourney::<2, 1>::new();

    //Now we need to set up our matches - this is done by supplying a list of
    //MatchOutcome enums - these can be one of three options:
    
    //MatchOutcome::Match(m, c): Competitor proceeds on to match with ID m in
    //                           position c.
    //MatchOutcome::Position(p): Competitor places in final position p.
    //MatchOutcome::Discard:     Competitor does not continue onwards.
    
    //Let's start with the final match - we'll want to work backwards, as earlier
    //matches will need access to the IDs of the matches they lead to.

    let final_match_id = tournament_template.add_match([
        MatchOutcome::Position(0), //The competitor sorted into the first outcome by the
                                   //match algorithm will place in final position 0.
        MatchOutcome::Discard      //The competitor sorted into the second outcome will not
                                   //continue on, as we aren't recording second place here.
    ], None); //None here is an option for staging, which we'll discuss later.

    //With this ID we can set up the two semi-final matches.

    let semifinal_match_id_0 = tournament_template.add_match([
        MatchOutcome::Match(final_match_id, 0), //The competitor sorted into the first
                                               //outcome by the match algorithm will
                                               //go on to be competitor 0 in the final match.
        MatchOutcome::Discard
    ], None);
    
    let semifinal_match_id_1 = tournament_template.add_match(
        [MatchOutcome::Match(final_match_id, 1), MatchOutcome::Discard], None
    );

    //The first four matches can now be set up the same way.
    let match_id_0 = tournament_template.add_match(
        [MatchOutcome::Match(semifinal_match_id_0, 0), MatchOutcome::Discard], None
    );
    let match_id_1 = tournament_template.add_match(
        [MatchOutcome::Match(semifinal_match_id_0, 1), MatchOutcome::Discard], None
    );
    let match_id_2 = tournament_template.add_match(
        [MatchOutcome::Match(semifinal_match_id_1, 0), MatchOutcome::Discard], None
    );
    let match_id_3 = tournament_template.add_match(
        [MatchOutcome::Match(semifinal_match_id_1, 1), MatchOutcome::Discard], None
    );

    //And we're done!

    //Now, we need an instance to run our tournament, and a register of competitors for it
    //to draw on.

    let mut competitors: Register<Competitor> = Register::new();
    let mut instance = TourneyInstance::new(
        &tournament_template, 
        &mut competitors
    );

    //Now we add 8 competitors to our register and store their Ids.
    let mut competitor_ids = Vec::new();
    for _ in 0..8 { 
        competitor_ids.push(instance.competitors.insert(Competitor::new(vec![0.0]))); 
    }
    //Note - the vec parameter passed to a competitor should contain any stats (as f32) we 
    //want to track. In this case, we're just going to track one statistic - win count.

    //And now we initialise the instance - this checks there are an equal number of open
    //tournament spots as there are competitor IDs supplied to the instance, and that the
    //tournament template was formatted correctly - note that you can validate this 
    //seperately on the template using tournament_template.validate_structure()

    match instance.initialise(competitor_ids) {
        Err(e) => { eprintln!("{}", e); }
        Ok(()) => {},
    }

    //With all this done, our tournament is ready to run, but we haven't yet programmed
    //any way to decide who wins a match. We'll do this by implementing the TournamentRunner
    //trait onto a custom struct.

    struct CoinFlipper;
    impl CoinFlipper { pub fn new() -> Self { Self } }
    impl TourneyRunner for CoinFlipper {
        //We just need to implement one function - this step function.

        //All this function does, essentially, is re-order a list of competitor IDs - 
        //So once it's done, the competitor at index 0 of the returned array will
        //be matched to the outcome at index 0 of the match they're in, and so on.

        //Here we'll just be dealing with two competitors per match, and we want a
        //coin flip on who wins - so we'll just swap the order of the competitors 
        //if a random float is higher than 0.5.
        fn step<const NP: usize>(
            &mut self, 
            competitor_ids: &[u32], 
            competitors: &mut Register<Competitor>
        ) -> [u32; NP] {
            //Note here we have to allow for the potential of more players (NP > 2),
            //as we can't specify the generic const for this specific implementation -
            //if you do know how to do this, please feel free to let me know
            let mut results = [0; NP];
            if rand::random::<f32>() > 0.5 {
                results[0] = competitor_ids[1];
                results[1] = competitor_ids[0];
            }
            else {
                results[0] = competitor_ids[0];
                results[1] = competitor_ids[1];
            }
            //we also need to register the 'winner' for the win count stat,
            //in this case defined as the competitor at index 0.
            competitors.get_mut(&results[0]).unwrap()._stats[0] += 1.0;

            results
        }
    }

    let mut runner = CoinFlipper::new();

    //With all this done, we're now ready to run our tournament! The structure we've
    //outlined should have 3 rounds, so let's step it forward 3 times, and then see
    //who won.

    for _ in 0..3 { instance.step(&mut runner); }
    eprintln!("Winner was: Competitor Number {}", instance.tourney.positions[0].unwrap());

    //This should come out different every time! And with that, you should have a fully
    //functioning tournament - you can create new instances from the same template and
    //register of competitors, and the competitor stats should be persistent, recording
    //total wins over every tournament you run.
```
