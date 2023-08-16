use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{
    utils::csparse::{is_number, CSParse},
    Instruction,
};

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
    Some(i_jc),
    Some(i_cc),
    Some(i_r),
    Some(i_loop),
    Some(i_ir),
    Some(i_sysc),
    Some(i_sysr),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(i_ldi),
    Some(i_ldm),
    Some(i_stm),
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
        reg.csparse()
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

pub fn i_jc(
    asm: Vec<String>,
    refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    inst_num: usize,
) -> Instruction {
    let w = get_wide(&asm[0]) - 1;
    let c = get_condition_code(&mut asm[0].clone()) << 4;
    if asm[1].starts_with("*") {
        let nn = String::from_utf8(asm[1].as_bytes()[1..].to_vec()).unwrap();
        if !is_number(&nn) {
            match w {
                1 => {
                    refill_table.insert(inst_num, ((1, 2), nn));
                    Instruction {
                        opcode: 0x10,
                        oprands: vec![c | w, 0, 0],
                    }
                }
                2 => {
                    refill_table.insert(inst_num, ((1, 4), nn));
                    Instruction {
                        opcode: 0x10,
                        oprands: vec![c | w, 0, 0, 0, 0],
                    }
                }
                3 => {
                    refill_table.insert(inst_num, ((1, 8), nn));
                    Instruction {
                        opcode: 0x10,
                        oprands: vec![c | w, 0, 0, 0, 0, 0, 0, 0, 0],
                    }
                }
                _ => panic!("Invalid opcode {}.", asm[0]),
            }
        } else {
            let num: u64 = nn.csparse().unwrap();
            match w {
                1 => Instruction {
                    opcode: 0x10,
                    oprands: vec![c | w, num as u8, (num >> 8) as u8],
                },
                2 => Instruction {
                    opcode: 0x10,
                    oprands: vec![
                        c | w,
                        num as u8,
                        (num >> 8) as u8,
                        (num >> 16) as u8,
                        (num >> 24) as u8,
                    ],
                },
                3 => Instruction {
                    opcode: 0x10,
                    oprands: vec![
                        c | w,
                        num as u8,
                        (num >> 8) as u8,
                        (num >> 16) as u8,
                        (num >> 24) as u8,
                        (num >> 32) as u8,
                        (num >> 40) as u8,
                        (num >> 48) as u8,
                        (num >> 56) as u8,
                    ],
                },
                _ => panic!("Invalid opcode {}.", asm[0]),
            }
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
    let w = get_wide(&asm[0]) - 1;
    let c = get_condition_code(&mut asm[0].clone()) << 4;
    if asm[1].starts_with("*") {
        let nn = String::from_utf8(asm[1].as_bytes()[1..].to_vec()).unwrap();
        if !is_number(&nn) {
            match w {
                1 => {
                    refill_table.insert(inst_num, ((1, 2), nn));
                    Instruction {
                        opcode: 0x11,
                        oprands: vec![c | w, 0, 0],
                    }
                }
                2 => {
                    refill_table.insert(inst_num, ((1, 4), nn));
                    Instruction {
                        opcode: 0x11,
                        oprands: vec![c | w, 0, 0, 0, 0],
                    }
                }
                3 => {
                    refill_table.insert(inst_num, ((1, 8), nn));
                    Instruction {
                        opcode: 0x11,
                        oprands: vec![c | w, 0, 0, 0, 0, 0, 0, 0, 0],
                    }
                }
                _ => panic!("Invalid opcode {}.", asm[0]),
            }
        } else {
            let num: u64 = nn.csparse().unwrap();
            match w {
                1 => Instruction {
                    opcode: 0x11,
                    oprands: vec![c | w, num as u8, (num >> 8) as u8],
                },
                2 => Instruction {
                    opcode: 0x11,
                    oprands: vec![
                        c | w,
                        num as u8,
                        (num >> 8) as u8,
                        (num >> 16) as u8,
                        (num >> 24) as u8,
                    ],
                },
                3 => Instruction {
                    opcode: 0x11,
                    oprands: vec![
                        c | w,
                        num as u8,
                        (num >> 8) as u8,
                        (num >> 16) as u8,
                        (num >> 24) as u8,
                        (num >> 32) as u8,
                        (num >> 40) as u8,
                        (num >> 48) as u8,
                        (num >> 56) as u8,
                    ],
                },
                _ => panic!("Invalid opcode {}.", asm[0]),
            }
        }
    } else {
        panic!("Invalid oprand {} for jc.", asm[1]);
    }
}

pub fn i_r(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x12,
        oprands: vec![],
    }
}

pub fn i_loop(
    asm: Vec<String>,
    refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    inst_num: usize,
) -> Instruction {
    if asm[2].starts_with("*") {
        let nn = String::from_utf8(asm[2].as_bytes().split_at(1).1.to_vec()).unwrap();
        if !is_number(&nn) {
            refill_table.insert(inst_num, ((1, 4), nn));
            Instruction {
                opcode: 0x13,
                oprands: vec![get_register_code(&asm[1]), 0, 0, 0, 0],
            }
        } else {
            let num: u64 = nn.csparse().unwrap();
            Instruction {
                opcode: 0x13,
                oprands: vec![
                    get_register_code(&asm[1]),
                    num as u8,
                    (num >> 8) as u8,
                    (num >> 16) as u8,
                    (num >> 24) as u8,
                ],
            }
        }
    } else {
        panic!("Invalid oprand {} for loop.", asm[2]);
    }
}

pub fn i_ir(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    let opr = asm[1].to_string();
    let opr = opr.as_bytes()[1..].to_vec();
    let opr = String::from_utf8(opr).unwrap();
    Instruction {
        opcode: 0x14,
        oprands: vec![opr
            .csparse()
            .expect(&format!("Invalid oprand {} for 'ir'.", asm[1]))],
    }
}

pub fn i_sysc(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x15,
        oprands: vec![],
    }
}

