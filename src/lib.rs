use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    mem::size_of,
};

use config::Config;
use elf_utilities::{header::Ehdr64, Elf64Half};
use vrisc::base::{BASE_INSTRUCTIONS, BASE_NMEMONICS};

pub mod config;
pub mod vrisc;

pub enum ExeFormat {
    Raw,
    Elf,
    Sel,
}

/// 伪指令与正常指令共享此结构
pub struct Instruction {
    // 255代表这是伪指令，实际内容为oprands的内容
    pub opcode: u8,
    pub oprands: Vec<u8>,
}

#[derive(Eq, Hash, PartialEq)]
struct Section {
    name: String,
    starts: u64,
    align: u64,
}

struct Assembler {
    instruction_sequence: Vec<Instruction>,
    sections_table: HashMap<usize, Section>,
    symbols_table: HashMap<usize, String>,
    /// 第一个是在instruction_sequence中的位置
    /// 第二个是在此指令中需要替换的地址在oprands中的位置及长度
    /// 第三个是要替换的地址
    refill_symbols: HashMap<usize, ((usize, usize), String)>,
    source_file: File,
    instruction_space: [Option<
        fn(Vec<String>, &mut HashMap<usize, ((usize, usize), String)>, usize) -> Instruction,
    >; 256],
}

pub fn run(config: Config) -> io::Result<()> {
    let file = File::open(config.input)?;
    let mut ass = Assembler::new(file);
    ass.init();
    ass.do_assembly()?;
    let obj = ass.generate_object(match config.format.as_str() {
        "elf64" => ExeFormat::Elf,
        "sel" => ExeFormat::Sel,
        "raw" => ExeFormat::Raw,
        _ => panic!("Unknown format."),
    });
    let mut output = OpenOptions::new()
        .create(true)
        .append(false)
        .write(true)
        .open(config.output)?;
    output.write(&obj)?;
    Ok(())
}

impl Assembler {
    fn new(file: File) -> Self {
        Assembler {
            sections_table: HashMap::new(),
            symbols_table: HashMap::new(),
            instruction_sequence: Vec::new(),
            refill_symbols: HashMap::new(),
            source_file: file,
            instruction_space: [None; 256],
        }
    }

    fn init(&mut self) {
        self.instruction_space[..64].copy_from_slice(&BASE_INSTRUCTIONS);
    }

