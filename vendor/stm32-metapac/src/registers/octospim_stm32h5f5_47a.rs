
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Octospim",
        extends: None,
        description: None,
        items: &[BlockItem {
            name: "cr",
            description: None,
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("OctospimCr"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "OctospimCr",
        extends: None,
        description: None,
        bit_size: 32,
        fields: &[
            Field {
                name: "muxen",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "mode",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "cssel_ovr_en",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "cssel_ovr_o1",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "cssel_ovr_o2",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "req2ack_time",
                description: None,
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                bit_size: 8,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[],
};
