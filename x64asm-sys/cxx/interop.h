#include <istream>
#include <sstream>
#include <string>
#include "../x64asm/include/x64asm.h"

std::istream *interop_istream_from_string(const char *data);
void interop_istream_delete(std::istream *stream);
const char* interop_string_to_ptr(const std::string *string);

x64asm::Code *interop_x64asm_code_new();
void interop_x64asm_code_read_att(x64asm::Code *code, const char *data);
size_t interop_x64asm_code_instruction_count(const x64asm::Code *code);
const x64asm::Instruction* interop_x64asm_code_instruction_ptr(const x64asm::Code *code);
x64asm::Opcode interop_x64asm_instruction_opcode(const x64asm::Instruction *instr);
std::string interop_x64asm_opcode_write_att(const x64asm::Opcode opc);
std::string interop_x64asm_opcode_to_string(const x64asm::Opcode opc);