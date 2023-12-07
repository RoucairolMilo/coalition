#![allow(non_snake_case)]

use std::collections::HashMap;
use std::io;
use std::io::Write;
use crate::models::coalition::{State}; //change here to coalitionsB
use crate::models::coalitionPerm::{State as StatePerm};
use std::time::Instant;
use calc::mean;
use calc::std;
use crate::tools::calc;

mod tests;
mod tools;
mod methods;
mod models;

fn main() {


    // COALITIONS


    let time_budget :f64 = 100.0; //100s c'est bien
    let mut stocked_coa_values : HashMap<Vec<u32>, f64> = HashMap::new();
    let mut init_st = State::new_with_seed(0, &mut stocked_coa_values); //état pour coalition merge
    let mut init_stp = StatePerm::new_with_seed(0, &mut stocked_coa_values); //état pour coalition permutation





    if true {
        let nb_simulation = 100;
        //TESTS préliminaires
        let mut stocked_coa_values : HashMap<Vec<u32>, f64> = HashMap::new();

        for repeat in 0..nb_simulation  {
            println!("expe {}", repeat);
            let mut init_st = State::new_with_seed(repeat, &mut stocked_coa_values);
            let mut init_stp = StatePerm::new_with_seed(repeat, &mut stocked_coa_values);



            //RHC
            stocked_coa_values = HashMap::new();
            methods::PermHillClimb::HillClimb(init_stp.clone(), time_budget, String::from(format!("RHC{}", repeat)));



            // model A
            /*
            stocked_coa_values = HashMap::new();
            let st = methods::NMCS::launch_nmcs(init_st.clone(),3, 0.0, true,time_budget, String::from(
                format!("NMCS{}", repeat)
            ));

            //let st = methods::lazyNMCSv2::launch_lazy_nmcs_v2(init_st.clone(),3, 10, 1.0, 0.0, time_budget as f64, 0, true, String::from(format!("prelimLNMCS{}", repeat)));

            stocked_coa_values = HashMap::new();
            let st = methods::lazyNMCSv3::launch_lazy_nmcs_v3(init_st.clone(),5, 0.0, 2, 10, 0.0, time_budget as f64, 0, true, String::from(
                format!("LNMCS{}", repeat)
            ));



            stocked_coa_values = HashMap::new();
            let st = methods::UCT::launch_UCT(init_st.clone(),1.0, 1000000, 0.0, time_budget, String::from(
                format!("UCT{}", repeat)
            ));


            stocked_coa_values = HashMap::new();
            let st = methods::CSGUCT::launch_CSG_UCT(init_st.clone(),1.0, 1000000, 0.0, time_budget, String::from(
                format!("CSG-UCT{}", repeat)
            ));
            */






            // model B

            /*
            stocked_coa_values = HashMap::new();
            let st = methods::lazyNMCSv3::launch_lazy_nmcs_v3(init_st.clone(),5, 0.9, 2, 0, 0.0, time_budget as f64, 0, true, String::from(
                format!("LNMCSRAW{}", repeat)
            ));


            stocked_coa_values = HashMap::new();
            let st = methods::NMCS::launch_nmcs(init_st.clone(),3, 0.0, true,time_budget, String::from(
                format!("NMCS{}", repeat)
            ));



            stocked_coa_values = HashMap::new();
            let st = methods::lazyNMCSv3::launch_lazy_nmcs_v3(init_st.clone(),5, 0.0, 2, 10, 0.0, time_budget as f64, 0, true, String::from(
                format!("LNMCS{}", repeat)
            ));

            stocked_coa_values = HashMap::new();
            let st = methods::UCT::launch_UCT(init_st.clone(),1.0, 1000000, 0.0, time_budget, String::from(
                format!("UCT{}", repeat)
            ));

             */



            //model A greedy
            /*
            stocked_coa_values = HashMap::new();
            let st = methods::NMCS::launch_nmcs(init_st.clone(),3, 0.0, true,time_budget, String::from(
                format!("NMCS{}", repeat)
            ));



            stocked_coa_values = HashMap::new();
            let st = methods::lazyNMCSv3::launch_lazy_nmcs_v3(init_st.clone(),5, 0.0, 1, 10, 0.0, time_budget as f64, 0, true, String::from(
                format!("LNMCS{}", repeat)
            ));

             */

        }
    }
}