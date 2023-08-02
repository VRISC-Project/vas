use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::Instruction;

pub const BASE_NMEMONICS: [Option<&str>; 64] = [
    Some("nop"),
    Some("add"),
    Some("sub"),
    Some("inc"),
    Some("dec"),
    Some("shl"),
    Some("shr"),
    Some("rol"),
    Some("ror"),
    Some("cmp"),
    Some("and"),
    Some("or"),
    Some("not"),
    Some("xor"),
    None,
    None,
    Some("jc"),
    Some("cc"),
    Some("r"),
    Some("loop"),
    Some("ir"),
    Some("sysc"),
    Some("sysr"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("ldi"),
    Some("ldm"),
    Some("stm"),
    Some("in"),
    Some("out"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("ei"),
    Some("di"),
    Some("ep"),
    Some("dp"),
    Some("livt"),
    Some("lkpt"),
    Some("lupt"),
    Some("lscp"),
    Some("lipdump"),
    Some("lflagdump"),
    Some("sipdump"),
    Some("sflagdump"),
    Some("cpuid"),
    Some("initext"),
    Some("destext"),
    None,
];

pub const BASE_INSTRUCTIONS: [Option<
    fn(Vec<String>, &mut HashMap<usize, ((usize, usize), String)>, usize) -> Instruction,
>; 64] = [
    Some(i_nop),
    Some(i_add),
    Some(i_sub),
    Some(i_inc),
    Some(i_dec),
    Some(i_shl),
    Some(i_shr),
    Some(i_rol),
    Some(i_ror),
    Some(i_cmp),
    Some(i_and),
    Some(i_or),
    Some(i_not),
    Some(i_xor),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(i_jc),
    Some(i_cc),
    Some(i_r),
    Some(i_loop),
    Some(i_ir),
    Some(i_sysc),
    Some(i_sysr),
    None,
    Some(i_ldi),
    Some(i_ldm),
    Some(i_stm),
    None,
    Some(i_in),
    Some(i_out),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(i_ei),
    Some(i_di),
    Some(i_ep),
    Some(i_dp),
    Some(i_livt),
    Some(i_lkpt),
    Some(i_lupt),
    Some(i_lscp),
    Some(i_lipdump),
    Some(i_lflagdump),
    Some(i_sipdump),
    Some(i_sflagdump),
    Some(i_cpuid),
    Some(i_initext),
    Some(i_destext),
    None,
];

fn get_wide(opstr: &str) -> u8 {
    if opstr.as_bytes()[(opstr.len() - 1) as usize] != b'b'
        && opstr.as_bytes()[(opstr.len() - 1) as usize] != b'w'
        && opstr.as_bytes()[(opstr.len() - 1) as usize] != b'd'
        && opstr.as_bytes()[(opstr.len() - 1) as usize] != b'q'
    {
        return 3;
    }
    match opstr.as_bytes()[..][opstr.as_bytes().len() - 1] {
        b'b' => {
            if opstr == "jb" || opstr == "jnb" || opstr == "cb" || opstr == "cnb" {
                3
            } else {
                0
            }
        }
        b'w' => 1,
        b'd' => 2,
        b'q' => 3,
        _ => u8::MAX,
    }
}

fn get_register_code(regstr: &str) -> u8 {
    if !regstr.starts_with("%x") {
        panic!("Invalid register oprand {}.", regstr);
    } else {
        let reg = String::from_utf8(regstr.as_bytes().split_at(2).1.to_vec()).unwrap();
        reg.parse()
            .expect(&format!("Invalid register oprand {}.", regstr))
    }
}

lazy_static! {
    static ref CONCODE: HashMap<String, u8> = HashMap::from([
        (String::from("z"), 1),
        (String::from("x"), 2),
        (String::from("o"), 3),
        (String::from("e"), 4),
        (String::from("ne"), 5),
        (String::from("h"), 6),
        (String::from("l"), 7),
        (String::from("nh"), 8),
        (String::from("nl"), 9),
        (String::from("b"), 0xa),
        (String::from("s"), 0xb),
        (String::from("nb"), 0xc),
        (String::from("ns"), 0xd),
    ]);
}

fn get_condition_code(opstr: &mut str) -> u8 {
    let mut opstr = Vec::from(unsafe { opstr.as_bytes_mut() });
    opstr.remove(0);
    let w = opstr[(opstr.len() - 1) as usize];
    if (w == b'b' && &opstr != b"jb" && &opstr != b"jnb" && &opstr != b"cb" && &opstr != b"cnb")
        || w == b'w'
        || w == b'd'
        || w == b'q'
    {
        opstr.remove(opstr.len() - 1);
    }
    if opstr.len() == 0 {
        0
    } else {
        if CONCODE.contains_key(&String::from_utf8(opstr.clone()).unwrap()) {
            *CONCODE.get(&String::from_utf8(opstr).unwrap()).unwrap()
        } else {
            panic!("Invalid condition code.");
        }
    }
}

/* 所有指令转码的实现 */

pub fn i_nop(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0,
        oprands: vec![],
    }
}

pub fn i_add(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x1,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            (get_wide(&asm[0]) << 4) | get_register_code(&asm[3]),
        ],
    }
}

