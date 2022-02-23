use crate::cpu::Cpu;
mod adc;
mod add;
mod call;
mod dec;
mod inc;
mod interrupt;
mod jp;
mod jr;
mod ld;
mod ldh;
mod logic;
mod ret;
mod rotate;
mod sbc;
mod stack;
mod sub;

#[derive(Debug, Clone)]
pub enum Argument {
    Byte(u8),
    Word(u16),
}

impl From<u8> for Argument {
    fn from(byte: u8) -> Self {
        Argument::Byte(byte)
    }
}

impl From<u16> for Argument {
    fn from(word: u16) -> Self {
        Argument::Word(word)
    }
}

pub struct OpCode<'a>(
    /// Mnemonic
    pub &'a str,
    /// Function to call
    pub fn(&mut Cpu),
    /// Instruction size (bytes)
    pub u8,
    /// Machine cycles
    pub u8,
);

#[rustfmt::skip]
pub const OPCODES: [OpCode; 256] = [
    // 0x0_
    OpCode("NOP"          , nop,                  1, 1),
    OpCode("LD BC, d16"   , ld::bc::imm,          3, 3),
    OpCode("LD (BC), A"   , ld::bc_ind::a,        1, 2),
    OpCode("INC BC"       , inc::bc,              1, 2),
    OpCode("INC B"        , inc::b,               1, 1),
    OpCode("DEC B"        , dec::b,               1, 1),
    OpCode("LD B, d8"     , ld::b::imm,           2, 2),
    OpCode("RLCA"         , rotate::rlca,         1, 1),
    OpCode("LD (a16, SP)" , ld::addr::sp,         3, 5),
    OpCode("ADD HL, BC"   , add::hl::bc,          1, 2),
    OpCode("LD A, (BC)"   , ld::a::bc_ind,        1, 2),
    OpCode("DEC BC"       , dec::bc,              1, 2),
    OpCode("INC C"        , inc::c,               1, 1),
    OpCode("DEC C"        , dec::c,               1, 1),
    OpCode("LD C, d8"     , ld::c::imm,           2, 2),
    OpCode("RRCA"         , rotate::rrca,         1, 1),

    // 0x1_
    OpCode("STOP"         , interrupt::stop,      2, 1),
    OpCode("LD DE, d16"   , ld::de::imm,          3, 3),
    OpCode("LD (DE), A"   , ld::de_ind::a,        1, 2),
    OpCode("INC DE"       , inc::de,              1, 2),
    OpCode("INC D"        , inc::d,               1, 1),
    OpCode("DEC D"        , dec::d,               1, 1),
    OpCode("LD D, d8"     , ld::d::imm,           2, 2),
    OpCode("RLA"          , rotate::rla,          1, 1),
    OpCode("JR r8"        , jr::r8,               2, 3),
    OpCode("ADD HL, DE"   , add::hl::de,          1, 2),
    OpCode("LD A, (DE)"   , ld::a::de_ind,        1, 2),
    OpCode("DEC DE"       , dec::de,              1, 2),
    OpCode("INC E"        , inc::e,               1, 1),
    OpCode("DEC E"        , dec::e,               1, 1),
    OpCode("LD E, d8"     , ld::e::imm,           2, 2),
    OpCode("RRA"          , rotate::rra,          1, 1),

    // 0x2_
    OpCode("JR NZ, r8"    , jr::nz,               2, 2), // 3 if branch taken
    OpCode("LD HL, d16"   , ld::hl::imm,          3, 3),
    OpCode("LD (HL+), A"  , ld::hl_ind::add::a,   1, 2),
    OpCode("INC HL"       , inc::hl,              1, 2),
    OpCode("INC H"        , inc::h,               1, 1),
    OpCode("DEC H"        , dec::h,               1, 1),
    OpCode("LD H, d8"     , ld::h::imm,           2, 2),
    OpCode("DAA"          , logic::daa,           1, 1),
    OpCode("JR Z, r8"     , jr::z,                2, 2), // 3 if branch taken
    OpCode("ADD HL, HL"   , add::hl::hl,          1, 2),
    OpCode("LD A, (HL+)"  , ld::a::hl_ind_add,    1, 2),
    OpCode("DEC HL"       , dec::hl,              1, 2),
    OpCode("INC L"        , inc::l,               1, 1),
    OpCode("DEC L"        , dec::l,               1, 1),
    OpCode("LD L, d8"     , ld::l::imm,           2, 2),
    OpCode("CPL"          , logic::cpl,           1, 1),

    // 0x3_
    OpCode("JR NC, r8"    , jr::nc,               2, 2), // 3 if branch taken
    OpCode("LD SP, d16"   , ld::sp::imm,          3, 3),
    OpCode("LD (HL-), A"  , ld::hl_ind::sub::a,   1, 2),
    OpCode("INC SP"       , inc::sp,              1, 2),
    OpCode("INC (HL)"     , inc::hl_ind,          1, 3),
    OpCode("DEC (HL)"     , dec::hl_ind,          1, 3),
    OpCode("LD (HL), d8"  , ld::hl_ind::imm,      2, 3),
    OpCode("SCF"          , logic::scf,           1, 1),
    OpCode("JR C, r8"     , jr::c,                2, 2), // 3 if branch taken
    OpCode("ADD HL, SP"   , add::hl::sp,          1, 2),
    OpCode("LD A, (HL-)"  , ld::a::hl_ind_sub,    1, 2),
    OpCode("DEC SP"       , dec::sp,              1, 2),
    OpCode("INC A"        , inc::a,               1, 1),
    OpCode("DEC A"        , dec::a,               1, 1),
    OpCode("LD A, d8"     , ld::a::imm,           2, 2),
    OpCode("CCF"          , logic::ccf,           1, 1),

    // 0x4_
    OpCode("LD B, B"      , ld::b::b,             1, 1),
    OpCode("LD B, C"      , ld::b::c,             1, 1),
    OpCode("LD B, D"      , ld::b::d,             1, 1),
    OpCode("LD B, E"      , ld::b::e,             1, 1),
    OpCode("LD B, H"      , ld::b::h,             1, 1),
    OpCode("LD B, L"      , ld::b::l,             1, 1),
    OpCode("LD B, (HL)"   , ld::b::hl_ind,        1, 2),
    OpCode("LD B, A"      , ld::b::a,             1, 1),
    OpCode("LD C, B"      , ld::c::b,             1, 1),
    OpCode("LD C, C"      , ld::c::c,             1, 1),
    OpCode("LD C, D"      , ld::c::d,             1, 1),
    OpCode("LD C, E"      , ld::c::e,             1, 1),
    OpCode("LD C, H"      , ld::c::h,             1, 1),
    OpCode("LD C, L"      , ld::c::l,             1, 1),
    OpCode("LD C, (HL)"   , ld::c::hl_ind,        1, 2),
    OpCode("LD C, A"      , ld::c::a,             1, 1),

    // 0x5_
    OpCode("LD D, B"      , ld::d::b,             1, 1),
    OpCode("LD D, C"      , ld::d::c,             1, 1),
    OpCode("LD D, D"      , ld::d::d,             1, 1),
    OpCode("LD D, E"      , ld::d::e,             1, 1),
    OpCode("LD D, H"      , ld::d::h,             1, 1),
    OpCode("LD D, L"      , ld::d::l,             1, 1),
    OpCode("LD D, (HL)"   , ld::d::hl_ind,        1, 2),
    OpCode("LD D, A"      , ld::d::a,             1, 1),
    OpCode("LD E, B"      , ld::e::b,             1, 1),
    OpCode("LD E, C"      , ld::e::c,             1, 1),
    OpCode("LD E, D"      , ld::e::d,             1, 1),
    OpCode("LD E, E"      , ld::e::e,             1, 1),
    OpCode("LD E, H"      , ld::e::h,             1, 1),
    OpCode("LD E, L"      , ld::e::l,             1, 1),
    OpCode("LD E, (HL)"   , ld::e::hl_ind,        1, 2),
    OpCode("LD E, A"      , ld::e::a,             1, 1),

    // 0x6_
    OpCode("LD H, B"      , ld::h::b,             1, 1),
    OpCode("LD H, C"      , ld::h::c,             1, 1),
    OpCode("LD H, D"      , ld::h::d,             1, 1),
    OpCode("LD H, E"      , ld::h::e,             1, 1),
    OpCode("LD H, H"      , ld::h::h,             1, 1),
    OpCode("LD H, L"      , ld::h::l,             1, 1),
    OpCode("LD H, (HL)"   , ld::h::hl_ind,        1, 2),
    OpCode("LD H, A"      , ld::h::a,             1, 1),
    OpCode("LD L, B"      , ld::l::b,             1, 1),
    OpCode("LD L, C"      , ld::l::c,             1, 1),
    OpCode("LD L, D"      , ld::l::d,             1, 1),
    OpCode("LD L, E"      , ld::l::e,             1, 1),
    OpCode("LD L, H"      , ld::l::h,             1, 1),
    OpCode("LD L, L"      , ld::l::l,             1, 1),
    OpCode("LD L, (HL)"   , ld::l::hl_ind,        1, 2),
    OpCode("LD L, A"      , ld::l::a,             1, 1),

    // 0x7_
    OpCode("LD (HL), B"   , ld::hl_ind::b,        1, 2),
    OpCode("LD (HL), C"   , ld::hl_ind::c,        1, 2),
    OpCode("LD (HL), D"   , ld::hl_ind::d,        1, 2),
    OpCode("LD (HL), E"   , ld::hl_ind::e,        1, 2),
    OpCode("LD (HL), H"   , ld::hl_ind::h,        1, 2),
    OpCode("LD (HL), L"   , ld::hl_ind::l,        1, 2),
    OpCode("HALT"         , interrupt::halt,      1, 1),
    OpCode("LD (HL), A"   , ld::hl_ind::a,        1, 2),
    OpCode("LD A, B"      , ld::a::b,             1, 1),
    OpCode("LD A, C"      , ld::a::c,             1, 1),
    OpCode("LD A, D"      , ld::a::d,             1, 1),
    OpCode("LD A, E"      , ld::a::e,             1, 1),
    OpCode("LD A, H"      , ld::a::h,             1, 1),
    OpCode("LD A, L"      , ld::a::l,             1, 1),
    OpCode("LD A, (HL)"   , ld::a::hl_ind,        1, 2),
    OpCode("LD A, A"      , ld::a::a,             1, 1),

    // 0x8_
    OpCode("ADD A, B"     , add::a::b,            1, 1),
    OpCode("ADD A, C"     , add::a::c,            1, 1),
    OpCode("ADD A, D"     , add::a::d,            1, 1),
    OpCode("ADD A, E"     , add::a::e,            1, 1),
    OpCode("ADD A, H"     , add::a::h,            1, 1),
    OpCode("ADD A, L"     , add::a::l,            1, 1),
    OpCode("ADD A, (HL)"  , add::a::hl_ind,       1, 2),
    OpCode("ADD A, A"     , add::a::a,            1, 1),
    OpCode("ADC A, B"     , adc::a::b,            1, 1),
    OpCode("ADC A, C"     , adc::a::c,            1, 1),
    OpCode("ADC A, D"     , adc::a::d,            1, 1),
    OpCode("ADC A, E"     , adc::a::e,            1, 1),
    OpCode("ADC A, H"     , adc::a::h,            1, 1),
    OpCode("ADC A, L"     , adc::a::l,            1, 1),
    OpCode("ADC A, (HL)"  , adc::a::hl_ind,       1, 2),
    OpCode("ADC A, A"     , adc::a::a,            1, 1),

    // 0x9_
    OpCode("SUB A, B"     , sub::b,               1, 1),
    OpCode("SUB A, C"     , sub::c,               1, 1),
    OpCode("SUB A, D"     , sub::d,               1, 1),
    OpCode("SUB A, E"     , sub::e,               1, 1),
    OpCode("SUB A, H"     , sub::h,               1, 1),
    OpCode("SUB A, L"     , sub::l,               1, 1),
    OpCode("SUB A, (HL)"  , sub::hl_ind,          1, 2),
    OpCode("SUB A, A"     , sub::a,               1, 1),
    OpCode("SBC A, B"     , sbc::b,               1, 1),
    OpCode("SBC A, C"     , sbc::c,               1, 1),
    OpCode("SBC A, D"     , sbc::d,               1, 1),
    OpCode("SBC A, E"     , sbc::e,               1, 1),
    OpCode("SBC A, H"     , sbc::h,               1, 1),
    OpCode("SBC A, L"     , sbc::l,               1, 1),
    OpCode("SBC A, (HL)"  , sbc::hl_ind,          1, 2),
    OpCode("SBC A, A"     , sbc::a,               1, 1),

    // 0xA_
    OpCode("AND A, B"     , logic::and::b,        1, 1),
    OpCode("AND A, C"     , logic::and::c,        1, 1),
    OpCode("AND A, D"     , logic::and::d,        1, 1),
    OpCode("AND A, E"     , logic::and::e,        1, 1),
    OpCode("AND A, H"     , logic::and::h,        1, 1),
    OpCode("AND A, L"     , logic::and::l,        1, 1),
    OpCode("AND A, (HL)"  , logic::and::hl_ind,   1, 2),
    OpCode("AND A, A"     , logic::and::a,        1, 1),
    OpCode("XOR A, B"     , logic::xor::b,        1, 1),
    OpCode("XOR A, C"     , logic::xor::c,        1, 1),
    OpCode("XOR A, D"     , logic::xor::d,        1, 1),
    OpCode("XOR A, E"     , logic::xor::e,        1, 1),
    OpCode("XOR A, H"     , logic::xor::h,        1, 1),
    OpCode("XOR A, L"     , logic::xor::l,        1, 1),
    OpCode("XOR A, (HL)"  , logic::xor::hl_ind,   1, 2),
    OpCode("XOR A, A"     , logic::xor::a,        1, 1),

    // 0xB_
    OpCode("OR A, B"      , logic::or::b,         1, 1),
    OpCode("OR A, C"      , logic::or::c,         1, 1),
    OpCode("OR A, D"      , logic::or::d,         1, 1),
    OpCode("OR A, E"      , logic::or::e,         1, 1),
    OpCode("OR A, H"      , logic::or::h,         1, 1),
    OpCode("OR A, L"      , logic::or::l,         1, 1),
    OpCode("OR A, (HL)"   , logic::or::hl_ind,    1, 2),
    OpCode("OR A, A"      , logic::or::a,         1, 1),
    OpCode("CP A, B"      , logic::cp::b,         1, 1),
    OpCode("CP A, C"      , logic::cp::c,         1, 1),
    OpCode("CP A, D"      , logic::cp::d,         1, 1),
    OpCode("CP A, E"      , logic::cp::e,         1, 1),
    OpCode("CP A, H"      , logic::cp::h,         1, 1),
    OpCode("CP A, L"      , logic::cp::l,         1, 1),
    OpCode("CP A, (HL)"   , logic::cp::hl_ind,    1, 2),
    OpCode("CP A, A"      , logic::cp::a,         1, 1),

    // 0xC_
    OpCode("RET NZ"       , ret::nz,              1, 2), // 5 if branch taken
    OpCode("POP BC"       , stack::pop::bc,       1, 3),
    OpCode("JP NZ, a16"   , jp::nz,               3, 3), // 4 if branch taken
    OpCode("JP a16"       , jp::a16,              3, 4),
    OpCode("CALL NZ, a16" , call::nz,             3, 3), // 6 if branch taken
    OpCode("PUSH BC"      , stack::push::bc,      1, 4),
    OpCode("ADD A, d8"    , add::a::imm,          2, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("RET Z"        , ret::z,               1, 2), // 5 if branch taken
    OpCode("RET"          , ret::ret,             1, 4),
    OpCode("JP Z, a16"    , jp::z,                3, 3), // 4 if branch taken
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("CALL Z, a16"  , call::z,              3, 3), // 6 if branch taken
    OpCode("CALL a16"     , call::a16,            3, 6),
    OpCode("ADC A, d8"    , adc::a::imm,          2, 2),
    OpCode("UDF"          , undefined,            1, 1),

    // 0xD_
    OpCode("RET NC"       , ret::nc,              1, 2), // 5 if branch taken
    OpCode("POP DE"       , stack::pop::de,       1, 3),
    OpCode("JP NC, a16"   , jp::nc,               3, 3), // 4 if branch taken
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("CALL NC, a16" , call::nc,             3, 3), // 6 if branch taken
    OpCode("PUSH DE"      , stack::push::de,      1, 4),
    OpCode("SUB A, d8"    , sub::imm,             2, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("RET C"        , ret::c,               1, 2), // 5 if branch taken
    OpCode("RETI"         , interrupt::reti,      1, 1),
    OpCode("JP C, a16"    , jp::c,                3, 3), // 4 if branch taken
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("CALL C, a16"  , call::c,              3, 3), // 6 if branch taken
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("SBC A, d8"    , sbc::imm,             2, 2),
    OpCode("UDF"          , undefined,            1, 1),

    // 0xE_
    OpCode("LDH (a8), A"  , ldh::addr::a,         2, 3),
    OpCode("POP HL"       , stack::pop::hl,       1, 3),
    OpCode("LD (C), A"    , ld::c_ind::a,         1, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("PUSH HL"      , stack::push::hl,      1, 4),
    OpCode("AND A, d8"    , logic::and::imm,      2, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("ADD SP, r8"   , add::sp::r8,          2, 4),
    OpCode("JP (HL)"      , jp::hl_ind,           1, 1),
    OpCode("LD (a16), A"  , ld::addr::a,          3, 4),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("XOR A, d8"    , logic::xor::imm,      2, 2),
    OpCode("UDF"          , undefined,            1, 1),

    // 0xF_
    OpCode("LDH A, (a8)"  , ldh::a::addr,         2, 3),
    OpCode("POP AF"       , stack::pop::af,       1, 3),
    OpCode("LD A, (C)"    , ld::a::c_ind,         1, 2),
    OpCode("DI"           , interrupt::di,        1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("PUSH AF"      , stack::push::af,      1, 4),
    OpCode("OR A, d8"     , logic::or::imm,       2, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("LD HL, SP+r8" , ld::hl::sp_add_reg,   2, 3),
    OpCode("LD SP, HL"    , ld::sp::hl,           1, 2),
    OpCode("LD A, (a16)"  , ld::a::addr,          3, 4),
    OpCode("EI"           , interrupt::ei,        1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("CP A, d8"     , logic::cp::imm,       2, 2),
    OpCode("UDF"          , undefined,            1, 1),
];

fn undefined(_: &mut Cpu) {
    unimplemented!();
}

/// Half carry flag is set if the upper nybble
/// changed due to addition operation
fn half_carry_add(old: u8, new: u8) -> bool {
    old & 0xf0 != new & 0xf0
}

/// Half carry flag is set if the lower nybble
/// of the minuend is less than the lower nybble
/// of the subtrahend
fn half_carry_sub(minuend: u8, subtrahend: u8) -> bool {
    minuend & 0x0f < subtrahend & 0x0f
}

/// Sign extend 8-bit value to 16-bit
fn sign_extend(byte: u8) -> u16 {
    if byte & 0x80 != 0 {
        0xFF00 | byte as u16
    } else {
        byte as u16
    }
}

/// 0x00: No operation
/// - - - -
fn nop(_: &mut Cpu) {}
