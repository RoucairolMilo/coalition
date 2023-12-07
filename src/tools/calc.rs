//use std::intrinsics::powf64;
use nalgebra::{DMatrix, Dynamic};

//ici mettre distance manhattan, matrice de distance, softmax
pub fn softmaxChoice(l : Vec<f64>) -> usize{
    let r = rand::random::<f64>();
    let mut sum = 0.0;
    for i in 0..l.len() {
        sum += l[i].exp();
    }
    let mut sum2 = 0.0;
    for i in 0..l.len() {
        sum2+= l[i].exp()/sum;
        if sum2 >= r{
            return i;
        }
    }
    println!("whaaat ????");
    println!("{} --- {} --- {}", r, sum, sum2);
    return l.len();
}

pub fn multiChoice(l : Vec<f64>, n : usize) -> Vec<usize>{
    let mut v = Vec::new();
    let mut L : Vec<f64> = l.clone();
    for i in 0..n{
        v.push(softmaxChoice(L.clone()));
        let coeff = v[v.len()-1] as f64;
        L.remove(v[v.len()-1]);
        for a in 0..L.len(){
            L[a] *= 1.0/(1.0-coeff);
        }
    }
    return v;
}

pub fn mean(l : &Vec<f64>) -> f64
{
    let mut sum: f64 = 0.0;
    for x in l {
        sum = sum + x;
    }

    return sum / l.len() as f64;
}

pub fn std(l : &Vec<f64>) -> f64
{
    let mean = mean(l);

    let mut sum: f64 = 0.0;
    for x in l {
        sum =  sum + (x-mean).powi(2);
    }

    sum = sum /l.len() as f64;

    return sum.powf (0.5);
}