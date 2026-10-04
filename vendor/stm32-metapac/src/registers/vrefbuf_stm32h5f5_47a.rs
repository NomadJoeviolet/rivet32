
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Vrefbuf",
        extends: None,
        description: None,
        items: &[
            BlockItem {
                name: "csr",
                description: None,
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("VrefbufCsr"),
                }),
            },
            BlockItem {
                name: "ccr",
                description: None,
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("VrefbufCcr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "VrefbufCcr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[Field {
                name: "trim",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 6,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "VrefbufCsr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "envr",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hiz",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrr",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrs",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
