use std::collections::VecDeque;

use num_bigint::BigInt;

use crate::script::{
    helper::encode_num,
    op_codes_helper::{
        op_checksig, op_dup, op_fromaltstack, op_hash160, op_hash256, op_toaltstack,
    },
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum OP_CODES {
    UNSET = -1,
    OP_0 = 0,
    OP_PUSHDATA1 = 76,
    OP_PUSHDATA2 = 77,
    OP_PUSHDATA4 = 78,
    OP_1NEGATE = 79,
    OP_1 = 81,
    OP_2 = 82,
    OP_3 = 83,
    OP_4 = 84,
    OP_5 = 85,
    OP_6 = 86,
    OP_7 = 87,
    OP_8 = 88,
    OP_9 = 89,
    OP_10 = 90,
    OP_11 = 91,
    OP_12 = 92,
    OP_13 = 93,
    OP_14 = 94,
    OP_15 = 95,
    OP_16 = 96,
    OP_NOP = 97,
    OP_IF = 99,
    OP_NOTIF = 100,
    OP_ELSE = 103,
    OP_ENDIF = 104,
    OP_VERIFY = 105,
    OP_RETURN = 106,
    OP_TOALTSTACK = 107,
    OP_FROMALTSTACK = 108,
    OP_2DROP = 109,
    OP_2DUP = 110,
    OP_3DUP = 111,
    OP_2OVER = 112,
    OP_2ROT = 113,
    OP_2SWAP = 114,
    OP_IFDUP = 115,
    OP_DEPTH = 116,
    OP_DROP = 117,
    OP_DUP = 118,
    OP_NIP = 119,
    OP_OVER = 120,
    OP_PICK = 121,
    OP_ROLL = 122,
    OP_ROT = 123,
    OP_SWAP = 124,
    OP_TUCK = 125,
    OP_SIZE = 130,
    OP_EQUAL = 135,
    OP_EQUALVERIFY = 136,
    OP_1ADD = 139,
    OP_1SUB = 140,
    OP_NEGATE = 143,
    OP_ABS = 144,
    OP_NOT = 145,
    OP_0NOTEQUAL = 146,
    OP_ADD = 147,
    OP_SUB = 148,
    OP_BOOLAND = 154,
    OP_BOOLOR = 155,
    OP_NUMEQUAL = 156,
    OP_NUMEQUALVERIFY = 157,
    OP_NUMNOTEQUAL = 158,
    OP_LESSTHAN = 159,
    OP_GREATERTHAN = 160,
    OP_LESSTHANOREQUAL = 161,
    OP_GREATERTHANOREQUAL = 162,
    OP_MIN = 163,
    OP_MAX = 164,
    OP_WITHIN = 165,
    OP_RIPEMD160 = 166,
    OP_SHA1 = 167,
    OP_SHA256 = 168,
    OP_HASH160 = 169,
    OP_HASH256 = 170,
    OP_CODESEPARATOR = 171,
    OP_CHECKSIG = 172,
    OP_CHECKSIGVERIFY = 173,
    OP_CHECKMULTISIG = 174,
    OP_CHECKMULTISIGVERIFY = 175,
    OP_NOP1 = 176,
    OP_CHECKLOCKTIMEVERIFY = 177,
    OP_CHECKSEQUENCEVERIFY = 178,
    OP_NOP4 = 179,
    OP_NOP5 = 180,
    OP_NOP6 = 181,
    OP_NOP7 = 182,
    OP_NOP8 = 183,
    OP_NOP9 = 184,
    OP_NOP10 = 185,
}

impl OP_CODES {
    pub fn stack_operations(
        op_code: OP_CODES,
        stack: &mut VecDeque<Vec<u8>>,
        altstack: &mut VecDeque<Vec<u8>>,
        z: Option<BigInt>,
    ) -> bool {
        // 1.to_le_bytes();
        println!("op_code: {:?}", op_code.clone());
        match op_code {
            OP_CODES::UNSET => false,
            OP_CODES::OP_0 => {
                stack.push_back(encode_num(0));
                true
            }
            OP_CODES::OP_PUSHDATA1 => todo!(),
            OP_CODES::OP_PUSHDATA2 => todo!(),
            OP_CODES::OP_PUSHDATA4 => todo!(),
            OP_CODES::OP_1NEGATE => {
                stack.push_back(encode_num(-1));
                true
            }
            OP_CODES::OP_1 => {
                stack.push_back(encode_num(1));
                true
            }
            OP_CODES::OP_2 => {
                stack.push_back(encode_num(2));
                true
            }
            OP_CODES::OP_3 => {
                stack.push_back(encode_num(3));
                true
            }
            OP_CODES::OP_4 => {
                stack.push_back(encode_num(4));
                true
            }
            OP_CODES::OP_5 => {
                stack.push_back(encode_num(5));
                true
            }
            OP_CODES::OP_6 => {
                stack.push_back(encode_num(6));
                true
            }
            OP_CODES::OP_7 => {
                stack.push_back(encode_num(7));
                true
            }
            OP_CODES::OP_8 => {
                stack.push_back(encode_num(8));
                true
            }
            OP_CODES::OP_9 => {
                stack.push_back(encode_num(9));
                true
            }
            OP_CODES::OP_10 => {
                stack.push_back(encode_num(10));
                true
            }
            OP_CODES::OP_11 => {
                stack.push_back(encode_num(11));
                true
            }
            OP_CODES::OP_12 => {
                stack.push_back(encode_num(12));
                true
            }
            OP_CODES::OP_13 => {
                stack.push_back(encode_num(13));
                true
            }
            OP_CODES::OP_14 => {
                stack.push_back(encode_num(14));
                true
            }
            OP_CODES::OP_15 => {
                stack.push_back(encode_num(15));
                true
            }
            OP_CODES::OP_16 => {
                stack.push_back(encode_num(16));
                true
            }
            OP_CODES::OP_NOP => todo!(),
            OP_CODES::OP_IF => todo!(),
            OP_CODES::OP_NOTIF => todo!(),
            OP_CODES::OP_ELSE => todo!(),
            OP_CODES::OP_ENDIF => todo!(),
            OP_CODES::OP_VERIFY => todo!(),
            OP_CODES::OP_RETURN => todo!(),
            OP_CODES::OP_TOALTSTACK => op_toaltstack(stack, altstack),
            OP_CODES::OP_FROMALTSTACK => op_fromaltstack(stack, altstack),
            OP_CODES::OP_2DROP => todo!(),
            OP_CODES::OP_2DUP => todo!(),
            OP_CODES::OP_3DUP => todo!(),
            OP_CODES::OP_2OVER => todo!(),
            OP_CODES::OP_2ROT => todo!(),
            OP_CODES::OP_2SWAP => todo!(),
            OP_CODES::OP_IFDUP => todo!(),
            OP_CODES::OP_DEPTH => todo!(),
            OP_CODES::OP_DROP => todo!(),
            OP_CODES::OP_DUP => op_dup(stack),
            OP_CODES::OP_NIP => todo!(),
            OP_CODES::OP_OVER => todo!(),
            OP_CODES::OP_PICK => todo!(),
            OP_CODES::OP_ROLL => todo!(),
            OP_CODES::OP_ROT => todo!(),
            OP_CODES::OP_SWAP => todo!(),
            OP_CODES::OP_TUCK => todo!(),
            OP_CODES::OP_SIZE => todo!(),
            OP_CODES::OP_EQUAL => todo!(),
            OP_CODES::OP_EQUALVERIFY => todo!(),
            OP_CODES::OP_1ADD => todo!(),
            OP_CODES::OP_1SUB => todo!(),
            OP_CODES::OP_NEGATE => todo!(),
            OP_CODES::OP_ABS => todo!(),
            OP_CODES::OP_NOT => todo!(),
            OP_CODES::OP_0NOTEQUAL => todo!(),
            OP_CODES::OP_ADD => todo!(),
            OP_CODES::OP_SUB => todo!(),
            OP_CODES::OP_BOOLAND => todo!(),
            OP_CODES::OP_BOOLOR => todo!(),
            OP_CODES::OP_NUMEQUAL => todo!(),
            OP_CODES::OP_NUMEQUALVERIFY => todo!(),
            OP_CODES::OP_NUMNOTEQUAL => todo!(),
            OP_CODES::OP_LESSTHAN => todo!(),
            OP_CODES::OP_GREATERTHAN => todo!(),
            OP_CODES::OP_LESSTHANOREQUAL => todo!(),
            OP_CODES::OP_GREATERTHANOREQUAL => todo!(),
            OP_CODES::OP_MIN => todo!(),
            OP_CODES::OP_MAX => todo!(),
            OP_CODES::OP_WITHIN => todo!(),
            OP_CODES::OP_RIPEMD160 => todo!(),
            OP_CODES::OP_SHA1 => todo!(),
            OP_CODES::OP_SHA256 => todo!(),
            OP_CODES::OP_HASH160 => op_hash160(stack),
            OP_CODES::OP_HASH256 => op_hash256(stack),
            OP_CODES::OP_CODESEPARATOR => todo!(),
            OP_CODES::OP_CHECKSIG => {
                if let Some(z) = z {
                    op_checksig(stack, z)
                } else {
                    false
                }
            }
            OP_CODES::OP_CHECKSIGVERIFY => todo!(),
            OP_CODES::OP_CHECKMULTISIG => todo!(),
            OP_CODES::OP_CHECKMULTISIGVERIFY => todo!(),
            OP_CODES::OP_NOP1 => todo!(),
            OP_CODES::OP_CHECKLOCKTIMEVERIFY => todo!(),
            OP_CODES::OP_CHECKSEQUENCEVERIFY => todo!(),
            OP_CODES::OP_NOP4 => todo!(),
            OP_CODES::OP_NOP5 => todo!(),
            OP_CODES::OP_NOP6 => todo!(),
            OP_CODES::OP_NOP7 => todo!(),
            OP_CODES::OP_NOP8 => todo!(),
            OP_CODES::OP_NOP9 => todo!(),
            OP_CODES::OP_NOP10 => todo!(),
        }
    }
}

impl From<u16> for OP_CODES {
    fn from(value: u16) -> Self {
        match value {
            0 => OP_CODES::OP_0,
            76 => OP_CODES::OP_PUSHDATA1,
            77 => OP_CODES::OP_PUSHDATA2,
            78 => OP_CODES::OP_PUSHDATA4,
            79 => OP_CODES::OP_1NEGATE,
            81 => OP_CODES::OP_1,
            82 => OP_CODES::OP_2,
            83 => OP_CODES::OP_3,
            84 => OP_CODES::OP_4,
            85 => OP_CODES::OP_5,
            86 => OP_CODES::OP_6,
            87 => OP_CODES::OP_7,
            88 => OP_CODES::OP_8,
            89 => OP_CODES::OP_9,
            90 => OP_CODES::OP_10,
            91 => OP_CODES::OP_11,
            92 => OP_CODES::OP_12,
            93 => OP_CODES::OP_13,
            94 => OP_CODES::OP_14,
            95 => OP_CODES::OP_15,
            96 => OP_CODES::OP_16,
            97 => OP_CODES::OP_NOP,
            99 => OP_CODES::OP_IF,
            100 => OP_CODES::OP_NOTIF,
            103 => OP_CODES::OP_ELSE,
            104 => OP_CODES::OP_ENDIF,
            105 => OP_CODES::OP_VERIFY,
            106 => OP_CODES::OP_RETURN,
            107 => OP_CODES::OP_TOALTSTACK,
            108 => OP_CODES::OP_FROMALTSTACK,
            109 => OP_CODES::OP_2DROP,
            110 => OP_CODES::OP_2DUP,
            111 => OP_CODES::OP_3DUP,
            112 => OP_CODES::OP_2OVER,
            113 => OP_CODES::OP_2ROT,
            114 => OP_CODES::OP_2SWAP,
            115 => OP_CODES::OP_IFDUP,
            116 => OP_CODES::OP_DEPTH,
            117 => OP_CODES::OP_DROP,
            118 => OP_CODES::OP_DUP,
            119 => OP_CODES::OP_NIP,
            120 => OP_CODES::OP_OVER,
            121 => OP_CODES::OP_PICK,
            122 => OP_CODES::OP_ROLL,
            123 => OP_CODES::OP_ROT,
            124 => OP_CODES::OP_SWAP,
            125 => OP_CODES::OP_TUCK,
            130 => OP_CODES::OP_SIZE,
            135 => OP_CODES::OP_EQUAL,
            136 => OP_CODES::OP_EQUALVERIFY,
            139 => OP_CODES::OP_1ADD,
            140 => OP_CODES::OP_1SUB,
            143 => OP_CODES::OP_NEGATE,
            144 => OP_CODES::OP_ABS,
            145 => OP_CODES::OP_NOT,
            146 => OP_CODES::OP_0NOTEQUAL,
            147 => OP_CODES::OP_ADD,
            148 => OP_CODES::OP_SUB,
            154 => OP_CODES::OP_BOOLAND,
            155 => OP_CODES::OP_BOOLOR,
            156 => OP_CODES::OP_NUMEQUAL,
            157 => OP_CODES::OP_NUMEQUALVERIFY,
            158 => OP_CODES::OP_NUMNOTEQUAL,
            159 => OP_CODES::OP_LESSTHAN,
            160 => OP_CODES::OP_GREATERTHAN,
            161 => OP_CODES::OP_LESSTHANOREQUAL,
            162 => OP_CODES::OP_GREATERTHANOREQUAL,
            163 => OP_CODES::OP_MIN,
            164 => OP_CODES::OP_MAX,
            165 => OP_CODES::OP_WITHIN,
            166 => OP_CODES::OP_RIPEMD160,
            167 => OP_CODES::OP_SHA1,
            168 => OP_CODES::OP_SHA256,
            169 => OP_CODES::OP_HASH160,
            170 => OP_CODES::OP_HASH256,
            171 => OP_CODES::OP_CODESEPARATOR,
            172 => OP_CODES::OP_CHECKSIG,
            173 => OP_CODES::OP_CHECKSIGVERIFY,
            174 => OP_CODES::OP_CHECKMULTISIG,
            175 => OP_CODES::OP_CHECKMULTISIGVERIFY,
            176 => OP_CODES::OP_NOP1,
            177 => OP_CODES::OP_CHECKLOCKTIMEVERIFY,
            178 => OP_CODES::OP_CHECKSEQUENCEVERIFY,
            179 => OP_CODES::OP_NOP4,
            180 => OP_CODES::OP_NOP5,
            181 => OP_CODES::OP_NOP6,
            182 => OP_CODES::OP_NOP7,
            183 => OP_CODES::OP_NOP8,
            184 => OP_CODES::OP_NOP9,
            185 => OP_CODES::OP_NOP10,
            _ => OP_CODES::UNSET,
        }
    }
}
