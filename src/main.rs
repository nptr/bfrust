use std::env;
use std::time::Instant;

extern "C" {
    fn getchar() -> i32;
}

enum OPC {
	Nop,
	CharPrint,
	CharGet,
	LoopBegin,
	LoopEnd,
	ExtCellChange,
	ExtDataChange
}

struct Instruction {
    ins: OPC,
    op1: i32
}

/**
 * Parse the program and create our internal representation of it.
 * Merge consecutive < or > into a change cell instruction with operand.
 * Merge consecutive + or - into an address change instruction with operand.
 */
fn parse_optimizing(instructions: &Vec<char>) -> Vec<Instruction> {
	
	let mut out: Vec<Instruction> = Vec::new();
	let mut prev_opc: char = ' ';

	let mut current_ins: Instruction = Instruction { ins: OPC::Nop, op1: 0 };

	for j in 0..instructions.len() {
		
		let opc: char = instructions[j];
		if opc == '>' {
			if prev_opc == '>' || prev_opc == '<' {
				current_ins.op1 += 1;
			} else {
				out.push(current_ins);
				current_ins = Instruction {
					ins: OPC::ExtCellChange,
					op1: 1
				};
			}
		} else if opc == '<' {
			if prev_opc == '<' || prev_opc == '>' {
				current_ins.op1 -= 1;
			} else {
				out.push(current_ins);
				current_ins = Instruction {
					ins: OPC::ExtCellChange,
					op1: -1
				};
			}
		} else if opc == '+' {
			if prev_opc == '+' || prev_opc == '-' {
				current_ins.op1 += 1;
			} else {
				out.push(current_ins);
				current_ins = Instruction {
					ins: OPC::ExtDataChange,
					op1: 1
				};
			}
		} else if opc == '-' {
			if prev_opc == '-' || prev_opc == '+' {
				current_ins.op1 -= 1;
			} else {
				out.push(current_ins);
				current_ins = Instruction {
					ins: OPC::ExtDataChange,
					op1: -1
				};
			}
		} else if opc == '.' {
			out.push(current_ins);
			current_ins = Instruction {
				ins: OPC::CharPrint,
				op1: 0
			};
		} else if opc == ',' {
			out.push(current_ins);
			current_ins = Instruction {
				ins: OPC::CharGet,
				op1: 0
			};
		} else if opc == '[' {
			out.push(current_ins);
			current_ins = Instruction {
				ins: OPC::LoopBegin,
				op1: 0
			};
		} else if opc == ']' {
			out.push(current_ins);
			current_ins = Instruction {
				ins: OPC::LoopEnd,
				op1: 0
			};
		} else {
			// ignoring unknown op codes
		}

		prev_opc = opc;
	}

	out.push(current_ins);
	return out;
}

/**
 * Iterate the program and "calculate" the jump destinations for [ and ].
 */
fn build_jump_table(instructions: &mut Vec<Instruction>) -> Result<bool, usize> {
	
	let mut openstack: Vec<usize> = Vec::new();
	for j in 0..instructions.len() {

		match instructions[j].ins {
			OPC::LoopBegin => { openstack.push(j);}
			OPC::LoopEnd => {
				let o = openstack.pop();
				if o.is_none() {
					return Err(j);
				}

				let s = o.unwrap();
				instructions[s].op1 = j as i32; // store end index
				instructions[j].op1 = s as i32; // store begin index
			}
			_ => { }
		}
	}

	if openstack.len() > 0 {
		return Err(openstack[0]);
	}

	return Ok(true);
}

fn main() {
	
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		print!("A slightly optimizing brainfuck interpreter by Jakob K.\n");
		print!("Cell width:    8 bit, wrapping\n");
		print!("Cell count:    64000\n");
		print!("Usage:         bfr <filepath>\n");
		return;
	}

	let res = std::fs::read_to_string(&args[1]);
	if res.is_err() {
		print!("Error: Failed to read file '{}'\n", args[1]);
		return;
	}

	let program: Vec<char> = res.unwrap().chars().collect();
	let mut instructions = parse_optimizing(&program);
	let res = build_jump_table(&mut instructions);
	if res.is_err() {
		print!("Error: No partner for bracket at position {}\n", res.unwrap_err());
		return;
	}

	let mut memory: [u8; 64000] = [0; 64000];
	let mut dptr: usize = 0;
	let mut iptr: usize = 0;
	let upperbound = memory.len() as i32;

	let time = Instant::now();

	while iptr < instructions.len() {
	
		let instruction = &instructions[iptr];
		match instruction.ins {
			OPC::ExtCellChange => {
				let mut tptr = dptr as i32;
				tptr = tptr.wrapping_add(instruction.op1) % upperbound;
				if tptr < 0 { 
					tptr = upperbound + tptr;
				}
				dptr = tptr as usize;
			},
			OPC::ExtDataChange => {
				let remainder = (instruction.op1 % 255) as u8;
				memory[dptr] = memory[dptr].wrapping_add(remainder);
			},
			OPC::CharPrint => {
				print!("{}", std::char::from_u32(memory[dptr] as u32).unwrap());
			},
			OPC::CharGet => {
				unsafe {
					// C to the rescue
					memory[dptr] = getchar() as u8;
				}
			},
			OPC::LoopBegin => {
				if memory[dptr] == 0 {
					iptr = (instruction.op1 + 1) as usize;
					continue;
				}
			},
			OPC::LoopEnd => {
				if memory[dptr] != 0 {
					iptr = (instruction.op1 + 1) as usize;
					continue;
				}
			}
			_ => { }
		}

		iptr += 1;
	}

	println!("\nFinished in {} ms\n", time.elapsed().as_millis());
}