pub fn i_sysr(
    _asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x16,
        oprands: vec![],
    }
}

pub fn i_ldi(
    asm: Vec<String>,
    refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    inst_num: usize,
) -> Instruction {
    let w = get_wide(&asm[0]);
    let r = get_register_code(&asm[2]) << 4;
    if !asm[1].starts_with("$") && !asm[1].starts_with("*") {
        panic!("Invalid oprand {} for ldi.", asm[1]);
    }
    let nn = String::from_utf8(asm[1].as_bytes().split_at(1).1.to_vec()).unwrap();
    if asm[1].starts_with("$") {
        let num: u64 = nn.parse().unwrap();
        match w {
            0 => Instruction {
                opcode: 0x20,
                oprands: vec![r | w, num as u8],
            },

            1 => Instruction {
                opcode: 0x20,
                oprands: vec![r | w, num as u8, (num >> 8) as u8],
            },
            2 => Instruction {
                opcode: 0x20,
                oprands: vec![
                    r | w,
                    num as u8,
                    (num >> 8) as u8,
                    (num >> 16) as u8,
                    (num >> 24) as u8,
                ],
            },
            3 => Instruction {
                opcode: 0x20,
                oprands: vec![
                    r | w,
                    num as u8,
                    (num >> 8) as u8,
                    (num >> 16) as u8,
                    (num >> 24) as u8,
                    (num >> 32) as u8,
                    (num >> 40) as u8,
                    (num >> 48) as u8,
                    (num >> 56) as u8,
                ],
            },
            _ => todo!(),
        }
    } else {
        refill_table.insert(
            inst_num,
            (
                (1, {
                    let mut n = 1usize;
                    for _ in 0..w {
                        n *= 2;
                    }
                    n
                }),
                nn,
            ),
        );
        match w {
            0 => Instruction {
                opcode: 0x20,
                oprands: vec![r | w, 0],
            },

            1 => Instruction {
                opcode: 0x20,
                oprands: vec![r | w, 0, 0],
            },
            2 => Instruction {
                opcode: 0x20,
                oprands: vec![r | w, 0, 0, 0, 0],
            },
            3 => Instruction {
                opcode: 0x20,
                oprands: vec![r | w, 0, 0, 0, 0, 0, 0, 0, 0],
            },
            _ => todo!(),
        }
    }
}

pub fn i_ldm(
    asm: Vec<String>,
    _refill_table: &mut HashMap<usize, ((usize, usize), String)>,
    _inst_num: usize,
) -> Instruction {
    Instruction {
        opcode: 0x21,
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
        opcode: 0x22,
        oprands: vec![
            get_register_code(&asm[2]) << 4 | get_register_code(&asm[1]),
            get_wide(&asm[0]),
        ],
    }
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
