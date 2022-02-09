use crate::cpu::Cpu;
mod add;
mod dec;
mod inc;
mod ld;
mod ldh;

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
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("LD (a16, SP)" , ld::addr::sp,         3, 5),
    OpCode("ADD HL, BC"   , add::hl::bc,          1, 2),
    OpCode("LD A, (BC)"   , ld::a::bc_ind,        1, 2),
    OpCode("DEC BC"       , dec::bc,              1, 2),
    OpCode("INC C"        , inc::c,               1, 1),
    OpCode("DEC C"        , dec::c,               1, 1),
    OpCode("LD C, d8"     , ld::c::imm,           2, 2),
    OpCode("UDF"          , undefined,            1, 1),

    // 0x1_
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("LD DE, d16"   , ld::de::imm,          3, 3),
    OpCode("LD (DE), A"   , ld::de_ind::a,        1, 2),
    OpCode("INC DE"       , inc::de,              1, 2),
    OpCode("INC D"        , inc::d,               1, 1),
    OpCode("DEC D"        , dec::d,               1, 1),
    OpCode("LD D, d8"     , ld::d::imm,           2, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("ADD HL, DE"   , add::hl::de,          1, 2),
    OpCode("LD A, (DE)"   , ld::a::de_ind,        1, 2),
    OpCode("DEC DE"       , dec::de,              1, 2),
    OpCode("INC E"        , inc::e,               1, 1),
    OpCode("DEC E"        , dec::e,               1, 1),
    OpCode("LD E, d8"     , ld::e::imm,           2, 2),
    OpCode("UDF"          , undefined,            1, 1),

    // 0x2_
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("LD HL, d16"   , ld::hl::imm,          3, 3),
    OpCode("LD (HL+), A"  , ld::hl_ind::add::a,   1, 2),
    OpCode("INC HL"       , inc::hl,              1, 2),
    OpCode("INC H"        , inc::h,               1, 1),
    OpCode("DEC H"        , dec::h,               1, 1),
    OpCode("LD H, d8"     , ld::h::imm,           2, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("ADD HL, HL"   , add::hl::hl,          1, 2),
    OpCode("LD A, (HL+)"  , ld::a::hl_ind_add,    1, 2),
    OpCode("DEC HL"       , dec::hl,              1, 2),
    OpCode("INC L"        , inc::l,               1, 1),
    OpCode("DEC L"        , dec::l,               1, 1),
    OpCode("LD L, d8"     , ld::l::imm,           2, 2),
    OpCode("UDF"          , undefined,            1, 1),

    // 0x3_
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("LD SP, d16"   , ld::sp::imm,          3, 3),
    OpCode("LD (HL-), A"  , ld::hl_ind::sub::a,   1, 2),
    OpCode("INC SP"       , inc::sp,              1, 2),
    OpCode("INC (HL)"     , inc::hl_ind,          1, 3),
    OpCode("DEC (HL)"     , dec::hl_ind,          1, 3),
    OpCode("LD (HL), d8"  , ld::hl_ind::imm,      2, 3),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("ADD HL, SP"   , add::hl::sp,          1, 2),
    OpCode("LD A, (HL-)"  , ld::a::hl_ind_sub,    1, 2),
    OpCode("DEC SP"       , dec::sp,              1, 2),
    OpCode("INC A"        , inc::a,               1, 1),
    OpCode("DEC A"        , dec::a,               1, 1),
    OpCode("LD A, d8"     , ld::a::imm,           2, 2),
    OpCode("UDF"          , undefined,            1, 1),

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
    OpCode("UDF"          , undefined,            1, 1),
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
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),

    // 0x9_
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),

    // 0xA_
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),

    // 0xB_
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),

    // 0xC_
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("ADD A, d8"    , add::a::imm,          2, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),

    // 0xD_
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),

    // 0xE_
    OpCode("LDH (a8), A"  , ldh::addr::a,         2, 3),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("LD (C), A"    , ld::c_ind::a,         1, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("ADD SP, r8"   , add::sp::r8,          2, 4),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("LD (a16), A"  , ld::addr::a,          3, 4),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),

    // 0xF_
    OpCode("LDH A, (a8)"  , ldh::a::addr,         2, 3),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("LD A, (C)"    , ld::a::c_ind,         1, 2),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("LD HL, SP+r8" , ld::hl::sp_add_reg,   2, 3),
    OpCode("LD SP, HL"    , ld::sp::hl,           1, 2),
    OpCode("LD A, (a16)"  , ld::a::addr,          3, 4),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
    OpCode("UDF"          , undefined,            1, 1),
];

fn undefined(_: &mut Cpu) {
    unimplemented!();
}

/// Half carry flag is set if the upper nybble
/// changed due to an arithmetic operation
fn half_carry(old: u8, new: u8) -> bool {
    old & 0xf0 != new & 0xf0
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
