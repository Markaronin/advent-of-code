use itertools::Itertools;

pub fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn clone_column<T: Clone>(v: &Vec<Vec<T>>, x: usize) -> Vec<T> {
    v.iter().map(|row| row[x].clone()).collect_vec()
}
