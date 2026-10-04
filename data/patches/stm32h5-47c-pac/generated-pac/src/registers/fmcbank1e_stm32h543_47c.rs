
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "FmcBank1e",
        extends: None,
        description: None,
        items: &[BlockItem {
            name: "bwtr",
            description: None,
            array: Some(Array::Regular(RegularArray { len: 7, stride: 4 })),
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::Raw,
                bit_size: 32,
                fieldset: None,
            }),
        }],
    }],
    fieldsets: &[],
    enums: &[],
};
