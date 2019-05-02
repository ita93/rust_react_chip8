/*
Ex9E - SKP Vx
Skip next instruction if key with the value of Vx is pressed.

Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.


ExA1 - SKNP Vx
Skip next instruction if key with the value of Vx is not pressed.

Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.


Fx07 - LD Vx, DT
Set Vx = delay timer value.

The value of DT is placed into Vx.


Fx0A - LD Vx, K
Wait for a key press, store the value of the key in Vx.

All execution stops until a key is pressed, then the value of that key is stored in Vx.

Keyboard layout (16-bit hexa keyboard): 
1	2	3	C
4	5	6	D
7	8	9	E
A	0	B	F
*/

pub struct Keyboard{
    //keys[i] == true means key I is in down position
    keys: [bool; 16],
}

impl Keyboard{
    pub fn press_down(&mut self, i: usize){
        self.keys[i] = true;
    }

    pub fn press_up(&mut self, i: usize){
        self.keys[i] = false;
    }

    pub fn is_key_down(&self, i: usize) -> bool{
        self.keys[i]
    }
}