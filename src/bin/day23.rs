use std::sync::mpsc;
type Int = advent::intcode::Integer;
enum Nat { Idle(usize), Active(usize), Data(Int, Int) }

struct State {
    network_id: usize,
    input: mpsc::Receiver<Int>,
    output: Vec<mpsc::Sender<Int>>,
    output_nat: mpsc::Sender<Nat>,
    buffer: Vec<Int>,
    read_attempts: usize,
}

impl State {
    fn new(network_id: usize, input: mpsc::Receiver<Int>, outputs: &[mpsc::Sender<Int>], output_nat: mpsc::Sender<Nat>) -> State {
        State {
            network_id,
            input,
            output: outputs.iter().cloned().collect(),
            output_nat,
            buffer: Vec::new(),
            read_attempts: 0
        }
    }

    fn notify_active(&self) {
        self.output_nat.send(Nat::Active(self.network_id)).unwrap()
    }

    fn notify_idle(&self) {
        self.output_nat.send(Nat::Idle(self.network_id)).unwrap()
    }
}

impl advent::intcode::State for State {
    fn input(&mut self) -> Int {
        match self.input.try_recv() {
            Ok(value) => {
                //println!("{} reads {}", self.network_id, value);
                self.read_attempts = 0;
                self.notify_active();
                value
            },
            Err(mpsc::TryRecvError::Empty) => {
                self.read_attempts += 1;
                if self.read_attempts > 1000 {
                    self.read_attempts = 0;
                    self.notify_idle();
                }
                -1
            },
            Err(mpsc::TryRecvError::Disconnected) => panic!()
        }
    }

    fn output(&mut self, value: Int) -> () {
        self.notify_active();
        self.buffer.push(value);
        if self.buffer.len() == 3 {
            let destination = self.buffer[0] as usize;
            let (x, y) = (self.buffer[1], self.buffer[2]);
            self.buffer.clear();

            //println!("{} sends {} {} to {}", self.network_id, x, y, destination);
            if destination != 255 {
                self.output[destination].send(x).unwrap();
                self.output[destination].send(y).unwrap();
                self.output_nat.send(Nat::Active(destination)).unwrap();
            } else {
                self.output_nat.send(Nat::Data(x, y)).unwrap();
            }
        }
    }
}

fn main() {
    let input = include_str!("input23.txt");
    let (mut tx, mut rx) = (Vec::new(), Vec::new());
    for _ in 0..50 {
        let (t, r) = mpsc::channel();
        tx.push(t);
        rx.push(r);
    }
    let (tx, rx) = (tx, rx);

    let (nat_tx, nat) = mpsc::channel();
    let mut activity = [true; 50];
    let mut activity_count = activity.len();

    for (id, rx) in rx.into_iter().enumerate() {
        let state = State::new(id, rx, &tx, nat_tx.clone());
        tx[id].send(id as Int).unwrap();
        std::thread::spawn(move || {
            let reader = std::io::Cursor::new(&input);
            let mut program = advent::intcode::read_program(reader).unwrap();
            let mut state = state;
            advent::intcode::run_program(&mut program, &mut state).unwrap();
        });
    }

    let mut nat_data = None;
    for msg in nat {
        match msg {
            Nat::Idle(id) => if activity[id] {
                activity[id] = false;
                activity_count -= 1;
                //println!("{} is idle; active count {}", id, activity_count)
            },
            Nat::Active(id) => if !activity[id] {
                activity[id] = true;
                activity_count += 1;
                //println!("{} is active; active count {}", id, activity_count)
            },
            Nat::Data(x, y) => {
                if nat_data.is_none() {
                    println!("First message sent to NAT: {} {}", x, y);
                }
                //println!("Message sent to NAT: {} {}", x, y);
                nat_data = Some((x, y));
            }
        };
        if activity_count == 0 {
            if let Some((x, y)) = nat_data {
                println!("Everything idle; sending {} {}", x, y);
                tx[0].send(x).unwrap();
                tx[0].send(y).unwrap();
                activity[0] = true;
                activity_count += 1;
            } else {
                println!("Everything idle; waiting...");
            }
        }
    }
}
