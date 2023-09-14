use eeric::prelude::*;

pub fn parse_r4_format(r4: &str) -> Result<format::R4, String> {
    let tokens: Vec<&str> = r4.split(',').map(str::trim).collect();

    if tokens.len() != 4 {
        return Err(format!(
            "Expected format: 'rd, rs1, rs2, rs3', got {} instead",
            r4
        ));
    }

    let rd = parse_operand(tokens[0])?;
    let rs1 = parse_operand(tokens[1])?;
    let rs2 = parse_operand(tokens[2])?;
    let rs3 = parse_operand(tokens[3])?;

    Ok(format::R4 { rd, rs1, rs2, rs3 })
}

pub fn parse_operand(op_str: &str) -> Result<usize, String> {
    let op = match op_str {
        "f0" | "ft0" => 0,
        "f1" | "ft1" => 1,
        "f2" | "ft2" => 2,
        "f3" | "ft3" => 3,
        "f4" | "ft4" => 4,
        "f5" | "ft5" => 5,
        "f6" | "ft6" => 6,
        "f7" | "ft7" => 7,
        "f8" | "fs0" => 8,
        "f9" | "fs1" => 9,
        "f10" | "fa0" => 10,
        "f11" | "fa1" => 11,
        "f12" | "fa2" => 12,
        "f13" | "fa3" => 13,
        "f14" | "fa4" => 14,
        "f15" | "fa5" => 15,
        "f16" | "fa6" => 16,
        "f17" | "fa7" => 17,
        "f18" | "fs2" => 18,
        "f19" | "fs3" => 19,
        "f20" | "fs4" => 20,
        "f21" | "fs5" => 21,
        "f22" | "fs6" => 22,
        "f23" | "fs7" => 23,
        "f24" | "fs8" => 24,
        "f25" | "fs9" => 25,
        "f26" | "fs10" => 26,
        "f27" | "fs11" => 27,
        "f28" | "ft8" => 28,
        "f29" | "ft9" => 29,
        "f30" | "ft10" => 30,
        "f31" | "ft11" => 31,
        _ => return Err(format!("Incorrect float operand: {}", op_str)),
    };

    Ok(op)
}

pub mod pseudo {
    pub fn parse_op_op_format(op_op: &str) -> Result<(usize, usize), String> {
        let tokens: Vec<&str> = op_op.split(',').map(str::trim).collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'freg1, freg2', got {} instead",
                op_op
            ));
        }

        let reg1 = super::parse_operand(tokens[0])?;
        let reg2 = super::parse_operand(tokens[1])?;

        Ok((reg1, reg2))
    }
}
