use std::collections::VecDeque;

pub struct Intcode {
    mem: Vec<i64>,
    pc: i64,
    opcode: i64,
    instruction: i64,
    reg: [i64; 3],
    addr: [i64; 3],
    rel_base: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    wait: bool,
}

impl Intcode {
    pub fn new(mem: Vec<i64>) -> Self {
        Self {
            mem,
            pc: 0,
            opcode: 0,
            instruction: 0,
            reg: [0; 3],
            addr: [0; 3],
            rel_base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            wait: false,
        }
    }

    pub fn run(&mut self) {
        self.wait = false;
        loop {
            self.fetch();
            match self.opcode {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.input(),
                4 => self.output(),
                5 => self.jump_true(),
                6 => self.jump_false(),
                7 => self.less_than(),
                8 => self.equals(),
                9 => self.shift_base(),
                99 => break,
                _ => panic!("Unrecognized instruction")
            }
            if self.wait {
                break;
            }
        }
    }

    pub fn set(&mut self, addr: usize, val: i64) {
        self.mem[addr] = val;
    }
    
    pub fn get(&mut self, addr: usize) -> i64 {
        self.mem[addr]
    }
    
    pub fn stdin(&mut self, num: i64) {
        self.input.push_back(num);
    }

    pub fn stdout(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn stdout_all(&mut self) -> Vec<i64> {
        let output: Vec<i64> = self.output.iter().map(|num| *num).collect();
        self.output = VecDeque::new();
        output
    }

    pub fn is_halted(&self) -> bool {
        self.opcode == 99
    }

    fn extend_memory(&mut self, addr: usize) {
        self.mem.resize(addr, 0);
    }

    fn load(&mut self, addr: i64) -> i64 {
        let addr = addr as usize;
        if addr >= self.mem.len() {
            self.extend_memory(addr + 1);
        }
        self.mem[addr]
    }

    fn store(&mut self, addr: i64, val: i64) {
        let addr = addr as usize;
        if addr >= self.mem.len() {
            self.extend_memory(addr + 1);
        }
        self.mem[addr] = val;
    }

    fn fetch(&mut self) {
        self.instruction = self.load(self.pc);
        self.opcode = self.instruction % 100;

    }

    fn load_reg(&mut self, num: usize) {
        for i in 0..num {
            let mode = self.instruction / 10_i64.pow(i as u32 + 2) % 10;
            self.addr[i] = match mode {
                0 | 1 => self.load(self.pc + i as i64 + 1),
                2 => self.load(self.pc + i as i64 + 1) + self.rel_base,
                _ => panic!("Unknown parameter mode"),
            };
            self.reg[i] = match mode {
                1 => self.addr[i],
                0 | 2 => self.load(self.addr[i]),
                _ => panic!("Unknown parameter mode"),
            };
        }
    }

    // Instruction Set

    fn add(&mut self) {
        self.load_reg(3);
        self.store(self.addr[2], self.reg[0] + self.reg[1]);
        self.pc += 4;
    }
    
    fn multiply(&mut self) {
        self.load_reg(3);
        self.store(self.addr[2], self.reg[0] * self.reg[1]);
        self.pc += 4;
    }
    
    fn input(&mut self) {
        self.load_reg(1);
        match self.input.pop_front() {
            Some(val) => {
                self.store(self.addr[0], val);
                self.pc += 2;
            },
            None => self.wait = true
        };
    }
    
    fn output(&mut self) {
        self.load_reg(1);
        self.output.push_back(self.reg[0]);
        self.pc += 2;
    }

    fn jump_true(&mut self) {
        self.load_reg(2);
        if self.reg[0] != 0 {
            self.pc = self.reg[1];
        } else {
            self.pc += 3;
        }
    }

    fn jump_false(&mut self) {
        self.load_reg(2);
        if self.reg[0] == 0 {
            self.pc = self.reg[1];
        } else {
            self.pc += 3;
        }
    }

    fn less_than(&mut self) {
        self.load_reg(3);
        self.store(self.addr[2], if self.reg[0] < self.reg[1] { 1 } else { 0 });
        self.pc += 4;
    }

    fn equals(&mut self) {
        self.load_reg(3);
        self.store(self.addr[2], if self.reg[0] == self.reg[1] { 1 } else { 0 });
        self.pc += 4;
    }

    fn shift_base(&mut self) {
        self.load_reg(1);
        self.rel_base += self.reg[0];
        self.pc += 2;
    }
}