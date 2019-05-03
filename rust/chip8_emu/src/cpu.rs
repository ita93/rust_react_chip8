/*Define CPU struct. it will contain every element of a Chip8 Emu.
* Include : 
* - 4kB memory
* - 16 8-bit GP registers
* - A keyboard
* - A Display memory (Graphic buffer - something's like it).
* - A program counter.
* - An Index register. //This register no more in modern arch
* - A delay timer. 
* - A sound timer.
* - We also need a stack and a stack pointer to save the current address
*   before we perform a jump, so we can be back to a overpassed addr easily.
*/
    
extern crate rand;
use rand::Rng;
use keyboard::Keyboard;
use display::Display;
use font::FONT_SET;

const MEMORY_SIZE: usize = 4096;
const NO_REGISTERS: usize = 16;

pub struct CPU{
    memory: [u8; MEMORY_SIZE],
    registers: [u8; NO_REGISTERS],
    keyboard: Keyboard,
    display: Display,
    pc: u16,
    ir: u16, //index register,
    stacks: [u16; 16], //Why stack 16bits? Because it need enough space to save memory addresses.
    sp: u8, //stack pointer
    delay_timer: u8,
    //Skip timer for now, I will be back later.
}

impl CPU{
    pub fn new() -> Self{
        CPU{
            memory: [0; MEMORY_SIZE],
            registers: [0; NO_REGISTERS],
            keyboard: Keyboard::new(),
            display: Display::new(),
            pc: 0,
            ir: 0, //index register,
            stacks: [0; 16],
            sp: 0, //stack pointer
            delay_timer: 0,
        }
    }

    pub fn reset(&mut self){
        self.memory = [0; MEMORY_SIZE];
        self.registers = [0; NO_REGISTERS];
        self.display.cls();
        self.pc = 0;
        self.ir = 0;
        self.stacks = [0; 16];
        self.sp = 0;
        self.delay_timer = 0;
    }

    //Fetch instruction phase
    //Need to read 16 bit because each instruction is presented as a word.NO_REGISTERS
    //http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0
    pub fn read_instruction(&mut self, addr: u16) -> u16{
        ((self.memory[addr as usize] as u16) << 8) |
            (self.memory[(addr + 1) as usize] as u16)
    }

