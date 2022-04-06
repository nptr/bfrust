use std::env;
use std::collections::HashMap;

extern "C" {
    fn getchar() -> i32;
}

fn build_jump_table(instructions: &Vec<char>) -> Result<HashMap<usize, usize>, usize> {
	let mut map: HashMap<usize, usize> = HashMap::new();
	let mut openstack: Vec<usize> = Vec::new();

	for j in 0..instructions.len() {
		if instructions[j] == '[' { openstack.push(j); }
		if instructions[j] == ']' {
			let o = openstack.pop();
			if o.is_none() {
				return Err(j);
			}
			map.insert(o.unwrap(), j);
			map.insert(j, o.unwrap());
		}
	}

	if openstack.len() > 0 {
		return Err(openstack[0]);
	}

	return Ok(map);
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		print!("A simple brainfuck interpreter by Jakob K.\n");
		print!("Cell width:    8 bit, wrapping\n");
		print!("Cell count:    64000\n");
		print!("Usage:         bfr <filepath>\n");
		return;
	}

	let res = std::fs::read_to_string(&args[1]);
	if res.is_err() {
		print!("Error: Failed to read file '{}'", args[1]);
		return;
	}
	let instructions: Vec<char> = res.unwrap().chars().collect();

	let res = build_jump_table(&instructions);
	if res.is_err() {
		print!("Error: No partner for bracket at position {}", res.unwrap_err());
		return;
	}
	let jumptable: HashMap<usize, usize> = res.unwrap();

	let mut memory: [u8; 64000] = [0; 64000];
	let mut dptr: usize = 0;
	let mut iptr: usize = 0;
	let upperbound = memory.len() - 1;
	
	while iptr < instructions.len() {
	
		let opc: char = instructions[iptr];

		if opc == '>' {
			if dptr == upperbound {
				dptr = 0;
			} else {
				dptr += 1;
			}
		} else if opc == '<' {
			if dptr == 0 {
				dptr = upperbound;
			} else {
				dptr -= 1;
			}
		} else if opc == '+' {
			memory[dptr] = memory[dptr].wrapping_add(1);
		} else if opc == '-' {
			memory[dptr] = memory[dptr].wrapping_sub(1);
		} else if opc == '.' {
			print!("{}", std::char::from_u32(memory[dptr] as u32).unwrap());
		} else if opc == ',' {
			unsafe {
				// C to the rescue
				memory[dptr] = getchar() as u8;
			}
		} else if opc == '[' {
			if memory[dptr] == 0 {
				let d = jumptable.get(&iptr).unwrap();
				iptr = d + 1;
				continue;
			}
		} else if opc == ']' {
			if memory[dptr] != 0 {
				let d = jumptable.get(&iptr).unwrap();
				iptr = d + 1;
				continue;
			}
		} else {
			// ignoring unknown op codes
		}

		iptr += 1;
	}
}

