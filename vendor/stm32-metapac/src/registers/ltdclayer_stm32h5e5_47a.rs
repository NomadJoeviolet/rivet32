
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "LtdcLayer",
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
                name: "whpcr",
                description: None,
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "wvpcr",
                description: None,
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "ckcr",
                description: None,
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "pfcr",
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
                name: "cacr",
                description: None,
                array: None,
                byte_offset: 0x14,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "dccr",
                description: None,
                array: None,
                byte_offset: 0x18,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "bfcr",
                description: None,
                array: None,
                byte_offset: 0x1c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cfbar",
                description: None,
                array: None,
                byte_offset: 0x28,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cfblr",
                description: None,
                array: None,
                byte_offset: 0x2c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cfblnr",
                description: None,
                array: None,
                byte_offset: 0x30,
                inner: BlockItemInner::Register(Register {
                    access: Access::Raw,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "clutwr",
                description: None,
                array: None,
                byte_offset: 0x40,
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
