use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::fs;
use std::process::exit;

fn main() {
    let fastin = fs::read_to_string("./fastin.txt").unwrap();
    let lines = fastin.lines().collect::<Vec<&str>>();

    println!("reding size");
    let size = match lines[0].parse::<i128>() {
        Ok(res) => res,
        Err(err) => {
            println!("err:1 {}", err);
            exit(0);
        }
    };

    println!("reading U");
    let bu = match lines[1].parse::<i128>() {
        Ok(res) => res,
        Err(err) => {
            println!("err:2 {}", err);
            exit(0);
        }
    };

    if size % 8 != 0 {
        println!("size shall be divisable by  8");
        exit(0);
    }

    let mut sum = 0;
    let mut public = vec![];
    for i in 2..size + 2 {
        let buf = lines[i as usize];
        let val = match buf.parse::<i128>() {
            Ok(res) => {
                sum += res;
                res
            }
            Err(err) => {
                println!("err3: {}", err);
                exit(0);
            }
        };
        public.append(&mut vec![val]);
    }

    let max = match public.iter().max() {
        Some(max) => *max,
        None => exit(0),
    };
    let min = match public.iter().min() {
        Some(min) => *min,
        None => exit(0),
    };
    println!(
        "size:{}, U:{}, sum:{}, avg:{}, max:{}, min:{},\n{:?}\n\n",
        size,
        bu,
        sum,
        sum/size,
        max,
        min,
        public.clone()
    );

    println!("solivng...\n\n");

    let pairs = (1..bu + 1)
        .map(|i| {
            let mut vec = vec![];
            for j in i..bu + 1 {
                if euc_lib::I128::euc(i, j) == 1 {
                    vec.append(&mut vec![(i, j)]);
                }
            }
            return vec;
        })
        .collect::<Vec<Vec<(i128, i128)>>>();
    let pairs = pairs
        .into_iter()
        .flatten()
        .unique()
        .collect::<Vec<(i128, i128)>>();
    println!("workable pairs created: {}\n", pairs.len());

    let options = pairs
        .par_iter()
        .map(|t| {
            let (i, j) = t;
            let mut div = 1;
            let mut n =  i128::abs((j*public[0]) - (i*public[1]));
            while n > max {
                n = n/div;
                let mut inv = euc_lib::I128::euc_ext(n, *i).t;
                if inv < 0 {
                    inv += n;
                }

                let q = inv * public[0] % n;
                if public[0] == (i * q) % n && public[1] == (j * q) % n {
                    return (*i, *j, q, n);
                }
                div += 1;
            }
            return (0, 0, 0, 0);
        })
        .collect::<Vec<(i128, i128, i128, i128)>>();

    let options = options
        .iter()
        .unique()
        .collect::<Vec<&(i128, i128, i128, i128)>>();
    println!("working options created, {}, \n", options.len());

    let estim = options
        .par_iter()
        .map(|t| {
            let (u0, u1, q, n) = t;
            let mut count = 0;
            let mut prev = u0+u1;
            let mut vec = vec![];
            vec.append(&mut vec![*u0]);
            vec.append(&mut vec![*u1]);
            for i in 2..public.len() {
                for j in prev..(*n) {
                    if public[i] == (j * q) % n {
                        count += 1;
                        prev += j;
                        vec.append(&mut vec![j]);
                        break;
                    }
                }
            }
            if count == size {
                return (*q, *n, vec);
            }
            return (0, 0, vec![]);
        })
        .collect::<Vec<(i128, i128, Vec<i128>)>>();

    let mut estim = estim
        .iter()
        .unique()
        .sorted()
        .collect::<Vec<&(i128, i128, Vec<i128>)>>();
    estim.sort();
    _ = estim.remove(0);
    println!("estim: {}, {:?}", estim.len(), estim[0])
}
