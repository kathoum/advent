fn main() {
    let r1 = 134792;
    let r2 = 675810;
    let mut c = 0;
    for d1 in 1 .. 7 {
        for d2 in d1 .. 10 {
            for d3 in d2 .. 10 {
                for d4 in d3 .. 10 {
                    for d5 in d4 .. 10 {
                        for d6 in d5 .. 10 {
                            if d1 == d2 && d2 != d3
                            || d2 == d3 && d1 != d2 && d3 != d4
                            || d3 == d4 && d2 != d3 && d4 != d5
                            || d4 == d5 && d3 != d4 && d5 != d6
                            || d5 == d6 && d4 != d5 {
                                let n = d6 + d5 * 10 + d4 * 100 + d3 * 1000 + d2 * 10000 + d1 * 100000;
                                if r1 <= n && n <= r2 {
                                    c += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!{"{}", c};
}