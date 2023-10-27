use std::collections::HashSet;
fn functional_push_left(acc: Vec<u32>, x: u32) -> Vec<u32> {
    [x].into_iter().chain(acc.into_iter()).collect()
}

fn get_multiples_until_limit_rec(limit: u32, x: u32, current: u32, acc: Vec<u32>) -> Vec<u32> {
    if current == 0 {
        return vec![];
    }
    if current >= limit {
        return acc;
    }
    let new_acc = functional_push_left(acc, current);
    get_multiples_until_limit_rec(limit, x, current + x, new_acc)

    // acc
}

fn get_multiples_until_limit(limit: u32, x: u32) -> Vec<u32> {
    get_multiples_until_limit_rec(limit, x, x, vec![])
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let multiples: HashSet<u32> = HashSet::from_iter(
        factors
            .into_iter()
            .map(|x| get_multiples_until_limit(limit, *x))
            .flatten(),
    );
    let sum = multiples.into_iter().sum();
    sum
    // 0
}