pub fn i_sub(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x2,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            (get_wide(&asm[0]) << 4) | get_register_code(&asm[3]),
        ],
    }
}

pub fn i_inc(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x3,
        oprands: vec![(get_wide(&asm[0]) << 4) | get_register_code(&asm[1])],
    }
}

pub fn i_dec(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x4,
        oprands: vec![(get_wide(&asm[0]) << 4) | get_register_code(&asm[1])],
    }
}

pub fn i_shl(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x5,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            get_wide(&asm[0]),
        ],
    }
}

pub fn i_shr(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x6,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            get_wide(&asm[0]),
        ],
    }
}

pub fn i_rol(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x7,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            get_wide(&asm[0]),
        ],
    }
}

pub fn i_ror(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x8,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            get_wide(&asm[0]),
        ],
    }
}

pub fn i_cmp(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x9,
        oprands: vec![(get_register_code(&asm[2]) << 4) | get_register_code(&asm[1])],
    }
}

pub fn i_and(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0xa,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            (get_wide(&asm[0]) << 4) | get_register_code(&asm[3]),
        ],
    }
}

pub fn i_or(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0xb,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            (get_wide(&asm[0]) << 4) | get_register_code(&asm[3]),
        ],
    }
}

pub fn i_not(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0xc,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            get_wide(&asm[0]),
        ],
    }
}

pub fn i_xor(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0xd,
        oprands: vec![
            (get_register_code(&asm[2]) << 4) | get_register_code(&asm[1]),
            (get_wide(&asm[0]) << 4) | get_register_code(&asm[3]),
        ],
    }
}

