use itertools::Itertools;
use std::collections::HashSet;

const NUM_SQUAD: usize = 11;

pub fn calc_rating(ratings: Vec<u32>) -> u32 {
    let num = ratings.len();
    assert_eq!(num, NUM_SQUAD);

    let sum = ratings.iter().sum::<u32>();
    let avg = sum as f32 / num as f32;
    let above_sum = ratings
        .into_iter()
        .map(|x| if x as f32 > avg { x as f32 - avg } else { 0f32 })
        .sum::<f32>();
    let rating = (sum as f32 + above_sum).round() / num as f32;

    rating as u32
}

pub fn generate(
    have: &Vec<u32>,
    cand: &Vec<u32>,
    target: u32,
    records_limit: usize,
) -> Vec<Vec<u32>> {
    assert!(have.len() <= NUM_SQUAD);

    let remain = NUM_SQUAD - have.len();
    let mut visited = HashSet::new();
    let mut possibles = Vec::new();
    let mut sorted_cand = cand.clone();
    sorted_cand.sort();

    for bench in (0..remain).map(|_| sorted_cand.clone()).multi_cartesian_product() {
        let mut squad = have.clone();
        squad.extend(&bench);

        let mut bench_sorted = bench.clone();
        bench_sorted.sort();

        if visited.contains(&bench_sorted) {
            continue;
        }

        visited.insert(bench_sorted.clone());

        if calc_rating(squad) == target {
            possibles.push(bench_sorted);

            if possibles.len() > records_limit {
                break;
            }
        }
    }

    possibles
}

pub fn show(cand: Vec<u32>, possibles: Vec<Vec<u32>>) {
    let mut cand = cand;
    cand.sort();

    let header = cand
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\t");
    println!("{}", header);
    println!("{}", "-".repeat((cand.len() - 1) * 8 + header.len()));

    for ratings in possibles {
        let row = cand
            .iter()
            .map(|x| ratings.iter().filter(|&y| y == x).count().to_string())
            .collect::<Vec<_>>()
            .join("\t");
        println!("{}", row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_calc_rating() {
        {
            let v = vec![81; 11];
            assert_eq!(81, calc_rating(v));
        }

        {
            let v = vec![85, 86, 86, 86, 87, 87, 87, 87, 87, 87, 88];
            assert_eq!(87, calc_rating(v));
        }
    }

    #[test]
    pub fn test_generate() {
        let have = vec![88, 87, 87, 86, 85, 85];
        let cand = vec![85, 84, 83];
        let target = 86;
        let limits = 10;

        let res = generate(&have, &cand, target, limits);

        assert_eq!(res.len(), 9);
        assert_eq!(res[0], vec![83, 83, 85, 85, 85]);
        assert_eq!(res[1], vec![83, 84, 84, 85, 85]);
        assert_eq!(res[2], vec![83, 84, 85, 85, 85]);
        assert_eq!(res[3], vec![83, 85, 85, 85, 85]);
        assert_eq!(res[4], vec![84, 84, 84, 84, 85]);
        assert_eq!(res[5], vec![84, 84, 84, 85, 85]);
        assert_eq!(res[6], vec![84, 84, 85, 85, 85]);
        assert_eq!(res[7], vec![84, 85, 85, 85, 85]);
        assert_eq!(res[8], vec![85, 85, 85, 85, 85]);
    }
}