use bitcoin::Opcode;
use bitcoin::Opcode::{
    OP_0, OP_1, OP_2, OP_3, OP_4, OP_5, OP_6, OP_7, OP_8, OP_9, OP_10, OP_11, OP_12, OP_13,
    OP_14, OP_15, OP_16, OP_NOP, OP_IF,
};
pub fn opcode_to_str(opcode: Opcode) -> &'static str {
    match opcode {
        Opcode::OP_0 => "OP_0",
        Opcode::OP_1 => "OP_1",
        Opcode::OP_2 => "OP_2",
        Opcode::OP_3 => "OP_3",
        Opcode::OP_4 => "OP_4",
        Opcode::OP_5 => "OP_5",
        Opcode::OP_6 => "OP_6",
        Opcode::OP_7 => "OP_7",
        Opcode::OP_8 => "OP_8",
        Opcode::OP_9 => "OP_9",
        Opcode::OP_10 => "OP_10",
        Opcode::OP_11 => "OP_11",
        Opcode::OP_12 => "OP_12",
        Opcode::OP_13 => "OP_13",
        Opcode::OP_14 => "OP_14",
        Opcode::OP_15 => "OP_15",
        Opcode::OP_16 => "OP_16",
        Opcode::OP_NOP => "OP_NOP",
        Opcode::OP_IF => "OP_IF",
    }
}

pub fn opcode_to_str_with_arg(opcode: Opcode, arg: u8) -> String {
    match opcode {
        Opcode::OP_0 => "OP_0",
        Opcode::OP_1 => "OP_1",
        Opcode::OP_2 => "OP_2",
        Opcode::OP_3 => "OP_3",
        Opcode::OP_4 => "OP_4",
        Opcode::OP_5 => "OP_5",
        Opcode::OP_6 => "OP_6",
        Opcode::OP_7 => "OP_7",
        Opcode::OP_8 => "OP_8",
        Opcode::OP_9 => "OP_9",
        Opcode::OP_10 => "OP_10",
        Opcode::OP_11 => "OP_11",
        Opcode::OP_12 => "OP_12",
        Opcode::OP_13 => "OP_13",
        Opcode::OP_14 => "OP_14",
        Opcode::OP_15 => "OP_15",
        Opcode::OP_16 => "OP_16",
        Opcode::OP_NOP => "OP_NOP",
        Opcode::OP_IF => format!("OP_IF {}", arg),
        Opcode::OP_NOTIF => format!("OP_NOTIF {}", arg),
        Opcode::OP_ELSE => "OP_ELSE",
    }
    .to_string()
}