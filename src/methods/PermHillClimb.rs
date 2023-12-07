use std::time::Instant;
//n'utiliser qu'avec coalitionPerm
use crate::models::coalitionPerm::{State, Move};
use crate::tools::resultSaver::writeLine;

pub fn HillClimb(inist : State, timeout : f64, registerName : String){

    let mut start_time = Instant::now();
    let mut success = true;
    let mut best_score = 0.0;
    while start_time.elapsed().as_secs_f64() < timeout {

        let mut rand_st = inist.clone();
        rand_st.randomize();
        //println!("hill climb started");
        success = true;
        while success && start_time.elapsed().as_secs_f64() < timeout {
            success = false;

            let moves = rand_st.best_greedy_moves(true);
            let sc = rand_st.score();
            if( sc > best_score){
                success = true;
                best_score = sc;
                writeLine(start_time.elapsed().as_secs_f64().to_string() + " " + &*best_score.to_string()+ "\n", registerName.clone());
                println!("Hill Climb Perm record battu ! {}", best_score);
            }
        }
    }


}