    /// 地址和标号在此过程不替换
    fn do_assembly(&mut self) -> io::Result<()> {
        let source_file = BufReader::new(self.source_file.try_clone().unwrap());
        for line in source_file.lines() {
            let line = line?;
            let line = line.trim();
            let line: Vec<&str> = line.split("//").collect(); //移除注释
            let line = line[0];
            let mut line: Vec<&str> = line.split(&[' ', ',']).collect();
            line.retain(|s| s.len() != 0);
            if line.len() == 0 {
                continue;
            }
            let opcode = {
                let mut i = 0u8;
                while (i as usize) < BASE_NMEMONICS.len() {
                    if let Some(instname) = BASE_NMEMONICS[i as usize] {
                        if line[0].starts_with(instname) {
                            break;
                        }
                    }
                    i += 1;
                }
                // 条件跳转指令需特殊处理
                if line[0].starts_with("j") {
                    i = 0x14;
                }
                if line[0].starts_with("c") && line[0] != "cpuid" {
                    i = 0x15;
                }

                if i as usize == BASE_NMEMONICS.len() {
                    None
                } else {
                    Some(i)
                }
            };
            let mut line = line.iter().map(|s| s.to_string()).collect();
            if let Some(opcode) = opcode {
                // 指令
                if let Some(inst) = self.instruction_space[opcode as usize] {
                    self.instruction_sequence.push(inst(
                        line,
                        &mut self.refill_symbols,
                        self.instruction_sequence.len(),
                    ));
                }
            } else {
                // 非指令
                if line[0].as_bytes()[0] == b'#' {
                    // 标号
                    if line[0] == "#n" || line[0] == "#p" {
                        panic!("Invalid symbol name {}.", line[0]);
                    }
                    self.symbols_table.insert(
                        self.instruction_sequence.len(),
                        line[0].split_at(1).1.to_string(),
                    );
                }
                // 伪指令
                else if line[0] == "db" {
                    line.remove(0);
                    for opr in line {
                        let opr = &opr.as_bytes()[1..];
                        let opr = String::from_utf8(opr.to_vec()).unwrap();
                        let i = if opr.starts_with("$") {
                            opr.parse::<u8>().unwrap()
                        } else {
                            self.refill_symbols
                                .insert(self.instruction_sequence.len(), ((0, 1), opr));
                            0
                        };
                        self.instruction_sequence.push(Instruction {
                            opcode: u8::MAX,
                            oprands: vec![i],
                        });
                    }
                } else if line[0] == "dw" {
                    line.remove(0);
                    for opr in line {
                        let opr = &opr.as_bytes()[1..];
                        let opr = String::from_utf8(opr.to_vec()).unwrap();
                        let i = if opr.starts_with("$") {
                            opr.parse::<u16>().unwrap()
                        } else {
                            self.refill_symbols
                                .insert(self.instruction_sequence.len(), ((0, 2), opr));
                            0
                        };
                        self.instruction_sequence.push(Instruction {
                            opcode: u8::MAX,
                            oprands: vec![i as u8, (i >> 8) as u8],
                        });
                    }
                } else if line[0] == "dd" {
                    line.remove(0);
                    for opr in line {
                        let opr = &opr.as_bytes()[1..];
                        let opr = String::from_utf8(opr.to_vec()).unwrap();
                        let i = if opr.starts_with("$") {
                            opr.parse::<u32>().unwrap()
                        } else {
                            self.refill_symbols
                                .insert(self.instruction_sequence.len(), ((0, 2), opr));
                            0
                        };
                        self.instruction_sequence.push(Instruction {
                            opcode: u8::MAX,
                            oprands: vec![
                                i as u8,
                                (i >> 8) as u8,
                                (i >> 16) as u8,
                                (i >> 24) as u8,
                            ],
                        });
                    }
                } else if line[0] == "dq" {
                    line.remove(0);
                    for opr in line {
                        let opr = &opr.as_bytes()[1..];
                        let opr = String::from_utf8(opr.to_vec()).unwrap();
                        let i = if opr.starts_with("$") {
                            opr.parse::<u64>().unwrap()
                        } else {
                            self.refill_symbols
                                .insert(self.instruction_sequence.len(), ((0, 2), opr));
                            0
                        };
                        self.instruction_sequence.push(Instruction {
                            opcode: u8::MAX,
                            oprands: vec![
                                i as u8,
                                (i >> 8) as u8,
                                (i >> 16) as u8,
                                (i >> 24) as u8,
                                (i >> 32) as u8,
                                (i >> 40) as u8,
                                (i >> 48) as u8,
                                (i >> 56) as u8,
                            ],
                        });
                    }
                } else if line[0] == "section" {
                    line.remove(0);
                    // 段声明
                    let name = line.clone();
                    let name = &name[0];
                    let mut starts = 0u64;
                    let mut align = 1u64;
                    line.remove(0);
                    for attr in line.clone() {
                        let attr: Vec<&str> = attr.split("=").collect();
                        let [attr, val, ..] = attr.as_slice() else {
                            panic!("Unknown attibute {}={}", attr[0], attr[1]);
                        };
                        match *attr {
                            "starts" => {
                                starts = val.parse().unwrap();
                            }
                            "align" => {
                                align = val.parse().unwrap();
                            }
                            _ => (),
                        }
                    }
                    self.sections_table.insert(
                        self.instruction_sequence.len(),
                        Section {
                            name: name.to_string(),
                            starts,
                            align,
                        },
                    );
                } else {
                    panic!("Unknown symbol {}.", line[0]);
                }
            }
        }
        Ok(())
    }

    fn generate_object(&self, format: ExeFormat) -> Vec<u8> {
        match format {
            ExeFormat::Elf => self.generate_object_elf(),
            ExeFormat::Sel => self.generate_object_sel(),
            ExeFormat::Raw => self.generate_object_raw(),
        }
    }

