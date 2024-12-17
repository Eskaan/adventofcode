fn main() {
    let input = std::fs::read_to_string("input/day17.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let split: Vec<u8> = lines[4][9..]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut vm = VM {
        stack: split.clone(),
        out: Vec::new(),
        pc: 0,
        reg_a: lines[0][12..].parse().unwrap(),
        reg_b: lines[1][12..].parse().unwrap(),
        reg_c: lines[2][12..].parse().unwrap(),
    };

    vm.run();

    println!("{}", vm.out());
}

#[derive(Debug, Clone)]
struct VM {
    stack: Vec<u8>,
    out: Vec<u8>,
    pc: usize,
    reg_a: u128,
    reg_b: u128,
    reg_c: u128,
}

impl VM {
    fn run(&mut self) {
        while let Some(&opcode) = self.stack.get(self.pc) {
            let combo = *self.stack.get(self.pc + 1).expect("Inalid state!");
            match opcode {
                0 => self.reg_a = self.reg_a / (2u128.pow(self.get_combo(combo) as u32)),
                6 => self.reg_b = self.reg_a / (2u128.pow(self.get_combo(combo) as u32)),
                7 => self.reg_c = self.reg_a / (2u128.pow(self.get_combo(combo) as u32)),
                3 => {
                    if self.reg_a != 0 {
                        self.pc = combo as usize;
                        continue;
                    }
                }
                2 => self.reg_b = self.get_combo(combo) & 0b111,
                5 => self.out.push((self.get_combo(combo) & 0b111) as u8),
                1 => self.reg_b ^= combo as u128,
                4 => self.reg_b ^= self.reg_c,
                _ => unreachable!("Invalid opcode!"),
            }
            self.pc += 2;
        }
    }

    fn get_combo(&self, combo: u8) -> u128 {
        match combo {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => unreachable!("Invalid reserved code!"),
            _ => unreachable!("Invalid combo code!"),
        }
    }

    fn out(&self) -> String {
        let mut out: String = self.out.iter().map(|i| format!("{i},")).collect();
        out.pop();
        out
    }
}
