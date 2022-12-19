use crate::instructs::*;

// This public structure of the virtual machine will contain the registers and memory of the segments
// of the opcode instructions and will contain the counter for the program when machine is running
pub struct Vm {
	pub registers: Vec<u32>,
	pub memory: Vec<Vec<u32>>,
	pub unmapped_segs: Vec<usize>,
	pub max_mapped_seg: usize,
	pub prog_count: u32,
	pub running: bool
}

// Virtual machine that will start to boot and set memory and increment counter 
impl Vm {
	pub fn boot(&mut self) -> u32 {
		let instruction = self.memory[0][self.prog_count as usize];
		self.prog_count += 1;
		instruction
	}

	pub fn execute(&mut self, word: u32){

		// Decore our file input from bitpack::getu that will retrieve an unsigned value 
		//from `word`, represented by `width` bits beginning at least-significant bit `lsb`.
		// let opcode: u8 = bitpack::bitpack::getu(word.into(), 4, 28).try_into().unwrap();
		// let word_u32: u32 = word.try_into().unwrap();
		let opcode = (word >> 28) & (1 << 4) - 1;

		// Excecute our Opcode conditions with the word of u32 bit
		match opcode {
			0 =>  cond_move(self, word),
			1 =>  seg_load(self, word),
			2 =>  seg_store(self, word),
			3 =>  add(self, word),
			4 =>  mul(self, word),
			5 =>  div(self, word),
			6 =>  nand(self, word),
			7 =>  halt(self),
			8 =>  map_seg(self, word),
			9 =>  unmap_seg(self, word),
			10 => output(self, word),
			11 => input(self, word),
			12 => load_prog(self, word),
			13 => load_val(self, word),
			 _ => panic!("Error")

		};
	}
} 