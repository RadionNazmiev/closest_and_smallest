use itertools::Itertools;

fn to_weights(mut m:i32) -> i32 {
    let mut sum = 0;
    while m != 0 {
        sum += m % 10;
        m /= 10;
    }
    sum
}

fn closest(s: &str) -> String {
    if s.is_empty() {
        return "".to_string()
    }
    let result = s
        .split_whitespace()
        .enumerate()
        .map(|x| {
            let v = x.1.parse::<i32>().unwrap();
            (to_weights(v), x.0 as i32, v)
        })
        .sorted_by_key(|x| x.0)
        .collect::<Vec<_>>();
    let final_result = result.iter().as_slice().windows(2).min_by_key(|&x|
        (x.get(0).unwrap().0 - x.get(1).unwrap().0).abs()).unwrap().clone();
    format!("[({},{},{})({},{},{})]",final_result[0].0,final_result[0].1,final_result[0].2,final_result[1].0,final_result[1].1,final_result[1].2,)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s: &str, exp: String) -> () {
        let ans = closest(s);
        assert_eq!(ans, exp, "Testing: {}", s);
    }

    #[test]
    fn basic_tests() {
        let mut s = "456899 50 11992 176 272293 163 389128 96 290193 85 52"; // [(13, 9, "85"), (14, 3, "176")]
        testing(s, "[(13,9,85)(14,3,176)]".to_string());
        s = "239382 162 254765 182 485944 134 468751 62 49780 108 54"; // "[[8, 5, 134], [8, 7, 62]]";
        testing(s, "[(8,5,134)(8,7,62)]".to_string());
        s = "241259 154 155206 194 180502 147 300751 200 406683 37 57";
        let r = "[(10,1,154)(10,9,37)]";
        testing(s, r.to_string());

    }
}
