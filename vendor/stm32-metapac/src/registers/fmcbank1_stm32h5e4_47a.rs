
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "FmcBank1",
        extends: None,
        description: None,
        items: &[
            BlockItem {
                name: "btcr",
                description: None,
                array: Some(Array::Regular(RegularArray { len: 8, stride: 4 })),
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "pcscntr",
                description: None,
                array: None,
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("FmcBank1Pcscntr"),
                }),
            },
        ],
    }],
    fieldsets: &[FieldSet {
        name: "FmcBank1Pcscntr",
        extends: None,
        description: None,
        bit_size: 32,
        fields: &[
            Field {
                name: "cscount",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            },
            Field {
                name: "cntb1en",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "cntb2en",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "cntb3en",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "cntb4en",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[],
};
