
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "GtzcMpcbb",
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
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cfglockr1",
                description: None,
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "seccfgr",
                description: None,
                array: Some(Array::Regular(RegularArray { len: 32, stride: 4 })),
                byte_offset: 0x100,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "privcfgr",
                description: None,
                array: Some(Array::Regular(RegularArray { len: 32, stride: 4 })),
                byte_offset: 0x200,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[],
    enums: &[],
};
