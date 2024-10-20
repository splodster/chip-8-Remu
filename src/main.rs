pub struct Cpu {
    v: [u8; 16],        // 8 bit general purpose register array
    i: u16,             // 16 bit index register
    pc: u16,            // Program counter
    sp: u16,            // stack pointer
    stack: [u16; 16],   // stack
    memory: [u8; 4096], // 4kb memory
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            v: [0; 16],
            i: 0,
            pc: 0x200, // chip-8 begins at 0x200
            sp: 0,
            stack: [0; 16],
            memory: [0; 4096],
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}

fn execute(cpu: &mut Cpu, opc: u16) {
    match opc & 0xF000 {
        // match the first 4 bits
        0x6000 => {
            // LD Vx, byte
            let x: u8 = ((opc & 0x0F00) >> 8) as u8; // Vx register
            let y: u8 = (opc & 0x00FF) as u8; // load y Vx
            cpu.v[x as usize] = y;
        }
        _ => println!("{:X} unknown", opc),
    }
}

fn main() {
    let mut cpu = Cpu::new();
    execute(&mut cpu, 0x65B4);
}
