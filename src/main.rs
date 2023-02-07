use itertools::Itertools;
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
        .iter()
        .map(|t| {
            let (i, j) = t;
            let mut div = 1;
            let mut n =  i128::abs((i*public[1])- (j*public[0]));
            
            let mut n_c = n.clone();
            while n_c > max {
                div += 1;
                n_c = n/div;
            }
            if div == 1 {
                return (0, 0, 0, 0);    
            }

            n = n/(div -1);
            let mut inv = euc_lib::I128::euc_ext(n, *i).t;
            if inv < 0 {
                inv += n;
            }
            let q = inv * public[0] % n;
            if public[0] == (i * q) % n && public[1] == (j * q) % n {
                return (*i, *j, q, n);
            }
            
            return (0, 0, 0, 0);
        })
        .collect::<Vec<(i128, i128, i128, i128)>>();

    let options = options
        .iter()
        .unique()
        .collect::<Vec<&(i128, i128, i128, i128)>>();
    println!("working options created, {}, \n", options.len());

    let first_estim = options
        .iter()
        .map(|t| {
            let (u0, u1, q, n) = t;
            let mut vec = vec![];
            vec.append(&mut vec![*u0]);
            vec.append(&mut vec![*u1]);
            if *n != 0 {
                let mut q_inv = euc_lib::I128::euc_ext(*n, *q).t;
                if q_inv < 0 {
                    q_inv += n;
                }
                for i in 2..public.len() {
                    let mut ui = (public[i] * q_inv) % n;
                    if ui < 0 {
                        ui += n;
                    }
                    sum += ui;
                    vec.append(&mut vec![ui]);
                }
                return (*q,*n,vec);
            }
            return (0, 0, vec![]);
        })
        .collect::<Vec<(i128, i128, Vec<i128>)>>();
    let mut first_estim = first_estim
        .iter()
        .unique()
        .sorted()
        .collect::<Vec<&(i128, i128, Vec<i128>)>>();
    first_estim.sort();
    _ = first_estim.remove(0);
    println!("first estim: {} \n", first_estim.len());

    for res in first_estim.clone() {
        let mut v = res.2.clone();
        v.sort();
        println!("one of solutions Q:{} N:{} key:{:?} \n", res.0, res.1, v);
    }

    let sec_estim = first_estim.iter().map(|est|{
        let v_o = est.2.clone();
        let mut v_s = est.2.clone();
        v_s.sort();
        sum = 0;
        for i in v_o.clone() {
            sum += i;
        }
        if v_o == v_s && sum < est.1 {
            return (est.0, est.1, est.2.clone());
        } else {
            return (0, 0, vec![]);
        }
    })
    .collect::<Vec<(i128, i128, Vec<i128>)>>();
    
    let mut sec_estim = sec_estim
        .iter()
        .unique()
        .sorted()
        .collect::<Vec<&(i128, i128, Vec<i128>)>>();
    sec_estim.sort();
    sec_estim.remove(0);

    println!("second estim: {} \n", sec_estim.len());
    
    println!("more correct solutions:");
    for res in sec_estim {
        println!("one of solutions Q:{} N:{} key:{:?} \n", res.0, res.1, res.2);
    }
    
}
