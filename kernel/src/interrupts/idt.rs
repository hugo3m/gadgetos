use crate::interrupts::exceptions;
use core::arch::asm;
use core::mem::size_of;

/// Number of entries in the IDT
const IDT_ENTRIES: usize = 256;

/// Default value of IDT entry
pub static IDT_ENTRY: IdtEntry = {
    // segment selector of GDT entry
    let selector: u16 = {
        // ring privilege level (0 for ring 0)
        let rpl = 0b00 << 0;
        // 0 => GDT, 1 => LDT
        let ti = 0b0 << 2;
        // bits 3-15 of GDT code entry, in my case 0x8 (0b1000)
        let index = 0b1 << 3;

        rpl | ti | index
    };
    // entry flags
    let flags: u8 = {
        //gate type: 0xe => 32bit interrupt gate, 0xf => 32bit trap gate
        let gate_type = 0xe << 0;
        // always zero
        let zero = 0 << 3;
        // ring allowed to use this interrupt
        let dpl = 0 << 5;
        // presence bit, 1 to enable
        let p = 1 << 7;
        gate_type | zero | dpl | p
    };

    IdtEntry {
        low_offset: 0,
        selector: selector,
        always0: 0u8,
        flags: flags,
        high_offset: 0,
    }
};

/// Static and unique IDT
pub static mut IDT: Idt = Idt {
    entries: [IDT_ENTRY; IDT_ENTRIES],
};

/// Structure for Interruptor descriptor table.
/// Must be packed to avoid padding for running ASM command.
#[repr(C, packed)]
pub struct Idt {
    /// Array storing IDT entries
    entries: [IdtEntry; IDT_ENTRIES],
}

impl Idt {
    /// Init the IDT.
    pub fn init(&mut self) {
        for i in 0..IDT_ENTRIES {
            self.add(i, exceptions::generic_handler as u32);
        }
        self.add_exceptions()
    }
    /// Add a new handler for the specific index.
    ///
    /// ## Arguments
    /// * index: entry at the index
    /// * handler:
    pub fn add(&mut self, index: usize, handler: u32) {
        self.entries[index].set(handler);
    }
    /// Add basic exceptions to the IDT.
    fn add_exceptions(&mut self) {
        self.add(0x0, exceptions::div_error as u32);
        self.add(0x6, exceptions::invalid_opcode as u32);
        self.add(0x8, exceptions::double_fault as u32);
        self.add(0xd, exceptions::general_protection_fault as u32);
        self.add(0xe, exceptions::page_fault as u32);
    }
    /// Load the IDT running asm!(lidt).
    pub fn load(&mut self) {
        // Instanciate a descriptor for the IDT
        let descriptor: IdtDescriptor = IdtDescriptor {
            size: (self.entries.len() * size_of::<IdtEntry>() - 1) as u16,
            offset: self as *const Idt,
        };
        // Load the IDT
        unsafe {
            asm!("lidt [{0}]", in(reg) &descriptor);
        }
    }
}

/// Structure describing an IDT entry
/// Must be packed to avoid padding for running ASM command.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct IdtEntry {
    /// 16 lowest bits of the 32 bits address in the segment.
    low_offset: u16,
    /// offset in the GDT
    selector: u16,
    /// always zero
    always0: u8,
    /// Flags
    flags: u8,
    /// 16 highest bits of the 32 bits address in the segment.
    high_offset: u16,
}

impl IdtEntry {
    /// Set the offset of the entry.
    ///
    /// ## Arguments
    /// * offset: value to set for the offset
    pub fn set(&mut self, offset: u32) {
        // Retrieve lower 16 bits of the offset
        self.low_offset = ((offset << 16) >> 16) as u16;
        // Retrieve higher 16 bits of the offset
        self.high_offset = (offset >> 16) as u16;
    }
}
#[repr(C, packed)]
/// Must be packed to avoid padding for running ASM command.
pub struct IdtDescriptor {
    /// IDT size
    size: u16,
    /// Pointer to IDT
    offset: *const Idt,
}
