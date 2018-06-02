use std::vec::Vec;
use std::string::String;

pub const INSTRUCTION_LENGTH: [usize; 256] = [
	1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1,
	2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 1, 1, 2, 1,
	2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
	2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 2, 3, 3, 2, 1,
	1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 1, 2, 1,
	2, 1, 2, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 1,
	2, 1, 2, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 1,
];

pub fn get_assembly(slice: &[u8]) -> Vec<String> {
	let mut asm = Vec::new();
	let mut i = 0;
	while i < slice.len() {
		if let Ok(ins) = get_assembly_for_instruction(&slice[i..]) {
			asm.push(ins);
			i += INSTRUCTION_LENGTH[slice[i] as usize];
		}
		else {
			break;
		}
	}
	asm
}

pub fn get_assembly_for_instruction(slice: &[u8]) -> Result<String,()> {
	if slice.len() == 0 || slice.len() < INSTRUCTION_LENGTH[slice[0] as usize] {
		return Err(());
	}
	else {
		let asm = match slice[0] {
			0x00 => format!("NOP"),
			0x01 => format!("LD BC, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0x02 => format!("LD (BC), A"),
			0x03 => format!("INC BC"),
			0x04 => format!("INC B"),
			0x05 => format!("DEC B"),
			0x06 => format!("LD B, {:X}", slice[1]),
			0x07 => format!("RLCA"),
			0x08 => format!("LD ({:X}), SP", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0x09 => format!("ADD HL, BC"),
			0x0A => format!("LD A, (BC)"),
			0x0B => format!("DEC BC"),
			0x0C => format!("INC C"),
			0x0D => format!("DEC C"),
			0x0E => format!("LD C, {:X}", slice[1]),
			0x0F => format!("RRCA"),
			0x10 => format!("STOP"),
			0x11 => format!("LD DE, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0x12 => format!("LD (DE), A"),
			0x13 => format!("INC DE"),
			0x14 => format!("INC D"),
			0x15 => format!("DEC D"),
			0x16 => format!("LD D, {:X}", slice[1]),
			0x17 => format!("RLA"),
			0x18 => format!("JR {:X}", slice[1] as i8),
			0x19 => format!("ADD HL, DE"),
			0x1A => format!("LD A, (DE)"),
			0x1B => format!("DEC DE"),
			0x1C => format!("INC E"),
			0x1D => format!("DEC E"),
			0x1E => format!("LD E, {:X}", slice[1]),
			0x1F => format!("RRA"),
			0x20 => format!("JR NZ, {:X}", slice[1] as i8),
			0x21 => format!("LD HL, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0x22 => format!("LD (HL+), A"),
			0x23 => format!("INC HL"),
			0x24 => format!("INC H"),
			0x25 => format!("DEC H"),
			0x26 => format!("LD H, {:X}", slice[1]),
			0x27 => format!("DAA"),
			0x28 => format!("JR Z, {:X}", slice[1] as i8),
			0x29 => format!("ADD HL, HL"),
			0x2A => format!("LD A, (HL+)"),
			0x2B => format!("DEC HL"),
			0x2C => format!("INC L"),
			0x2D => format!("DEC L"),
			0x2E => format!("LD L, {:X}", slice[1]),
			0x2F => format!("CPL"),
			0x30 => format!("JR NC, {:X}", slice[1] as i8),
			0x31 => format!("LD SP, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0x32 => format!("LD (HL-), A"),
			0x33 => format!("INC SP"),
			0x34 => format!("INC (HL)"),
			0x35 => format!("DEC (HL)"),
			0x36 => format!("LD (HL), {:X}", slice[1]),
			0x37 => format!("SCF"),
			0x38 => format!("JR C, {:X}", slice[1] as i8),
			0x39 => format!("ADD HL, SP"),
			0x3A => format!("LD A, (HL-)"),
			0x3B => format!("DEC SP"),
			0x3C => format!("INC A"),
			0x3D => format!("DEC A"),
			0x3E => format!("LD A, {:X}", slice[1]),
			0x3F => format!("CCF"),
			0x40 => format!("LD B, B"),
			0x41 => format!("LD B, C"),
			0x42 => format!("LD B, D"),
			0x43 => format!("LD B, E"),
			0x44 => format!("LD B, H"),
			0x45 => format!("LD B, L"),
			0x46 => format!("LD B, (HL)"),
			0x47 => format!("LD B, A"),
			0x48 => format!("LD C, B"),
			0x49 => format!("LD C, C"),
			0x4A => format!("LD C, D"),
			0x4B => format!("LD C, E"),
			0x4C => format!("LD C, H"),
			0x4D => format!("LD C, L"),
			0x4E => format!("LD C, (HL)"),
			0x4F => format!("LD C, A"),
			0x50 => format!("LD D, B"),
			0x51 => format!("LD D, C"),
			0x52 => format!("LD D, D"),
			0x53 => format!("LD D, E"),
			0x54 => format!("LD D, H"),
			0x55 => format!("LD D, L"),
			0x56 => format!("LD D, (HL)"),
			0x57 => format!("LD D, A"),
			0x58 => format!("LD E, B"),
			0x59 => format!("LD E, C"),
			0x5A => format!("LD E, D"),
			0x5B => format!("LD E, E"),
			0x5C => format!("LD E, H"),
			0x5D => format!("LD E, L"),
			0x5E => format!("LD E, (HL)"),
			0x5F => format!("LD E, A"),
			0x60 => format!("LD H, B"),
			0x61 => format!("LD H, C"),
			0x62 => format!("LD H, D"),
			0x63 => format!("LD H, E"),
			0x64 => format!("LD H, H"),
			0x65 => format!("LD H, L"),
			0x66 => format!("LD H, (HL)"),
			0x67 => format!("LD H, A"),
			0x68 => format!("LD L, B"),
			0x69 => format!("LD L, C"),
			0x6A => format!("LD L, D"),
			0x6B => format!("LD L, E"),
			0x6C => format!("LD L, H"),
			0x6D => format!("LD L, L"),
			0x6E => format!("LD L, (HL)"),
			0x6F => format!("LD L, A"),
			0x70 => format!("LD (HL), B"),
			0x71 => format!("LD (HL), C"),
			0x72 => format!("LD (HL), D"),
			0x73 => format!("LD (HL), E"),
			0x74 => format!("LD (HL), H"),
			0x75 => format!("LD (HL), L"),
			0x76 => format!("HALT"),
			0x77 => format!("LD (HL), A"),
			0x78 => format!("LD A, B"),
			0x79 => format!("LD A, C"),
			0x7A => format!("LD A, D"),
			0x7B => format!("LD A, E"),
			0x7C => format!("LD A, H"),
			0x7D => format!("LD A, L"),
			0x7E => format!("LD A, (HL)"),
			0x7F => format!("LD A, A"),
			0x80 => format!("ADD A, B"),
			0x81 => format!("ADD A, C"),
			0x82 => format!("ADD A, D"),
			0x83 => format!("ADD A, E"),
			0x84 => format!("ADD A, H"),
			0x85 => format!("ADD A, L"),
			0x86 => format!("ADD A, (HL)"),
			0x87 => format!("ADD A, A"),
			0x88 => format!("ADC A, B"),
			0x89 => format!("ADC A, C"),
			0x8A => format!("ADC A, D"),
			0x8B => format!("ADC A, E"),
			0x8C => format!("ADC A, H"),
			0x8D => format!("ADC A, L"),
			0x8E => format!("ADC A, (HL)"),
			0x8F => format!("ADC A, A"),
			0x90 => format!("SUB A, B"),
			0x91 => format!("SUB A, C"),
			0x92 => format!("SUB A, D"),
			0x93 => format!("SUB A, E"),
			0x94 => format!("SUB A, H"),
			0x95 => format!("SUB A, L"),
			0x96 => format!("SUB A, (HL)"),
			0x97 => format!("SUB A, A"),
			0x98 => format!("SBC A, B"),
			0x99 => format!("SBC A, C"),
			0x9A => format!("SBC A, D"),
			0x9B => format!("SBC A, E"),
			0x9C => format!("SBC A, H"),
			0x9D => format!("SBC A, L"),
			0x9E => format!("SBC A, (HL)"),
			0x9F => format!("SBC A, A"),
			0xA0 => format!("AND B"),
			0xA1 => format!("AND C"),
			0xA2 => format!("AND D"),
			0xA3 => format!("AND E"),
			0xA4 => format!("AND H"),
			0xA5 => format!("AND L"),
			0xA6 => format!("AND (HL)"),
			0xA7 => format!("AND A"),
			0xA8 => format!("XOR B"),
			0xA9 => format!("XOR C"),
			0xAA => format!("XOR D"),
			0xAB => format!("XOR E"),
			0xAC => format!("XOR H"),
			0xAD => format!("XOR L"),
			0xAE => format!("XOR (HL)"),
			0xAF => format!("XOR A"),
			0xB0 => format!("OR B"),
			0xB1 => format!("OR C"),
			0xB2 => format!("OR D"),
			0xB3 => format!("OR E"),
			0xB4 => format!("OR H"),
			0xB5 => format!("OR L"),
			0xB6 => format!("OR (HL)"),
			0xB7 => format!("OR A"),
			0xB8 => format!("CP B"),
			0xB9 => format!("CP C"),
			0xBA => format!("CP D"),
			0xBB => format!("CP E"),
			0xBC => format!("CP H"),
			0xBD => format!("CP L"),
			0xBE => format!("CP (HL)"),
			0xBF => format!("CP A"),
			0xC0 => format!("RET NZ"),
			0xC1 => format!("POP BC"),
			0xC2 => format!("JP NZ, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0xC3 => format!("JP {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0xC4 => format!("CALL NZ, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0xC5 => format!("PUSH BC"),
			0xC6 => format!("ADD A, {:X}", slice[1]),
			0xC7 => format!("RST 00H"),
			0xC8 => format!("RET Z"),
			0xC9 => format!("RET"),
			0xCA => format!("JP Z, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0xCB => get_assembly_for_extended_instruction(&slice[1..]),
			0xCC => format!("CALL Z, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0xCD => format!("CALL {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0xCE => format!("ADD A, {:X}", slice[1]),
			0xCF => format!("RST 08H"),
			0xD0 => format!("RET NC"),
			0xD1 => format!("POP DE"),
			0xD2 => format!("JP NZ, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			//D3 INVALID
			0xD4 => format!("CALL NC, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0xD5 => format!("PUSH DE"),
			0xD6 => format!("SUB {:X}", slice[1]),
			0xD7 => format!("RST 10H"),
			0xD8 => format!("RET C"),
			0xD9 => format!("RETI"),
			0xDA => format!("JP C, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			//DB INVALID
			0xDC => format!("CALL C, {:X}", (((slice[2] as u16) << 8) | slice[1] as u16)),
			//DD INVALID
			0xDE => format!("SBC A, {:X}", slice[1]),
			0xDF => format!("RST 18H"),
			0xE0 => format!("LD (FF00 + {:X}), A", slice[1]),
			0xE1 => format!("POP HL"),
			0xE2 => format!("LD (C), A"),
			//E3..E4 INVALID
			0xE5 => format!("PUSH HL"),
			0xE6 => format!("AND {}", slice[1]),
			0xE7 => format!("RST 20H"),
			0xE8 => format!("ADD SP, {:X}", slice[1] as i8),
			0xE9 => format!("JP HL"),
			0xEA => format!("LD ({:X}), A", (((slice[2] as u16) << 8) | slice[1] as u16)),
			//EB..ED
			0xEE => format!("XOR {:X}", slice[1]),
			0xEF => format!("RST 28H"),
			0xF0 => format!("LD A, (FF00 + {:X})", slice[1]),
			0xF1 => format!("POP AF"),
			0xF2 => format!("LD A, (C)"),
			0xF3 => format!("DI"),
			//F4 INVALID
			0xF5 => format!("PUSH AF"),
			0xF6 => format!("OR {:X}", slice[1]),
			0xF7 => format!("RST 30H"),
			0xF8 => format!("LD HL, SP+{:X}", slice[1] as i8),
			0xF9 => format!("LD SP, HL"),
			0xFA => format!("LD A, ({:X})", (((slice[2] as u16) << 8) | slice[1] as u16)),
			0xFB => format!("EI"),
			//FC..FD INVALID
			0xFE => format!("CP {:X}", slice[1]),
			0xFF => format!("RST 38H"),
			_ => format!("Invalid opcode {:X}", slice[0]),
		};
		Ok(asm)
	}
}

#[allow(dead_code)]
fn get_assembly_for_extended_instruction(slice: &[u8]) -> String {
	match slice[0] {
		0x00 => format!("RLC B"),
		0x01 => format!("RLC C"),
		0x02 => format!("RLC D"),
		0x03 => format!("RLC E"),
		0x04 => format!("RLC H"),
		0x05 => format!("RLC L"),
		0x06 => format!("RLC (HL)"),
		0x07 => format!("RLC A"),
		0x08 => format!("RRC B"),
		0x09 => format!("RRC C"),
		0x0A => format!("RRC D"),
		0x0B => format!("RRC E"),
		0x0C => format!("RRC H"),
		0x0D => format!("RRC L"),
		0x0E => format!("RRC (HL)"),
		0x0F => format!("RRC A"),
		0x10 => format!("RL B"),
		0x11 => format!("RL C"),
		0x12 => format!("RL D"),
		0x13 => format!("RL E"),
		0x14 => format!("RL H"),
		0x15 => format!("RL L"),
		0x16 => format!("RL (HL)"),
		0x17 => format!("RL A"),
		0x18 => format!("RR B"),
		0x19 => format!("RR C"),
		0x1A => format!("RR D"),
		0x1B => format!("RR E"),
		0x1C => format!("RR H"),
		0x1D => format!("RR L"),
		0x1E => format!("RR (HL)"),
		0x1F => format!("RR A"),
		0x20 => format!("SLA B"),
		0x21 => format!("SLA C"),
		0x22 => format!("SLA D"),
		0x23 => format!("SLA E"),
		0x24 => format!("SLA H"),
		0x25 => format!("SLA L"),
		0x26 => format!("SLA (HL)"),
		0x27 => format!("SLA A"),
		0x28 => format!("SRA B"),
		0x29 => format!("SRA C"),
		0x2A => format!("SRA D"),
		0x2B => format!("SRA E"),
		0x2C => format!("SRA H"),
		0x2D => format!("SRA L"),
		0x2E => format!("SRA (HL)"),
		0x2F => format!("SRA A"),
		0x30 => format!("SWAP B"),
		0x31 => format!("SWAP C"),
		0x32 => format!("SWAP D"),
		0x33 => format!("SWAP E"),
		0x34 => format!("SWAP H"),
		0x35 => format!("SWAP L"),
		0x36 => format!("SWAP (HL)"),
		0x37 => format!("SWAP A"),
		0x38 => format!("SRL B"),
		0x39 => format!("SRL C"),
		0x3A => format!("SRL D"),
		0x3B => format!("SRL E"),
		0x3C => format!("SRL H"),
		0x3D => format!("SRL L"),
		0x3E => format!("SRL (HL)"),
		0x3F => format!("SRL A"),
		0x40 => format!("BIT 0,B"),
		0x41 => format!("BIT 0,C"),
		0x42 => format!("BIT 0,D"),
		0x43 => format!("BIT 0,E"),
		0x44 => format!("BIT 0,H"),
		0x45 => format!("BIT 0,L"),
		0x46 => format!("BIT 0,(HL)"),
		0x47 => format!("BIT 0,A"),
		0x48 => format!("BIT 1,B"),
		0x49 => format!("BIT 1,C"),
		0x4A => format!("BIT 1,D"),
		0x4B => format!("BIT 1,E"),
		0x4C => format!("BIT 1,H"),
		0x4D => format!("BIT 1,L"),
		0x4E => format!("BIT 1,(HL)"),
		0x4F => format!("BIT 1,A"),
		0x50 => format!("BIT 2,B"),
		0x51 => format!("BIT 2,C"),
		0x52 => format!("BIT 2,D"),
		0x53 => format!("BIT 2,E"),
		0x54 => format!("BIT 2,H"),
		0x55 => format!("BIT 2,L"),
		0x56 => format!("BIT 2,(HL)"),
		0x57 => format!("BIT 2,A"),
		0x58 => format!("BIT 3,B"),
		0x59 => format!("BIT 3,C"),
		0x5A => format!("BIT 3,D"),
		0x5B => format!("BIT 3,E"),
		0x5C => format!("BIT 3,H"),
		0x5D => format!("BIT 3,L"),
		0x5E => format!("BIT 3,(HL)"),
		0x5F => format!("BIT 3,A"),
		0x60 => format!("BIT 4,B"),
		0x61 => format!("BIT 4,C"),
		0x62 => format!("BIT 4,D"),
		0x63 => format!("BIT 4,E"),
		0x64 => format!("BIT 4,H"),
		0x65 => format!("BIT 4,L"),
		0x66 => format!("BIT 4,(HL)"),
		0x67 => format!("BIT 4,A"),
		0x68 => format!("BIT 5,B"),
		0x69 => format!("BIT 5,C"),
		0x6A => format!("BIT 5,D"),
		0x6B => format!("BIT 5,E"),
		0x6C => format!("BIT 5,H"),
		0x6D => format!("BIT 5,L"),
		0x6E => format!("BIT 5,(HL)"),
		0x6F => format!("BIT 5,A"),
		0x70 => format!("BIT 6,B"),
		0x71 => format!("BIT 6,C"),
		0x72 => format!("BIT 6,D"),
		0x73 => format!("BIT 6,E"),
		0x74 => format!("BIT 6,H"),
		0x75 => format!("BIT 6,L"),
		0x76 => format!("BIT 6,(HL)"),
		0x77 => format!("BIT 6,A"),
		0x78 => format!("BIT 7,B"),
		0x79 => format!("BIT 7,C"),
		0x7A => format!("BIT 7,D"),
		0x7B => format!("BIT 7,E"),
		0x7C => format!("BIT 7,H"),
		0x7D => format!("BIT 7,L"),
		0x7E => format!("BIT 7,(HL)"),
		0x7F => format!("BIT 7,A"),
		0x80 => format!("RES 0,B"),
		0x81 => format!("RES 0,C"),
		0x82 => format!("RES 0,D"),
		0x83 => format!("RES 0,E"),
		0x84 => format!("RES 0,H"),
		0x85 => format!("RES 0,L"),
		0x86 => format!("RES 0,(HL)"),
		0x87 => format!("RES 0,A"),
		0x88 => format!("RES 1,B"),
		0x89 => format!("RES 1,C"),
		0x8A => format!("RES 1,D"),
		0x8B => format!("RES 1,E"),
		0x8C => format!("RES 1,H"),
		0x8D => format!("RES 1,L"),
		0x8E => format!("RES 1,(HL)"),
		0x8F => format!("RES 1,A"),
		0x90 => format!("RES 2,B"),
		0x91 => format!("RES 2,C"),
		0x92 => format!("RES 2,D"),
		0x93 => format!("RES 2,E"),
		0x94 => format!("RES 2,H"),
		0x95 => format!("RES 2,L"),
		0x96 => format!("RES 2,(HL)"),
		0x97 => format!("RES 2,A"),
		0x98 => format!("RES 3,B"),
		0x99 => format!("RES 3,C"),
		0x9A => format!("RES 3,D"),
		0x9B => format!("RES 3,E"),
		0x9C => format!("RES 3,H"),
		0x9D => format!("RES 3,L"),
		0x9E => format!("RES 3,(HL)"),
		0x9F => format!("RES 3,A"),
		0xA0 => format!("RES 4,B"),
		0xA1 => format!("RES 4,C"),
		0xA2 => format!("RES 4,D"),
		0xA3 => format!("RES 4,E"),
		0xA4 => format!("RES 4,H"),
		0xA5 => format!("RES 4,L"),
		0xA6 => format!("RES 4,(HL)"),
		0xA7 => format!("RES 4,A"),
		0xA8 => format!("RES 5,B"),
		0xA9 => format!("RES 5,C"),
		0xAA => format!("RES 5,D"),
		0xAB => format!("RES 5,E"),
		0xAC => format!("RES 5,H"),
		0xAD => format!("RES 5,L"),
		0xAE => format!("RES 5,(HL)"),
		0xAF => format!("RES 5,A"),
		0xB0 => format!("RES 6,B"),
		0xB1 => format!("RES 6,C"),
		0xB2 => format!("RES 6,D"),
		0xB3 => format!("RES 6,E"),
		0xB4 => format!("RES 6,H"),
		0xB5 => format!("RES 6,L"),
		0xB6 => format!("RES 6,(HL)"),
		0xB7 => format!("RES 6,A"),
		0xB8 => format!("RES 7,B"),
		0xB9 => format!("RES 7,C"),
		0xBA => format!("RES 7,D"),
		0xBB => format!("RES 7,E"),
		0xBC => format!("RES 7,H"),
		0xBD => format!("RES 7,L"),
		0xBE => format!("RES 7,(HL)"),
		0xBF => format!("RES 7,A"),

		0xC0 => format!("SET 0,B"),
		0xC1 => format!("SET 0,C"),
		0xC2 => format!("SET 0,D"),
		0xC3 => format!("SET 0,E"),
		0xC4 => format!("SET 0,H"),
		0xC5 => format!("SET 0,L"),
		0xC6 => format!("SET 0,(HL)"),
		0xC7 => format!("SET 0,A"),
		0xC8 => format!("SET 1,B"),
		0xC9 => format!("SET 1,C"),
		0xCA => format!("SET 1,D"),
		0xCB => format!("SET 1,E"),
		0xCC => format!("SET 1,H"),
		0xCD => format!("SET 1,L"),
		0xCE => format!("SET 1,(HL)"),
		0xCF => format!("SET 1,A"),
		0xD0 => format!("SET 2,B"),
		0xD1 => format!("SET 2,C"),
		0xD2 => format!("SET 2,D"),
		0xD3 => format!("SET 2,E"),
		0xD4 => format!("SET 2,H"),
		0xD5 => format!("SET 2,L"),
		0xD6 => format!("SET 2,(HL)"),
		0xD7 => format!("SET 2,A"),
		0xD8 => format!("SET 3,B"),
		0xD9 => format!("SET 3,C"),
		0xDA => format!("SET 3,D"),
		0xDB => format!("SET 3,E"),
		0xDC => format!("SET 3,H"),
		0xDD => format!("SET 3,L"),
		0xDE => format!("SET 3,(HL)"),
		0xDF => format!("SET 3,A"),
		0xE0 => format!("SET 4,B"),
		0xE1 => format!("SET 4,C"),
		0xE2 => format!("SET 4,D"),
		0xE3 => format!("SET 4,E"),
		0xE4 => format!("SET 4,H"),
		0xE5 => format!("SET 4,L"),
		0xE6 => format!("SET 4,(HL)"),
		0xE7 => format!("SET 4,A"),
		0xE8 => format!("SET 5,B"),
		0xE9 => format!("SET 5,C"),
		0xEA => format!("SET 5,D"),
		0xEB => format!("SET 5,E"),
		0xEC => format!("SET 5,H"),
		0xED => format!("SET 5,L"),
		0xEE => format!("SET 5,(HL)"),
		0xEF => format!("SET 5,A"),
		0xF0 => format!("SET 6,B"),
		0xF1 => format!("SET 6,C"),
		0xF2 => format!("SET 6,D"),
		0xF3 => format!("SET 6,E"),
		0xF4 => format!("SET 6,H"),
		0xF5 => format!("SET 6,L"),
		0xF6 => format!("SET 6,(HL)"),
		0xF7 => format!("SET 6,A"),
		0xF8 => format!("SET 7,B"),
		0xF9 => format!("SET 7,C"),
		0xFA => format!("SET 7,D"),
		0xFB => format!("SET 7,E"),
		0xFC => format!("SET 7,H"),
		0xFD => format!("SET 7,L"),
		0xFE => format!("SET 7,(HL)"),
		0xFF => format!("SET 7,A"),
		_ => format!("This is literally impossible.")
	}
}
