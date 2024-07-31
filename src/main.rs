use std::collections::HashMap;
use itertools::Itertools;
use rayon::prelude::*;

const NUMBER_OF_COINS: usize = 5;
const RANGE_LIMIT: usize = 100;

type CoinsType = [usize; NUMBER_OF_COINS - 1];

fn main() {
    let range_combinations: Vec<usize> = (2..=RANGE_LIMIT).collect();
    let base_moedas_utilizadas: [[usize; NUMBER_OF_COINS]; RANGE_LIMIT + 1] =
        [[0; NUMBER_OF_COINS]; RANGE_LIMIT + 1];
    let min_value: usize = 400;

    let combinations: Vec<CoinsType> = range_combinations
        .into_iter()
        .combinations(NUMBER_OF_COINS - 1)
        .map(|combo| {
            let slice: &[_] = &combo;
            <CoinsType>::try_from(slice).expect("Slice with incorrect length")
        })
        .collect();

    let a: HashMap<CoinsType, usize> = combinations
        .into_par_iter()
        .filter_map(|new_moedas| {
            let mut moedas_utilizadas = base_moedas_utilizadas.clone();

            for v in 1..=RANGE_LIMIT {
                let mut cur_v: usize = v;
                let counter = &mut moedas_utilizadas[v];
                for (pos, &m) in new_moedas.iter().rev().enumerate() {
                    let mut qtd = 0;
                    while m <= cur_v {
                        cur_v -= m;
                        qtd += 1;
                    }
                    counter[pos] = qtd;
                }
                counter[NUMBER_OF_COINS - 1] = cur_v;
            }

            let v = moedas_utilizadas
                .iter()
                .skip(1)
                .map(|v| v.iter().sum::<usize>())
                .sum::<usize>();

            if v <= min_value {
                Some((new_moedas, v))
            } else {
                None
            }
        })
        .collect();

    // Step 1: Find the minimum value
    let min_value = a.values().min().cloned();

    // Step 2: Collect all (key, value) pairs with the minimum value
    let min_pairs: Vec<(&CoinsType, &usize)> = match min_value {
        Some(min) => a.iter().filter(|&(_, &v)| v == min).collect(),
        None => Vec::new(),
    };

    // Print the result
    for (key, value) in min_pairs {
        println!("Key: {:?}, Value: {:?}", key, (*value as f32) / 100.0);
    }
}
