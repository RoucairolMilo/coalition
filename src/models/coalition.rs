use std::collections::HashMap;
use rand::rngs::StdRng;
use rand::{random, Rng,SeedableRng};
use rand::distributions::{Uniform, Distribution, Normal};
use std::time::Instant;
use std::fmt;

const n_A : i16 = 100; //number of agents
const VALUE_FUNCTION : &str = "uniform"; //uniform, normal, agent_based, NDCS, couple_based

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Move{

    pub coa1 : usize,
    pub coa2 : usize
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "coa1: {}, coa2: {}", self.coa1, self.coa2)
    }
}

#[derive(Clone)]
pub struct State{
    pub coalitions : Vec<Vec<u32>>,
    pub seq : Vec<Move>,
    pub seed : u16,
    pub inst : Instant,
    pub  stocked : *mut HashMap<Vec<u32>, f64>,
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

        for i in 0..n_A {
            coa.push(vec![i as u32]);
            pow.push(Uniform::new(0.0, 1.0).sample(&mut rng));
            pow2.push(Vec::new());
            for _ in 0..n_A {
                pow2[i as usize].push(Uniform::new(0.0, 1.0).sample(&mut rng));
            }
        }

        Self{ coalitions : coa, seq : Vec::new(), seed : sd, inst : Instant::now(), stocked : stocked, power : pow, power2 : pow2 }
    }

    pub fn play(&mut self, m : Move){
        let mut cl = self.coalitions[m.coa2].clone();
        self.coalitions[m.coa1].append(&mut cl);
        self.coalitions.remove(m.coa2);


        self.seq.push(m);
    }

    pub fn legal_moves(& self) ->Vec<Move>{
        let mut vec :Vec<Move> = Vec::new();
        for i in 0..self.coalitions.len() {
            for j in 0..self.coalitions.len() {

                if i < j {
                    let m1 = Move{coa1 : i, coa2 : j };
                    vec.push(m1);
                }

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
                if VALUE_FUNCTION == "normal" {
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


                if VALUE_FUNCTION == "NDCS" {

                }
                let mut rng = StdRng::seed_from_u64(coaID + self.seed as u64);
                let distrib = Normal::new(coa.len() as f64, (coa.len() as f64).sqrt());
                ret = distrib.sample(&mut rng) as f64;

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

    pub fn score(&mut self) -> f64{
        let mut total : f64 = 0.0;

        for e in &self.coalitions.clone() {
            total += self.get_coa_utility(e);
        }
        return total;
    }

    pub fn smoothedScore(&mut self) ->f64{return self.score();}

    pub fn heuristic(&mut self, m : Move) -> f64{
        let utiBefore = self.get_coa_utility(&self.coalitions[m.coa1].clone()) + self.get_coa_utility(&self.coalitions[m.coa2].clone());

        let mut coa1 = self.coalitions[m.coa1].clone();
        let mut coa2 = self.coalitions[m.coa2].clone();
        coa2.append(&mut coa1);

        return self.get_coa_utility(&coa2) - utiBefore;



        let beforeMscore = self.score();
        let mut afterMove = self.clone();
        afterMove.play(m);
        let afterMscore = afterMove.score();
        return afterMscore - beforeMscore;
    }

    pub fn terminal(& self) -> bool{
        return self.coalitions.len() == 1;
    }
}
