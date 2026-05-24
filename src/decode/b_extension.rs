use crate::instruction::b_extension::BOpcode;
use crate::instruction::{InstFormat, Instruction, OpcodeKind};
use crate::decode::DecodingError;

pub fn decode_b_ext(inst: u32) -> Result<Instruction, DecodingError> {
    let opcode = inst & 0x7f;
    let funct3 = (inst >> 12) & 0x7;
    let funct7 = (inst >> 25) & 0x7f;
    let rd = ((inst >> 7) & 0x1f) as usize;
    let rs1 = ((inst >> 15) & 0x1f) as usize;
    let rs2 = ((inst >> 20) & 0x1f) as usize;
    let shamt = ((inst >> 20) & 0x3f) as i32;

    match (opcode, funct3, funct7) {
        (0x13, 0x1, 0x14) => Ok(Instruction {
            opc: OpcodeKind::B(BOpcode::BSETI),
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: None,
            imm: Some(shamt),
            inst_format: InstFormat::BExtShamtFormat,
            is_compressed: false,
        }),
        (0x33, 0x1, 0x14) => Ok(Instruction {
            opc: OpcodeKind::B(BOpcode::BSET),
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: None,
            inst_format: InstFormat::RFormat,
            is_compressed: false,
        }),
        (0x13, 0x1, 0x24) => Ok(Instruction {
            opc: OpcodeKind::B(BOpcode::BCLRI),
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: None,
            imm: Some(shamt),
            inst_format: InstFormat::BExtShamtFormat,
            is_compressed: false,
        }),
        (0x33, 0x1, 0x24) => Ok(Instruction {
            opc: OpcodeKind::B(BOpcode::BCLR),
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: None,
            inst_format: InstFormat::RFormat,
            is_compressed: false,
        }),
        (0x13, 0x5, 0x24) => Ok(Instruction {
            opc: OpcodeKind::B(BOpcode::BEXTI),
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: None,
            imm: Some(shamt),
            inst_format: InstFormat::BExtShamtFormat,
            is_compressed: false,
        }),
        (0x33, 0x5, 0x24) => Ok(Instruction {
            opc: OpcodeKind::B(BOpcode::BEXT),
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: None,
            inst_format: InstFormat::RFormat,
            is_compressed: false,
        }),
        (0x13, 0x1, 0x34) => Ok(Instruction {
            opc: OpcodeKind::B(BOpcode::BINVI),
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: None,
            imm: Some(shamt),
            inst_format: InstFormat::BExtShamtFormat,
            is_compressed: false,
        }),
        (0x33, 0x1, 0x34) => Ok(Instruction {
            opc: OpcodeKind::B(BOpcode::BINV),
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: None,
            inst_format: InstFormat::RFormat,
            is_compressed: false,
        }),
        _ => Err(DecodingError::InvalidOpcode),
    }
}