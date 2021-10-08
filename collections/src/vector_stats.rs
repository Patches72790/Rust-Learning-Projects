use std::collections::HashMap;

pub fn mean_from_vector(v: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    
    for i in v.iter() {
        sum += i;
    }
    
    return sum / (v.len()) as i32;
}

pub fn median_from_vector() {

}

pub fn mode_from_vector(v: Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for i in v.iter() {
        let count = hm.entry(i).or_insert(0);
        *count += 1; 
    }

    let mut max = -1;
    for (k, v) in &hm {
        if v > &max {
            max = **k;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_1() {
        let a = mean_from_vector(vec![1, 2, 3, 4]);
        assert_eq!(a, 2);
    }

    fn test_median_1() {}

    #[test]
    fn test_mode_1() {
        let a = mode_from_vector(vec![1, 1, 1, 2, 3, 3, 4, 4, 5, 5, 5, 5]);
        assert_eq!(a, 5);
    }
}
