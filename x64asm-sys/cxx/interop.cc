#include "interop.h"

std::istream *interop_istream_from_string(const char *data) {
    return new std::istringstream(data);
}

void interop_istream_delete(std::istream *stream) {
    delete stream;
}

const char* interop_string_to_ptr(const std::string *string) {
    return string->c_str();
}

x64asm::Code *interop_x64asm_code_new() {
    return new x64asm::Code();
}

void interop_x64asm_code_read_att(x64asm::Code *code, const char *data) {
    std::stringstream ss(data);
    ss >> *code;
}

size_t interop_x64asm_code_instruction_count(const x64asm::Code *code) {
    return code->size();
}

const x64asm::Instruction* interop_x64asm_code_instruction_ptr(const x64asm::Code *code) {
    return code->data();
}

x64asm::Opcode interop_x64asm_instruction_opcode(const x64asm::Instruction *instr) {
    return instr->get_opcode();
}

std::string interop_x64asm_opcode_write_att(const x64asm::Opcode opc) {
    return x64asm::opcode_write_att(opc);
}

std::string interop_x64asm_opcode_to_string(const x64asm::Opcode opc) {
    std::stringstream ss;
    ss << opc;
    
    return ss.str();
}