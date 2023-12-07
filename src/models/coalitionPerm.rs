use std::collections::HashMap;
use rand::rngs::StdRng;
use rand::{random, Rng,SeedableRng};
use rand::distributions::{Uniform, Distribution, Normal};
use std::time::Instant;

const n_A : i16 = 100; //number of agents
const VALUE_FUNCTION : &str = "uniform"; //uniform, normal, agent_based, NDCS, couple_based

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Move{
    pub agent : u32,
    pub coa2 : usize
}

#[derive(Clone)]
pub struct State{
    pub agents : Vec<i16>,
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

        let mut coa : Vec<Vec<u32>> = Vec::new();
        let mut ag : Vec<i16> = Vec::new();
        for i in 0..n_A {
            coa.push(vec![i as u32]);
            ag.push(i)
        }

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
        Self{ agents : ag, coalitions : coa, seq : Vec::new(), seed : sd, inst : Instant::now(), stocked : stocked, power : pow, power2 : pow2 }
    }

    pub fn new_with_seed(mut sd: u16, stocked : *mut HashMap<Vec<u32>, f64>) -> Self
    {
        if sd == 0 {
            let mut rng = rand::thread_rng();
            sd = rng.gen::<u16>();
        }

        let mut rng = StdRng::seed_from_u64(sd as u64);
        let mut coa : Vec<Vec<u32>> = Vec::new();
        let mut ag : Vec<i16> = Vec::new();
        for i in 0..n_A {
            coa.push(vec![i as u32]);
            ag.push(i)
        }
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

        Self{ agents : ag, coalitions : coa, seq : Vec::new(), seed : sd, inst : Instant::now(), stocked : stocked, power : pow, power2 : pow2 }
    }

    pub fn randomize(& mut self){

        let mut rng = rand::thread_rng();

        let coa_num = n_A/5 ;
        self.coalitions = Vec::new();

        for i in 0..coa_num {
            self.coalitions.push(Vec::new());
        }

        for a in 0..n_A {
            let coa = rng.gen_range(0, coa_num as usize);
            self.coalitions[coa].push(a as u32);
            self.agents[a as usize] = coa as i16;
        }
    }

    pub fn play(&mut self, m : Move){
        /*
        println!(" agag");
        println!("{:?}", m.agent);
        println!("{:?}", m.coa2);
        println!("{:?}", self.coalitions);
        println!("{:?}", self.agents);
    */

        let coa1 = self.agents[m.agent as usize] as usize;
        if m.coa2 >= self.coalitions.len() {
            self.coalitions.push(Vec::new());
        }
        self.coalitions[m.coa2].push(m.agent);
        let index = self.coalitions[coa1].iter().position(|x| *x == m.agent ).unwrap();
        self.coalitions[coa1].swap_remove(index);
        self.agents[m.agent as usize] = m.coa2 as i16;

        if(self.coalitions[coa1].len() == 0){
            self.coalitions.remove(coa1);
            for a  in 0..n_A as usize {
                if self.agents[a] >= coa1 as i16 {
                    self.agents[a]-=1;
                }
            }
        }



        //println!("score 1 : {}", self.score()); //vérifier le déterminisme
        //println!("score 2 : {}", self.score());

        self.seq.push(m);
    }

    pub fn legal_moves(& self) ->Vec<Move>{
        let mut vec :Vec<Move> = Vec::new();
        for agent in 0..n_A {
            for coa1 in 0..self.coalitions.len() {
                for coa2 in 0..(self.coalitions.len()+1) {
                    if coa1 != coa2 {
                        let m1 = Move{agent : agent as u32, coa2 : coa2 };
                        vec.push(m1);
                    }
                }
            }
        }

        return vec;
    }

    pub fn best_greedy_moves(& mut self, applyMove : bool) ->Vec<Move>{

        let mut vec :Vec<Move> = vec![Move{agent : (n_A+1) as u32, coa2 : 10000 }; n_A as usize];


        for ag in 0..n_A  as usize {
            let mut uti_coa = Vec::new();
            for coa in 0..self.coalitions.len() {
                let temp = &self.coalitions[coa].clone();
                uti_coa.push(self.get_coa_utility(temp));
            }

            let coa1  = self.agents[ag] as usize;
            let mut best_coa2 = coa1;
            let mut best_delta = 0.0;
            for coa2 in 0..(self.coalitions.len()+1) {
                if coa1 != coa2 {

                    if coa2 < self.coalitions.len(){
                        let mut new_coa1: Vec<u32> = self.coalitions[coa1].clone();
                        let mut new_coa2: Vec<u32> = self.coalitions[coa2].clone();
                        let index = new_coa1.iter().position(|x| *x == ag as u32).unwrap();
                        new_coa1.swap_remove(index);
                        new_coa2.push(ag as u32);
                        let delta = self.get_coa_utility(&new_coa1) + self.get_coa_utility(&new_coa2) - uti_coa[coa1] - uti_coa[coa2];
                        if delta > best_delta {
                            best_coa2 = coa2;
                            best_delta = delta
                        }
                    }else{
                        let mut new_coa1: Vec<u32> = self.coalitions[coa1].clone();
                        let mut new_coa2: Vec<u32> = Vec::new();
                        let index = new_coa1.iter().position(|x| *x == ag as u32).unwrap();
                        new_coa1.swap_remove(index);
                        new_coa2.push(ag as u32);
                        let delta = self.get_coa_utility(&new_coa1) + self.get_coa_utility(&new_coa2) - uti_coa[coa1];
                        if delta > best_delta {
                            best_coa2 = coa2;
                            best_delta = delta
                        }
                    }
                }
            }

            if coa1 != best_coa2 {
                let m1 = Move{agent : ag as u32, coa2 : best_coa2 };
                vec[ag] = m1;
                if applyMove {
                    self.play(m1);
                }
            }else{
                //println!("no move for agent {}", ag);
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

                if coaID == 0{return 0.0;}

                //uniform
                if VALUE_FUNCTION == "uniform"{
                    let mut rng = StdRng::seed_from_u64(coaID + self.seed as u64);
                    let distrib = Uniform::new(0.0, 1.0 * coa.len() as f64); //uniform
                    //let distrib = Uniform::new(0.0, 1_f64); //uniform 0 1
                    ret = distrib.sample(&mut rng) as f64;
                }


                //normal
                if VALUE_FUNCTION == "normal"{
                    let mut rng = StdRng::seed_from_u64(coaID + self.seed as u64);
                    let distrib = Normal::new(10.0 * coa.len() as f64, 0.1);
                    ret = distrib.sample(&mut rng) as f64;
                }

                //agent based
                if VALUE_FUNCTION == "agent_based"{
                    let mut rng = StdRng::seed_from_u64(coaID + self.seed as u64);

                    let mut sum = 0.0;
                    for &a in coa {
                        sum+=Uniform::new(0.0,  self.power[a as usize] as f64).sample(&mut rng);
                    }
                    ret = sum;
                }


                //NDCS
                if VALUE_FUNCTION == "NDCS"{
                    let mut rng = StdRng::seed_from_u64(coaID + self.seed as u64);
                    let distrib = Normal::new(coa.len() as f64, (coa.len() as f64).sqrt());
                    ret = distrib.sample(&mut rng) as f64;
                }



                //couple based
                if VALUE_FUNCTION == "couple_based"{
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

        //just testing
        //println!("{}", self.get_coa_utility(&vec![1_u32, 2_u32, 3_u32, 4_u32, 5_u32]));
        //println!("{}", self.get_coa_utility(&vec![11_u32, 12_u32, 13_u32, 14_u32, 15_u32]));


        for e in 0..self.coalitions.len() {
            let temp = &self.coalitions[e].clone();
            total += self.get_coa_utility(temp);
        }
        return total;
    }

    pub fn smoothedScore(&mut self) ->f64{return self.score();}

    pub fn heuristic(&self, m : Move) -> f64{ return 0.0; }

    pub fn terminal(& self) -> bool{
        return self.coalitions.len() == 1;
    }
}
