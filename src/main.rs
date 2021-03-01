use std::{fs::File, io::Read};
use rand::Rng;

const START_ADDRESS: usize = 0x200;
const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;
const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS : usize = 0x50;

// 00E0 - CLS
// 00EE - RET
// 0nnn - SYS addr
// 1nnn - JP addr
// 2nnn - CALL addr
// 3xkk - SE Vx, byte
// 4xkk - SNE Vx, byte
// 5xy0 - SE Vx, Vy
// 6xkk - LD Vx, byte
// 7xkk - ADD Vx, byte
// 8xy0 - LD Vx, Vy
// 8xy1 - OR Vx, Vy
// 8xy2 - AND Vx, Vy
// 8xy3 - XOR Vx, Vy
// 8xy4 - ADD Vx, Vy
// 8xy5 - SUB Vx, Vy
// 8xy6 - SHR Vx {, Vy}
// 8xy7 - SUBN Vx, Vy
// 8xyE - SHL Vx {, Vy}
// 9xy0 - SNE Vx, Vy
// Annn - LD I, addr
// Bnnn - JP V0, addr
// Cxkk - RND Vx, byte
// Dxyn - DRW Vx, Vy, nibble
// Ex9E - SKP Vx
// ExA1 - SKNP Vx
// Fx07 - LD Vx, DT
// Fx0A - LD Vx, K
// Fx15 - LD DT, Vx
// Fx18 - LD ST, Vx
// Fx1E - ADD I, Vx
// Fx29 - LD F, Vx
// Fx33 - LD B, Vx
// Fx55 - LD [I], Vx
// Fx65 - LD Vx, [I]

struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 0xFFF],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    video: [u32; (VIDEO_WIDTH * VIDEO_HEIGHT)],
    keyboard: [bool; 16],
    opcode: u16,
    fontset: [u8; FONTSET_SIZE as usize]
}

impl Chip8 {
    fn new() -> Self {
        let mut instance = Chip8 {
            registers: [0; 16],
            memory: [0; 0xFFF],
            index: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            video: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
            keyboard: [false; 16],
            opcode: 0,
            fontset:[
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80,  // F]
            ],
        };
        instance.pc = START_ADDRESS as u16;
        let mut index = 0usize;
        for &font in &instance.fontset {
            instance.memory[FONTSET_START_ADDRESS + index] = font;
            index+=1;
        }
        return instance;
    }
    fn load_rom(&mut self,rom : Vec<u8>) {
        let mut index = 0usize;
        for &byte in &rom  {
            self.memory[START_ADDRESS as usize + index] = byte;
            index+=1;
        }
    }

    fn generate_random_number() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=255)
    }


}

fn main() {
    let mut buffer = Vec::new();
    File::open("./res/Test_01.ch8") // Take care of errors after complete.
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(buffer); // move here so we can discard the buffer

    println!("End line test");
}
