
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Dlyb",
        extends: None,
        description: None,
        items: &[
            BlockItem {
                name: "cr",
                description: None,
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "cfgr",
                description: None,
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: Some("Cfgr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "sel",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "unit",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lng",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lngf",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "den",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sen",
                    description: None,
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