    fn generate_object_raw(&self) -> Vec<u8> {
        let mut i = 0u64;
        let mut addr = 0u64;
        let mut address_table: HashMap<String, u64> = HashMap::new();
        let mut section = &Section {
            name: "null".to_string(),
            starts: 0,
            align: 8,
        };
        // 计算段及标号的地址
        for inst in self.instruction_sequence.as_slice() {
            if let Some(sec) = self.sections_table.get(&(i as usize)) {
                section = sec.clone();
                if section.starts != 0 && addr <= section.starts {
                    addr = section.starts;
                }
                if addr % section.align != 0 {
                    addr += section.align - addr % section.align;
                }
                address_table.insert(section.name.clone(), addr);
            }
            if let Some(sym) = self.symbols_table.get(&(i as usize)) {
                if addr % section.align != 0 {
                    addr += section.align - addr % section.align;
                }
                let symname = if section.name != "null" {
                    section.name.clone() + "." + sym
                } else {
                    sym.clone()
                };
                address_table.insert(symname, addr);
            }
            addr += if inst.opcode != u8::MAX {
                inst.oprands.len() as u64 + 1
            } else {
                inst.oprands.len() as u64
            };
            i += 1;
        }
        // 生成目标代码
        i = 0;
        addr = 0;
        let binding = Section {
            name: "null".to_string(),
            starts: 0,
            align: 8,
        };
        section = &binding;
        let mut result = Vec::new();
        for inst in self.instruction_sequence.as_slice() {
            if let Some(sec) = self.sections_table.get(&(i as usize)) {
                section = sec.clone();
                if section.starts != 0 && addr <= section.starts {
                    for _ in 0..(section.starts - addr) {
                        result.push(0);
                    }
                    addr = section.starts;
                }
                if addr % section.align != 0 {
                    for _ in 0..(section.align - addr % section.align) {
                        result.push(0);
                    }
                    addr += section.align - addr % section.align;
                }
            }
            if let Some(_) = self.symbols_table.get(&(i as usize)) {
                if addr % section.align != 0 {
                    for _ in 0..(section.align - addr % section.align) {
                        result.push(0);
                    }
                    addr += section.align - addr % section.align;
                }
            }
            let mut inst = Instruction {
                opcode: inst.opcode,
                oprands: inst.oprands.clone(),
            };
            // 替换标号
            if let Some(((starts, length), name)) = self.refill_symbols.get(&(i as usize)) {
                let addr = {
                    if name == "n" {
                        let mut x = i;
                        let mut list: Vec<(&String, &u64)> = address_table.iter().collect();
                        list.sort_by_key(|&(_, num)| num);
                        while list[x as usize].0 != &(section.name.clone() + ".") {
                            x += 1;
                            if x as usize >= list.len() {
                                panic!("There is no location symbol.");
                            }
                        }
                        *list[x as usize].1
                    } else if name == "p" {
                        let mut x = i;
                        let mut list: Vec<(&String, &u64)> = address_table.iter().collect();
                        list.sort_by_key(|&(_, num)| num);
                        while list[x as usize].0 != &(section.name.clone() + ".") {
                            x -= 1;
                            if x == 0 {
                                panic!("There is no location symbol.");
                            }
                        }
                        *list[x as usize].1
                    } else {
                        if let Some(addr) = address_table.get(name) {
                            *addr
                        } else {
                            panic!("Unknown symbol \"{}\"", name);
                        }
                    }
                };
                // bytes sequence
                let bseq = [
                    addr as u8,
                    (addr >> 8) as u8,
                    (addr >> 16) as u8,
                    (addr >> 24) as u8,
                    (addr >> 32) as u8,
                    (addr >> 40) as u8,
                    (addr >> 48) as u8,
                    (addr >> 56) as u8,
                ];
                for i in 0..*length {
                    inst.oprands[*starts + i] = bseq[i];
                }
            }
            if inst.opcode != u8::MAX {
                result.push(inst.opcode);
            }
            for i in inst.oprands.clone() {
                result.push(i);
            }
            addr += if inst.opcode != u8::MAX {
                inst.oprands.len() as u64 + 1
            } else {
                inst.oprands.len() as u64
            };
            i += 1;
        }
        result
    }

    fn generate_object_sel(&self) -> Vec<u8> {
        todo!();
    }

    fn generate_object_elf(&self) -> Vec<u8> {
        let _ehdr = Ehdr64 {
            e_ident: [
                b'\x7f', b'E', b'L', b'F', 2, 1, 1, 120, //OSABI的自定义值ELFOSABI_META
                1, 0, 0, 0, 0, 0, 0, 0,
            ],
            e_type: 2,
            e_machine: 10086, //自定义值VRISC
            e_version: 1,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: size_of::<Ehdr64>() as Elf64Half,
            e_phentsize: 0x38,
            e_phnum: 0,
            e_shentsize: 0x40,
            e_shnum: self.sections_table.len() as Elf64Half,
            e_shstrndx: 0,
        };
        todo!();
    }
}
