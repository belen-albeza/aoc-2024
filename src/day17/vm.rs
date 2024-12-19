#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
#[repr(u8)]
enum Opcode {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

struct Operand(u8);

impl Operand {
    fn literal(&self) -> u32 {
        self.0 as u32
    }

    fn combo(&self, registers: [u32; 3]) -> u32 {
        match self.0 {
            0..=3 => self.0 as u32,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    registers: [u32; 3],
    ip: usize,
    rom: Vec<u8>,
    out: Vec<u32>,
}

impl VM {
    pub fn new(registers: [u32; 3], rom: Vec<u8>) -> Self {
        Self {
            registers,
            rom,
            ip: 0,
            out: vec![],
        }
    }

    pub fn set_register(&mut self, idx: usize, value: u32) {
        self.registers[idx] = value;
    }

    pub fn rom(&self) -> Vec<u8> {
        self.rom.to_owned()
    }

    pub fn run(&mut self) {
        // println!("{:?}", self);
        while let Some(raw_opcode) = self.read_byte() {
            match Opcode::from(raw_opcode) {
                Opcode::Adv => self.exec_adv(),
                Opcode::Bxl => self.exec_bxl(),
                Opcode::Bst => self.exec_bst(),
                Opcode::Jnz => self.exec_jnz(),
                Opcode::Bxc => self.exec_bxc(),
                Opcode::Out => self.exec_out(),
                Opcode::Bdv => self.exec_bdv(),
                Opcode::Cdv => self.exec_cdv(),
            }
            // println!("\n{:?}", self);
        }
    }

    pub fn output(&self) -> Vec<u32> {
        self.out.to_owned()
    }

    fn read_byte(&mut self) -> Option<u8> {
        let byte = self.rom.get(self.ip)?;
        self.ip += 1;

        Some(*byte)
    }

    fn read_combo_operand(&mut self) -> u32 {
        Operand(self.read_byte().unwrap()).combo(self.registers)
    }

    fn read_literal_operand(&mut self) -> u32 {
        Operand(self.read_byte().unwrap()).literal()
    }

    fn exec_adv(&mut self) {
        self.registers[0] = self.division();
    }

    fn exec_bxl(&mut self) {
        let operand = self.read_literal_operand();
        self.registers[1] ^= operand;
    }

    fn exec_bst(&mut self) {
        let operand = self.read_combo_operand();
        self.registers[1] = operand % 8;
    }

    fn exec_jnz(&mut self) {
        let operand = self.read_literal_operand();
        if self.registers[0] != 0 {
            self.ip = operand as usize;
        }
    }

    fn exec_bxc(&mut self) {
        _ = self.read_byte();

        self.registers[1] ^= self.registers[2];
    }

    fn exec_out(&mut self) {
        let operand = self.read_combo_operand();
        self.out.push(operand % 8);
    }

    fn exec_bdv(&mut self) {
        self.registers[1] = self.division();
    }

    fn exec_cdv(&mut self) {
        self.registers[2] = self.division();
    }

    fn division(&mut self) -> u32 {
        let operand = self.read_combo_operand();
        let numerator = self.registers[0];
        let denominator = 2_u32.pow(operand);

        numerator / denominator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vm(rom: Vec<u8>) -> VM {
        VM::new([0; 3], rom)
    }

    #[test]
    fn adv_opcode() {
        let mut vm = vm(vec![0, 5]);
        vm.registers[0] = 16;
        vm.registers[1] = 3;

        vm.run();

        assert_eq!(vm.registers[0], 2);
        assert_eq!(vm.ip, 2);
    }

    #[test]
    fn bxl_opcode() {
        let mut vm = vm(vec![1, 0b0000_0101]);
        vm.registers[1] = 0b0000_1100;

        vm.run();

        assert_eq!(vm.ip, 2);
        assert_eq!(vm.registers[1], 0b0000_1001);
    }

    #[test]
    fn bst_opcode() {
        let mut vm = vm(vec![2, 4]);
        vm.registers[0] = 10;

        vm.run();

        assert_eq!(vm.ip, 2);
        assert_eq!(vm.registers[1], 2);
    }

    #[test]
    fn jnz_opcode_zero() {
        let mut vm = vm(vec![3, 0]);
        vm.registers[0] = 0;

        vm.run();

        assert_eq!(vm.ip, 2);
    }

    #[test]
    fn jnz_opcode_nonzero() {
        let mut vm = vm(vec![3, 8]);
        vm.registers[0] = 1;

        vm.run();

        assert_eq!(vm.ip, 8);
    }

    #[test]
    fn bxc_opcode() {
        let mut vm = vm(vec![4, 0]);
        vm.registers[1] = 0b0000_1100;
        vm.registers[2] = 0b0000_0101;

        vm.run();

        assert_eq!(vm.ip, 2);
        assert_eq!(vm.registers[1], 0b0000_1001);
    }

    #[test]
    fn out_opcode() {
        let mut vm = vm(vec![5, 6]);
        vm.registers[2] = 10;

        vm.run();

        assert_eq!(vm.ip, 2);
        assert_eq!(vm.out, vec![2]);
    }

    #[test]
    fn bdv_opcode() {
        let mut vm = vm(vec![6, 5]);
        vm.registers[0] = 16;
        vm.registers[1] = 3;

        vm.run();

        assert_eq!(vm.registers[1], 2);
        assert_eq!(vm.ip, 2);
    }

    #[test]
    fn cdv_opcode() {
        let mut vm = vm(vec![7, 5]);
        vm.registers[0] = 16;
        vm.registers[1] = 3;

        vm.run();

        assert_eq!(vm.registers[2], 2);
        assert_eq!(vm.ip, 2);
    }
}
