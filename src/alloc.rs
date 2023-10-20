// A test allocator that I made to help me understand the stack and memory.

use byteorder::{ByteOrder, LittleEndian};

pub static mut MEMORY: Vec<u8> = Vec::new();
pub static mut GLOBAL_ALLOC: DefaultAllocator = DefaultAllocator::new();
pub static mut STACK_POINTER: usize = 0;

#[derive(Debug, Clone, Copy)]
pub struct DefaultAllocator {}

impl DefaultAllocator {
    pub const fn new() -> Self {
        Self {}
    }

    pub unsafe fn push(&mut self, item: &[u8]) -> usize {
        let spos = STACK_POINTER.clone();
        let pos = spos + 4;
        let mut buf = [0u8; 4];

        LittleEndian::write_u32(&mut buf, pos as u32);

        MEMORY.insert(pos, buf[0]);
        MEMORY.insert(pos + 1, buf[1]);
        MEMORY.insert(pos + 2, buf[2]);
        MEMORY.insert(pos + 3, buf[3]);

        STACK_POINTER += 4;

        for (idx, item) in item.iter().enumerate() {
            MEMORY[pos + idx] = *item;
        }

        for i in 0..(STACK_POINTER - 4) {
            if i % 4 == 0 {
                let mut buf = [MEMORY[i], MEMORY[i + 1], MEMORY[i + 2], MEMORY[i + 3]];
                let mut item = LittleEndian::read_u32(&buf);

                item += 4;

                LittleEndian::write_u32(&mut buf, item);

                MEMORY[i] = buf[0];
                MEMORY[i + 1] = buf[1];
                MEMORY[i + 2] = buf[2];
                MEMORY[i + 3] = buf[3];
            }
        }

        spos
    }

    pub unsafe fn pop(&mut self, spos: &[u8]) -> Box<[u8]> {
        let sp = LittleEndian::read_u32(&spos) as usize;
        let next_sp = (LittleEndian::read_u32(&spos) + 4) as usize;
        let spv = LittleEndian::read_u32(&MEMORY[sp..(sp + 4)]) as usize;
        let next_spv = LittleEndian::read_u32(&MEMORY[next_sp..(next_sp + 4)]) as usize;
        let data = MEMORY[spv..next_spv].to_vec().into_boxed_slice();

        for v in spv..next_spv {
            MEMORY.remove(v);
        }

        MEMORY.remove(sp);
        MEMORY.remove(sp + 1);
        MEMORY.remove(sp + 2);
        MEMORY.remove(sp + 3);

        data
    }
}