    pub fn delay_desc(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    //Execute instruciton phase
    pub fn execute_ops(&mut self, opcode: u16){
        /*we need to get these params:
        nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
        n or nibble - A 4-bit value, the lowest 4 bits of the instruction
        x - A 4-bit value, the lower 4 bits of the high byte of the instruction
        y - A 4-bit value, the upper 4 bits of the low byte of the instruction
        kk or byte - An 8-bit value, the lowest 8 bits of the instruction
        */
        let op = ((opcode & 0xF000)>>12) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let kk = ((opcode & 0x00FF)) as u8;
        let nnn = (opcode) & 0x0FFF;
        match (op, x, y, n){
            //Don't need to implement SYS (dead)
            //CLS
            (0, 0, 0xE, 0) => self.display.cls(),
            //RET
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stacks[self.sp as usize];
            },
            //JP addr
            (1, _, _, _) => {
                self.pc = nnn;
            },
            //CALL addr
            (2, _, _, _) => {
                self.sp += 1;
                self.stacks[self.sp as usize - 1] = self.pc;
                self.pc = nnn;
            },
            //SE Vx, byte
            (3, _, _, _) => {
                self.pc += if self.registers[x] == kk {2} else {0};
            },
            //SNE Vx, byte
            (4, _, _, _) => {
                self.pc += if self.registers[x] != kk {2} else {0};
            },
            //SE Vx, Vy
            (5, _, _, _) => {
                self.pc += if self.registers[x] == self.registers[y] {2} else {0};
            }
            //LD Vx, byte
            (6, _, _, _) => {
                self.registers[x] = kk;
            }
            //ADD Vx, byte
            (7, _, _, _) => {
                self.registers[x] += kk;
            },
            //LD Vx, Vy
            (8, _, _, 0) => {
                self.registers[x] = self.registers[y];
            },
            //OR Vx, Vy
            (8, _, _, 1) => {
                self.registers[x] |= self.registers[y];
            },
            // AND Vx, Vy
            (8, _, _, 2) => {
                self.registers[x] &= self.registers[y];
            },
            //XOR Vx, Vy
            (8, _, _, 3) => {
                self.registers[x] ^= self.registers[y];
            },
            //ADD Vx, Vy
            (8, _, _, 4) => {
                let sum:u16 = self.registers[x] as u16 + self.registers[y] as u16;
                self.registers[x] = (sum & 0x00FF) as u8;
                self.registers[0xF] = if sum > 255 {1} else {0};
            },
            //SUB Vx, Vy
            (8, _, _, 5) => {
                let r1 = self.registers[x];
                let r2 = self.registers[y];
                self.registers[0xF] = if r1 > r2 {1} else {0};
                let sub_res = r1 as u16 - r2 as u16;
                self.registers[x] = (sub_res & 0x00FF) as u8;
            },
            //SHR Vx {, Vy}
            (8, _, _, 6) => {
                self.registers[0xF] = self.registers[x] & 0x01;
                self.registers[x] = self.registers[x] >> 1;
            },
            //SUBN Vx, Vy
            (8, _, _, 7) => {
                let res = self.registers[y] as i8 - self.registers[x] as i8;
                self.registers[x] = res as u8;
                self.registers[0xF] = if res < 0 { 1 } else { 0 };
            },
            //SHL Vx {, Vy}
            (8, _, _, 0xE) => {
                self.registers[0xF] = self.registers[x] & 0x10;
                self.registers[x] = self.registers[x] << 1;
            },
            //SNE Vx, Vy
            (9, _, _, 0) => {
                self.pc += if self.registers[x] != self.registers[y] {2} else {0}
            },
            //LD I, addr
            (0xA, _, _, _) => {
                self.ir = nnn;
            },
            //JP V0, addr
            (0xB, _, _, _) => {
                self.pc += self.registers[0] as u16 + nnn;
            },
            //RND Vx, byte 
            (0xC, _, _, _) => {
                /*The interpreter generates a random number from 0 to 255,
                which is then ANDed with the value kk. The results are 
                stored in Vx. See instruction 8xy2 for more information on AND.*/
                let mut rng = rand::thread_rng();
                self.registers[x] = rng.gen::<u8>() & kk;
            },
            //DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                let sprites = &self.memory[self.ir as usize..(self.ir + n as u16) as usize];
                self.registers[0xF] = if self.display.draw(self.registers[x] as usize, self.registers[y] as usize, sprites) {
                    1
                } else {
                    0
                };
            },
            //SKP Vx
            (0xE, _, 9, 0xE) => {
                self.pc += if self.keyboard.is_key_down(x as usize) {2} else {0};
            },
            //SKNP Vx
            (0xE, _, 0xA, 1) => {
                self.pc += if !self.keyboard.is_key_down(x as usize) {2} else {0};
            },
            //LD Vx, DT
            (0xF, _, 0, 7) => {
                self.registers[x] = self.delay_timer;
            },
            //LD Vx, K 
            (0xF, _, 0, 0xA) => {
                //Wait till K pressed, then store value key in REG X.
            },
            //LD DT, Vx 
            (0xF, _, 1, 5) => {
                self.delay_timer = self.registers[x];
            },
            //LD ST, Vx
            (0xF, _, 1, 8) => {
                //ignore it for now.
            },
            //ADD I, Vx
            (0xF, _, 1, 0xE) => {
                self.ir += self.registers[x] as u16;
            },
            //LD F, Vx 
            (0xF, _, 2, 9) => {
                self.ir = self.registers[x] as u16 * 5;
            },
            //LD B, Vx
            (0xF, _, 3, 3) => {
                self.memory[self.ir as usize] = self.registers[x] / 100;
                self.memory[self.ir as usize + 1] = (self.registers[x] / 10) % 10;
                self.memory[self.ir as usize + 2] = (self.registers[x] % 100) % 10;
            },
            //LD [I], Vx
            //[I] means value that stored at addr I.
            (0xF, _, 5, 5) => {
                self.memory[self.ir as usize..=(self.ir as usize + x)]
                    .copy_from_slice(&self.registers[0..=x]);
            },
            //LD Vx, [I]
            (0xF, _, 6, 5) => {
                self.registers[0..=x].
                    copy_from_slice(&self.memory[self.ir as usize..=(self.ir as usize + x)]);
            }
            _ => {
                //igonre it
            }
        }
    }
}
