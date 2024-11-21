use std::{collections::BTreeMap, mem::swap};

pub fn manhattan_mst(mut xs: Vec<i64>, mut ys: Vec<i64>) -> Vec<(usize, usize)> {
    let n = xs.len();
    let mut idx: Vec<_> = (0..n).collect();
    let mut ret = vec![];
    for s in 0..2 {
        for t in 0..2 {
            idx.sort_by(|i, j| (xs[*i] + ys[*i]).cmp(&(xs[*j] + ys[*j])));
            let mut sweep = BTreeMap::new();
            for i in idx.iter() {
                let mut to_move = vec![];
                for (y_pos, j) in sweep.range(-ys[*i]..) {
                    if xs[*i] - xs[*j] < ys[*i] - ys[*j] {
                        break;
                    }
                    to_move.push((*y_pos, *j));
                    ret.push((*i, *j));
                }
                for (key, _) in to_move {
                    sweep.remove(&key);
                }
                sweep.insert(-ys[*i], *i);
            }
            swap(&mut xs, &mut ys);
        }
        for x in xs.iter_mut() {
            *x = -(*x);
        }
    }
    ret
}

#[test]
fn test_manhattan_mst() {
    manhattan_mst(vec![0, 0, 10, 10], vec![0, 10, 0, 10]);
}
