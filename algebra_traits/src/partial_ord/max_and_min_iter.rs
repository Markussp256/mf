use container_traits::Iter;
use utils::CumSum;

fn is_nan<T:PartialEq>(f:&T) -> bool {
    f != f
}

// fn ordering<T:PartialOrd>(lhs:&T, rhs:&T) -> Ordering {
//     if lhs == rhs {
//         Ordering::Equal
//     } else if lhs > rhs || is_nan(&rhs) {
//         Ordering::Greater // also happens when rhs is nan and lhs is not nan
//     } else {
//         // also happens when both are nan
//         Ordering::Less
//     }
// }

fn update_max<'a, T:PartialOrd>(new_item:&'a T, v:& mut Vec<&'a T>) {
    if is_nan(new_item) {
        return;
    }
    if !v.is_empty() && new_item > v[0] {
        v.clear();
    }
    if v.is_empty() || new_item >= v[0] {
        v.push(new_item);
    }
}

fn update_min<'a, T:PartialOrd>(new_item:&'a T, v:& mut Vec<&'a T>) {
    if is_nan(new_item) {
        return;
    }
    if !v.is_empty() && new_item < v[0] {
        v.clear();
    }
    if v.is_empty() || new_item <= v[0] {
        v.push(new_item);
    }
}

pub trait MaxesOfIter<T: PartialOrd> : Sized {
    fn maxes(&self) -> Vec<&T>;
    fn indices_of_maxes(& self) -> Vec<usize>;
}

impl<T:'static+PartialOrd, I:Iter<T>> MaxesOfIter<T> for I {
    fn maxes(&self) -> Vec<&T> {
        let mut v=Vec::new();
        self.iter()
            .for_each(|new_item|update_max(new_item,&mut v));
        v
    }
    fn indices_of_maxes(& self) -> Vec<usize> {
        let mut iter=self.iter();
        self.maxes()
            .into_iter()
            .map(|t|iter.position(|t_rhs|t_rhs == t).unwrap())
            .cum_sum()
            .collect()
    }
}

pub trait MinsOfIter<T: 'static+PartialOrd> : Sized {
    fn mins(&self) -> Vec<&T>;
    fn indices_of_mins(& self) -> Vec<usize>;
}

impl<T:'static+PartialOrd, I:Iter<T>> MinsOfIter<T> for I {
    fn mins(&self) -> Vec<&T> {
        let mut v=Vec::new();
        self.iter()
            .for_each(|new_item|update_min(new_item,&mut v));
        v
    }
    fn indices_of_mins(& self) -> Vec<usize> {
        let mut iter=self.iter();
        self.mins()
            .into_iter()
            .map(|t|iter.position(|t_rhs|t_rhs == t).unwrap())
            .cum_sum()
            .collect()
    }
}


#[test]
fn test_max() {
    let v=vec![f64::INFINITY, 1.0, f64::NAN];
    assert_eq!(v.indices_of_mins(), vec![1]);
}