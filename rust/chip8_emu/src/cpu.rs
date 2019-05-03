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
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let kk = ((opcode & 0x00FF)) as u8;
        let nnn = ((opcode) & 0x0FFF);
        match (op, x, y, n){
            //Don't need to implement SYS (dead)
            //CLS
            (0, 0, 0xE, 0) => self.display.cls(),
            //RET
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stacks[self.sp];
            },
            //JP addr
            (1, _, _, _) => {
                self.pc = nnn;
            },
            //CALL addr
            (2, _, _, _) => {
                self.sp += 1;
                self.stacks[self.sp - 1] = self.pc;
                self.pc = nnn;
            },
            //SE Vx, byte
            (3, _, _, _) => {
                if self.registers[x] == kk {
                    self.pc += 2;
                }
            },
            //SNE Vx, byte
            (4, _, _, _) => {
                if self.registers[x] != kk {
                    self.pc += 2;
                }
            },
            //SE Vx, Vy
            (5, _, _, _) => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }
            _ => {
                //igonre it
            }
        }
    }
}