use std::{ffi::{CString, CStr}, fmt::{Display, Debug}};
use x64asm_sys::{x64asm_Instruction, x64asm_Opcode};

pub struct Code {
    code: *mut x64asm_sys::x64asm_Code,
}

impl Code {
    pub fn parse_from_att_assembly(asm: &str) -> Code {
        let str = CString::new(asm).unwrap();
        unsafe {
            let code = x64asm_sys::interop_x64asm_code_new();
            x64asm_sys::interop_x64asm_code_read_att(code, str.as_ptr());
        
            Code {
                code,
            }
        }
    }

    pub fn instructions(&self) -> Instructions {
        let slice = unsafe {
            let ptr = x64asm_sys::interop_x64asm_code_instruction_ptr(self.code);
            let count = x64asm_sys::interop_x64asm_code_instruction_count(self.code);
            std::slice::from_raw_parts(ptr, count)
        };
        Instructions(slice)
    }
}

pub struct Instructions<'a>(&'a [x64asm_Instruction]);

impl<'a> Instructions<'a> {
    #[must_use]
    pub fn get(&self, index: usize) -> Instruction<'a> {
        Instruction(&self.0[index])
    }

    pub fn iter(&self) -> impl Iterator<Item = Instruction<'a>> {
        self.0.iter().map(Instruction)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct Instruction<'a>(&'a x64asm_Instruction);

impl<'a> Instruction<'a> {
    pub fn opcode(&self) -> Opcode {
        Opcode(unsafe {
            x64asm_sys::interop_x64asm_instruction_opcode(self.0)
        })
    }

    pub fn operands(&self) -> Operands {
        Operands(self.0)
    }
}

#[derive(Copy, Clone)]
pub struct Operands<'a>(&'a x64asm_Instruction);

impl<'a> Operands<'a> {
    pub fn len(&self) -> usize {
        todo!()
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Opcode(x64asm_Opcode);

impl Opcode {
    pub fn to_att(&self) -> String {
        unsafe {
            let string = x64asm_sys::interop_x64asm_opcode_write_att(self.0);
            let c_str = x64asm_sys::interop_string_to_ptr(&string);

            CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    } 
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let string = x64asm_sys::interop_x64asm_opcode_to_string(self.0);
            let c_str = x64asm_sys::interop_string_to_ptr(&string);

            write!(f, "{}", CStr::from_ptr(c_str).to_str().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Code;

    #[test]
    pub fn test_opcode_str() {
        let code = Code::parse_from_att_assembly("addq %rax, %rbx");
        let instr = code.instructions().get(0);

        println!("Instruction: {} / {}", instr.opcode(), instr.opcode().to_att());

        assert_eq!(instr.opcode().to_string(), "addq_r64_r64");
    }
}