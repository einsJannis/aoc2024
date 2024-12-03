pub fn compute(input: Box<str>) -> i64 {
    let mut a = input
        .split("\n")
        .filter_map(|it| {
            if it == "" {
                None
            } else {
                let mut e = it
                    .splitn(2, "   ")
                    .map(|num| i64::from_str_radix(num, 10).expect("invalid input"));
                Some((
                    e.next().expect("invalid input"),
                    e.next().expect("invalid input"),
                ))
            }
        })
        .unzip::<i64, i64, Vec<i64>, Vec<i64>>();
    a.0.sort();
    a.1.sort();
    return a
        .0
        .into_iter()
        .zip(a.1.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
}
