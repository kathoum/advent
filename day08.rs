use std::io::{Read, Seek};

trait Image {
    fn count_equal(&self, val: u8) -> usize;
    fn print(&self, wid: usize, hei: usize);
}

impl Image for [u8] {
    fn count_equal(&self, val: u8) -> usize {
        self.iter().filter(|&x| *x == val).count()
    }
    fn print(&self, wid: usize, hei: usize) {
        for h in 0..hei {
            for w in 0..wid {
                print!("{}", match self[h * wid + w] {
                    b'2' => ' ',
                    b'1' => '#',
                    b'0' => ' ',
                    _ => '?',
                });
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn main() -> std::io::Result<()> {
    let mut input = std::fs::File::open("input08.txt")?;
    const W: usize = 25;
    const H: usize = 6;
    let mut layer = [0u8; W * H];
    let mut least_zeros = layer.len();
    let mut check = 0;
    while input.read_exact(&mut layer).is_ok() {
        let zeros = layer.count_equal(b'0');
        if zeros < least_zeros {
            least_zeros = zeros;
            check = layer.count_equal(b'1') * layer.count_equal(b'2');
        }
    }
    println!("Check: {}", check);

    let mut image = [b'2'; W * H];
    input.seek(std::io::SeekFrom::Start(0))?;
    while input.read_exact(&mut layer).is_ok() {
        for (x, y) in image.iter_mut().zip(layer.iter()) {
            if *x == b'2' {
                *x = *y;
            }
        }
    }
    image.print(W, H);

    Ok(())
}