pub fn i_0e(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_0f(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_10(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_11(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_12(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_13(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_jc(
    asm: Vec<String>,
    refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    inst_num: usize,
) -> Instruction {
    let w = get_wide(&asm[0]);
    let c = get_condition_code(&mut asm[0].clone()) << 4;
    if asm[1].starts_with("*") {
        match w {
            1 => {
                refill_table.insert(
                    inst_num,
                    (
                        (1, 2),
                        String::from_utf8(asm[1].as_bytes().split_at(1).1.to_vec()).unwrap(),
                    ),
                );
                Instruction {
                    opcode: 0x14,
                    oprands: vec![c | w, 0, 0],
                }
            }
            2 => {
                refill_table.insert(
                    inst_num,
                    (
                        (1, 4),
                        String::from_utf8(asm[1].as_bytes().split_at(1).1.to_vec()).unwrap(),
                    ),
                );
                Instruction {
                    opcode: 0x14,
                    oprands: vec![c | w, 0, 0, 0, 0],
                }
            }
            3 => {
                refill_table.insert(
                    inst_num,
                    (
                        (1, 8),
                        String::from_utf8(asm[1].as_bytes().split_at(1).1.to_vec()).unwrap(),
                    ),
                );
                Instruction {
                    opcode: 0x14,
                    oprands: vec![c | w, 0, 0, 0, 0, 0, 0, 0, 0],
                }
            }
            _ => panic!("Invalid opcode {}.", asm[0]),
        }
    } else {
        panic!("Invalid oprand {} for jc.", asm[1]);
    }
}

pub fn i_cc(
    asm: Vec<String>,
    refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    inst_num: usize,
) -> Instruction {
    let w = get_wide(&asm[0]);
    let c = get_condition_code(&mut asm[0].clone()) << 4;
    if asm[1].starts_with("*") {
        match w {
            1 => {
                refill_table.insert(
                    inst_num,
                    (
                        (1, 2),
                        String::from_utf8(asm[1].as_bytes().split_at(1).1.to_vec()).unwrap(),
                    ),
                );
                Instruction {
                    opcode: 0x14,
                    oprands: vec![c | w, 0, 0],
                }
            }
            2 => {
                refill_table.insert(
                    inst_num,
                    (
                        (1, 4),
                        String::from_utf8(asm[1].as_bytes().split_at(1).1.to_vec()).unwrap(),
                    ),
                );
                Instruction {
                    opcode: 0x14,
                    oprands: vec![c | w, 0, 0, 0, 0],
                }
            }
            3 => {
                refill_table.insert(
                    inst_num,
                    (
                        (1, 8),
                        String::from_utf8(asm[1].as_bytes().split_at(1).1.to_vec()).unwrap(),
                    ),
                );
                Instruction {
                    opcode: 0x14,
                    oprands: vec![c | w, 0, 0, 0, 0, 0, 0, 0, 0],
                }
            }
            _ => panic!("Invalid opcode {}.", asm[0]),
        }
    } else {
        panic!("Invalid oprand {} for cc.", asm[1]);
    }
}

pub fn i_r(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x16,
        oprands: vec![],
    }
}

pub fn i_loop(
    asm: Vec<String>,
    refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    inst_num: usize,
) -> Instruction {
    refill_table.insert(
        inst_num,
        (
            (2, 4),
            String::from_utf8(asm[2].as_bytes().split_at(1).1.to_vec()).unwrap(),
        ),
    );
    Instruction {
        opcode: 0x17,
        oprands: vec![get_register_code(&asm[1]), 0, 0, 0, 0],
    }
}

pub fn i_ir(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x18,
        oprands: vec![asm[1]
            .parse()
            .expect(&format!("Invalid oprand {} for 'ir'.", asm[1]))],
    }
}

pub fn i_sysc(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x19,
        oprands: vec![],
    }
}

pub fn i_sysr(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x1a,
        oprands: vec![],
    }
}

pub fn i_1b(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_ldi(
    asm: Vec<String>,
    refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    inst_num: usize,
) -> Instruction {
    let w = get_wide(&asm[0]);
    let r = get_register_code(&asm[2]) << 4;
    if asm[1].starts_with("$") {
        refill_table.insert(
            inst_num,
            (
                (2, 2 ^ w as usize),
                String::from_utf8(asm[1].as_bytes().split_at(1).1.to_vec()).unwrap(),
            ),
        );
    } else {
        panic!("Invalid oprand {}.", asm[1]);
    }
    match w {
        0 => Instruction {
            opcode: 0x1c,
            oprands: vec![r | w, 0],
        },

        1 => Instruction {
            opcode: 0x1c,
            oprands: vec![r | w, 0, 0],
        },
        2 => Instruction {
            opcode: 0x1c,
            oprands: vec![r | w, 0, 0, 0, 0],
        },
        3 => Instruction {
            opcode: 0x1c,
            oprands: vec![r | w, 0, 0, 0, 0, 0, 0, 0, 0],
        },
        _ => todo!(),
    }
}

pub fn i_ldm(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x1d,
        oprands: vec![
            get_register_code(&asm[2]) << 4 | get_register_code(&asm[1]),
            get_wide(&asm[0]),
        ],
    }
}

pub fn i_stm(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x1d,
        oprands: vec![
            get_register_code(&asm[2]) << 4 | get_register_code(&asm[1]),
            get_wide(&asm[0]),
        ],
    }
}

pub fn i_1f(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_in(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!(); //TODO
}

pub fn i_out(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!(); //TODO
}

pub fn i_22(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_23(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_24(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_25(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_26(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_27(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_28(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_29(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_2a(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_2b(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_2c(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_2d(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_2e(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_2f(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}

pub fn i_ei(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x30,
        oprands: vec![],
    }
}

pub fn i_di(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x31,
        oprands: vec![],
    }
}

pub fn i_ep(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x32,
        oprands: vec![],
    }
}

pub fn i_dp(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x33,
        oprands: vec![],
    }
}

pub fn i_livt(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x34,
        oprands: vec![get_register_code(&asm[1])],
    }
}

pub fn i_lkpt(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x35,
        oprands: vec![get_register_code(&asm[1])],
    }
}

pub fn i_lupt(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x36,
        oprands: vec![get_register_code(&asm[1])],
    }
}

pub fn i_lscp(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x37,
        oprands: vec![get_register_code(&asm[1])],
    }
}

pub fn i_lipdump(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x38,
        oprands: vec![get_register_code(&asm[1])],
    }
}

pub fn i_lflagdump(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x39,
        oprands: vec![get_register_code(&asm[1])],
    }
}

pub fn i_sipdump(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x3a,
        oprands: vec![get_register_code(&asm[1])],
    }
}

pub fn i_sflagdump(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x3b,
        oprands: vec![get_register_code(&asm[1])],
    }
}

pub fn i_cpuid(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x3c,
        oprands: vec![],
    }
}

pub fn i_initext(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x3d,
        oprands: vec![get_register_code(&asm[1])],
    }
}

pub fn i_destext(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x3e,
        oprands: vec![],
    }
}

pub fn i_3f(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    todo!();
}
