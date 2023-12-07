use std::collections::HashMap;
use rand::rngs::StdRng;
use rand::{random, Rng,SeedableRng};
use rand::distributions::{Uniform, Distribution, Normal};
use std::time::Instant;

const n_A : i16 = 100; //number of agents
const VALUE_FUNCTION : &str = "uniform"; //uniform, normal, agent_based, NDCS, couple_based

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Move{

    pub coa : usize,
}

#[derive(Clone)]
pub struct State{
    pub coalitions : Vec<Vec<u32>>,
    pub locked_coa : usize,
    pub locked_moves : usize,
    pub seq : Vec<Move>,
    pub seed : u16,
    pub inst : Instant,
    pub stocked : *mut HashMap<Vec<u32>, f64>,
    pub power : Vec<f64>,
    pub power2 : Vec<Vec<f64>>
}

impl State{
    pub const CONSIDER_NON_TERM: bool = true;
    pub fn new(stocked : *mut HashMap<Vec<u32>, f64>) -> Self {

        let mut rng = rand::thread_rng();
        let sd = rng.gen::<u16>();

        State::new_with_seed(sd, stocked)
    }

    pub fn new_with_seed(mut sd: u16, stocked : *mut HashMap<Vec<u32>, f64>) -> Self
    {
        let mut rng = StdRng::seed_from_u64(sd as u64);
        let mut coa : Vec<Vec<u32>> = Vec::new();
        let mut pow = Vec::new();
        let mut pow2= Vec::new();

        //let mesure = Instant::now();
        for i in 0..n_A {
            coa.push(vec![i as u32]);
            pow.push(Uniform::new(0.0, 1.0).sample(&mut rng));
            pow2.push(Vec::new());
            for _ in 0..n_A {
                pow2[i as usize].push(Uniform::new(0.0, 1.0).sample(&mut rng));
            }
        }
        Self{ coalitions : coa, locked_coa : 0, locked_moves : 0, seq : Vec::new(), seed : sd, inst : Instant::now(), stocked : stocked, power : pow, power2 : pow2}
    }
    pub fn play(&mut self, m : Move){

        if m.coa == self.locked_coa {
            self.locked_coa+=1;
            self.locked_moves = self.locked_coa;
        }
        else{
            let mut cl = self.coalitions[m.coa].clone();
            self.coalitions[self.locked_coa].append(&mut cl);
            self.coalitions.remove(m.coa);
            self.locked_moves = m.coa;
        }


        //println!("score 1 : {}", self.score()); //vérifier le déterminisme
        //println!("score 2 : {}", self.score());

        self.seq.push(m);
    }

    pub fn legal_moves(& self) ->Vec<Move>{
        let mut vec :Vec<Move> = Vec::new();


        if true {
            //all moves mode
            for i in self.locked_coa..self.coalitions.len() {
                let m1 = Move{coa : i};
                vec.push(m1);
            }
        }else{
            //unique moves mode (unbalanced search space tree)
            for i in self.locked_moves..self.coalitions.len() {
                let m1 = Move{coa : i};
                vec.push(m1);
            }

            if self.locked_moves != self.locked_coa {
                vec.push(Move{coa : self.locked_coa});
            }
        }


        return vec;
    }

    pub fn get_coa_utility(&mut self, coa : &Vec<u32>) -> f64{
        let mut ret = 0.0;
        unsafe{
            if (*self.stocked).contains_key(&coa.clone()) {
                ret = *(*self.stocked).get(coa).unwrap();
            }else{

                let mut coaID : u64 = 0;
                for e  in 0..coa.len() {
                    coaID += 2_u64.pow(coa[e]%63);
                }


                //uniform
                if VALUE_FUNCTION == "uniform" {
                    let mut rng = StdRng::seed_from_u64(coaID + self.seed as u64);
                    let distrib = Uniform::new(0.0, 1.0 * coa.len() as f64); //uniform
                    //let distrib = Uniform::new(0.0, 1_f64); //uniform 0 1
                    ret = distrib.sample(&mut rng) as f64;
                }




                //normal
                if VALUE_FUNCTION == "uniform" {
                    let mut rng = StdRng::seed_from_u64(coaID + self.seed as u64);
                    let distrib = Normal::new(10.0 * coa.len() as f64, 0.1);
                    ret = distrib.sample(&mut rng) as f64;
                }





                //agent based
                if VALUE_FUNCTION == "agent_based" {
                    let mut rng = StdRng::seed_from_u64(coaID + self.seed as u64);

                    let mut sum = 0.0;
                    for &a in coa {
                        sum+=Uniform::new(0.0,  self.power[a as usize] as f64).sample(&mut rng);
                    }
                    ret = sum;
                }


                //NDCS
                if VALUE_FUNCTION == "NDCS" {
                    let mut rng = StdRng::seed_from_u64(coaID + self.seed as u64);
                    let distrib = Normal::new(coa.len() as f64, (coa.len() as f64).sqrt());
                    ret = distrib.sample(&mut rng) as f64;
                }



                //couple based
                if VALUE_FUNCTION == "couple_based" {
                    let mut sum = 0.0;
                    let mut n = 0.0;

                    if coa.len() > 1 {
                        for a in 0..coa.len()-1 {
                            for b in a+1..coa.len(){
                                if a < b {
                                    sum+= self.power2[coa[a] as usize][coa[b] as usize];
                                }else{
                                    sum+= self.power2[coa[b] as usize][coa[a] as usize];
                                }
                                n+=1.0;
                            }
                        }
                        ret = sum/n * coa.len() as f64;
                    }else{
                        ret = self.power2[coa[0] as usize][coa[0] as usize]
                    }

                }







                (*self.stocked).insert(coa.to_vec(), ret);

                //this is for the modified distributions, unused
                /*
                //let distrib = Normal::new(10.0 * coa.len() as f64, 0.01); //normal
                //let distrib = Normal::new(coa.len() as f64, self.coalitions[i].len() as f64); //NDCS

                let mut ret = distrib.sample(&mut rng); //not modified

                if false { //modified
                    //rng = StdRng::seed_from_u64(self.inst.elapsed().subsec_nanos() as u64);
                    if rng.gen_bool(0.2) {
                        ret += Uniform::new(0.0, 50.0).sample(&mut rng);
                    }
                }*/
            }
        }
        return ret ;
    }

    pub fn score(& mut self) -> f64{
        let mut total : f64 = 0.0;


        for e in &self.coalitions.clone() {
            total += self.get_coa_utility(e);
        }
        return total;
    }

    pub fn smoothedScore(&mut self) ->f64{return self.score();}

    pub fn heuristic(&self, m : Move) -> f64{return 0.0;
    }

    pub fn terminal(& self) -> bool{
        return self.coalitions.len() == self.locked_coa;
    }
}
