pub fn biggest_gap(mut values:Vec<f64>) -> Option<(f64,f64)> {
    if values.len() < 2 {
        return None;
    }

    // Ensure the values are sorted, handling NaN values gracefully
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Initialize the maximum gap found to zero
    let mut max_gap = 0.0;

    let mut res=(0.0,0.0);

    // Iterate over the sorted values to find the maximum gap
    for window in values.windows(2) {
        if let [a, b] = window {
            let gap = b - a;
            if gap > max_gap {
                res=(a.clone(),b.clone());
                max_gap = gap;
            }
        }
    }
    Some(res)
}


pub fn biggest_gap_per(mut values:Vec<f64>, period:f64) -> Option<(f64,f64)> {

    assert!(period>0.0);

    for vali in values.clone() {
        assert!(vali>=0.0);
        assert!(vali<period);
    }

    if values.len() < 2 {
        return None;
    }

    // Ensure the values are sorted, handling NaN values gracefully
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Initialize the maximum gap found gap over the period boundary
    let largest=values.last().unwrap().clone();
    let mut max_gap = values[0]+(period-largest);
    let mut res=(largest,values[0]);
    // Iterate over the sorted values to find the maximum gap
    for window in values.windows(2) {
        if let [a, b] = window {
            let gap = b - a;
            if gap > max_gap {
                res=(a.clone(),b.clone());
                max_gap = gap;
            }
        }
    }
    Some(res)
}