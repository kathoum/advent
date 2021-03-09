fn main() {
    let input = include_str!("input02.txt");

    let boxes: Vec<(u32, u32, u32)> = input.lines().map(|line| {
        let n: Vec<&str> = line.split('x').collect();
        match n.as_slice() {
            [l, w, h] => {
                let l = l.parse().unwrap();
                let w = w.parse().unwrap();
                let h = h.parse().unwrap();
                (l, w, h)
            }
            _ => panic!("Error parsing line {}", line)
        }
    }).collect();

    let area = boxes.iter().map(|&(l, w, h)| required_paper(l, w, h)).sum::<u32>();
    println!("The total area is {} square feet", area);

    let len = boxes.iter().map(|&(l, w, h)| required_ribbon(l, w, h)).sum::<u32>();
    println!("The total ribbon length is {} feet", len);
}

fn required_paper(l: u32, w: u32, h: u32) -> u32 {
    2*(l*w + w*h + h*l) + l*w*h / l.max(w).max(h)
}

fn required_ribbon(l: u32, w: u32, h: u32) -> u32 {
    2*(l+w+h - l.max(w).max(h)) + l*w*h
}
