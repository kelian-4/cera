use super::{HighlightFlags, Syntax};

pub fn assembly_syntax() -> Syntax {
    Syntax {
        filetype: "assembly",
        filematch: &[".asm", ".s", ".S", ".nasm"],
        keywords: &[
            "mov", "movl", "movq", "movb", "movw", "movzx", "movsx", "lea", "push", "pop", "pusha",
            "popa", "call", "ret", "retn", "jmp", "je", "jne", "jg", "jge", "jl", "jle", "ja",
            "jae", "jb", "jbe", "jz", "jnz", "js", "jns", "jo", "jno", "jc", "jnc", "loop", "add",
            "sub", "mul", "imul", "div", "idiv", "inc", "dec", "neg", "not", "and", "or", "xor",
            "shl", "shr", "sal", "sar", "rol", "ror", "cmp", "test", "nop", "int", "syscall",
            "sysenter", "leave", "enter", "cld", "std", "rep", "repe", "repne", "movs", "stos",
            "lods", "scas", "cmps", "hlt", "cli", "sti", "in", "out", "xchg", "cmov", "bswap",
            "adc", "sbb", "cdq", "cqo", "cwd", "rax|", "rbx|", "rcx|", "rdx|", "rsi|", "rdi|",
            "rbp|", "rsp|", "rip|", "r8|", "r9|", "r10|", "r11|", "r12|", "r13|", "r14|", "r15|",
            "eax|", "ebx|", "ecx|", "edx|", "esi|", "edi|", "ebp|", "esp|", "eip|", "ax|", "bx|",
            "cx|", "dx|", "al|", "ah|", "bl|", "bh|", "cl|", "ch|", "dl|", "dh|", "si|", "di|",
            "bp|", "sp|",
        ],
        types: &[
            "byte", "word", "dword", "qword", "tbyte", "oword", "xmmword", "ymmword", "ptr",
        ],
        constants: &[],
        singleline_comment_start: ";",
        alt_comment_start: "#",
        multiline_comment_start: "/*",
        multiline_comment_end: "*/",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: ".",
        annotation_prefixes: &[],
        variable_prefixes: "%",
        ident_chars: "",
        operators: "+-*/=<>",
        line_rules: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::FUNCTIONS)
            .union(HighlightFlags::CASE_INSENSITIVE),
    }
}
