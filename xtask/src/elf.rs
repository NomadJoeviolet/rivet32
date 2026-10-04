//! Strict little-endian ARM ELF32 and generated linker-memory validation.
use crate::Result;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct MemoryRegion {
    pub name: String,
    pub origin: u64,
    pub length: u64,
}
impl MemoryRegion {
    fn contains(&self, start: u64, len: u64) -> bool {
        start >= self.origin
            && start
                .checked_add(len)
                .zip(self.origin.checked_add(self.length))
                .is_some_and(|(end, limit)| end <= limit)
    }
    fn is_ram(&self) -> bool {
        let name = self.name.to_ascii_uppercase();
        name.contains("RAM") || name.contains("TCM") || name.contains("CCM")
    }
}
fn number(value: &str) -> Result<u64> {
    let (digits, scale) = if let Some(n) = value.strip_suffix(['K', 'k']) {
        (n, 1024)
    } else if let Some(n) = value.strip_suffix(['M', 'm']) {
        (n, 1024 * 1024)
    } else {
        (value, 1)
    };
    let n = if let Some(hex) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16)
    } else {
        digits.parse()
    }
    .map_err(|_| format!("unsupported memory number {value}"))?;
    n.checked_mul(scale).ok_or("memory number overflow".into())
}
pub fn parse_memory_regions(source: &str) -> Result<Vec<MemoryRegion>> {
    let mut clean = String::new();
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'/') {
            for c in chars.by_ref() {
                if c == '\n' {
                    break;
                }
            }
            clean.push(' ');
        } else if c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            let mut previous = '\0';
            let mut ended = false;
            for c in chars.by_ref() {
                if previous == '*' && c == '/' {
                    ended = true;
                    break;
                }
                previous = c;
            }
            if !ended {
                return Err("unterminated memory comment".into());
            }
            clean.push(' ');
        } else if "{}():,=;".contains(c) {
            clean.push(' ');
            clean.push(c);
            clean.push(' ');
        } else {
            clean.push(c);
        }
    }
    let tokens: Vec<_> = clean.split_whitespace().collect();
    let memory = tokens
        .iter()
        .position(|t| *t == "MEMORY")
        .ok_or("linker script has no MEMORY block")?;
    if tokens.get(memory + 1) != Some(&"{") {
        return Err("invalid MEMORY block".into());
    }
    let mut i = memory + 2;
    let mut regions = vec![];
    while tokens.get(i) != Some(&"}") {
        if tokens.get(i) == Some(&";") {
            i += 1;
            continue;
        }
        let name = *tokens.get(i).ok_or("unterminated MEMORY block")?;
        i += 1;
        if tokens.get(i) == Some(&"(") {
            while tokens.get(i) != Some(&")") {
                i += 1;
                if i >= tokens.len() {
                    return Err("invalid memory attributes".into());
                }
            }
            i += 1;
        }
        for expected in [":", "ORIGIN", "="] {
            if tokens.get(i) != Some(&expected) {
                return Err(format!("expected {expected} in memory region {name}"));
            }
            i += 1;
        }
        let origin = number(tokens.get(i).ok_or("missing region origin")?)?;
        i += 1;
        for expected in [",", "LENGTH", "="] {
            if tokens.get(i) != Some(&expected) {
                return Err(format!("expected {expected} in memory region {name}"));
            }
            i += 1;
        }
        let length = number(tokens.get(i).ok_or("missing region length")?)?;
        i += 1;
        if length == 0
            || origin
                .checked_add(length)
                .is_none_or(|end| end > 1_u64 << 32)
        {
            return Err(format!("invalid memory region {name}"));
        }
        regions.push(MemoryRegion {
            name: name.into(),
            origin,
            length,
        });
    }
    if regions.is_empty() {
        return Err("empty memory map".into());
    }
    Ok(regions)
}

#[derive(Debug, Serialize)]
pub struct LoadSegment {
    pub file_offset: u64,
    pub virtual_address: u64,
    pub load_address: u64,
    pub file_bytes: u64,
    pub memory_bytes: u64,
    pub flags: u32,
}
#[derive(Debug, Serialize)]
pub struct ElfReport {
    pub entry: u32,
    pub initial_sp: u32,
    pub reset: u32,
    pub vector_address: u32,
    pub loaded_bytes: u64,
    pub memory_bytes: u64,
    pub load_segments: Vec<LoadSegment>,
    pub silicon_memory_validation: &'static str,
}
fn bytes_at(bytes: &[u8], at: u64, len: u64) -> Result<&[u8]> {
    let end = at.checked_add(len).ok_or("ELF offset overflow")?;
    let start = usize::try_from(at).map_err(|_| "ELF offset too large")?;
    let end = usize::try_from(end).map_err(|_| "ELF offset too large")?;
    bytes
        .get(start..end)
        .ok_or_else(|| "truncated ELF table or segment".into())
}
fn u16_at(bytes: &[u8], at: u64) -> Result<u16> {
    Ok(u16::from_le_bytes(
        bytes_at(bytes, at, 2)?.try_into().unwrap(),
    ))
}
fn u32_at(bytes: &[u8], at: u64) -> Result<u32> {
    Ok(u32::from_le_bytes(
        bytes_at(bytes, at, 4)?.try_into().unwrap(),
    ))
}
pub fn inspect_elf(bytes: &[u8], regions: &[MemoryRegion]) -> Result<ElfReport> {
    if regions.iter().any(|r| {
        r.length == 0
            || r.origin
                .checked_add(r.length)
                .is_none_or(|end| end > 1_u64 << 32)
    }) {
        return Err("memory region exceeds ELF32 address space".into());
    }
    if bytes_at(bytes, 0, 7)? != b"\x7fELF\x01\x01\x01" {
        return Err("expected little-endian ELF32 version 1".into());
    }
    if u16_at(bytes, 16)? != 2 || u16_at(bytes, 18)? != 40 || u32_at(bytes, 20)? != 1 {
        return Err("expected executable ARM ELF".into());
    }
    if u16_at(bytes, 40)? != 52 {
        return Err("invalid ELF32 header size".into());
    }
    let entry = u32_at(bytes, 24)?;
    let phoff = u64::from(u32_at(bytes, 28)?);
    let phsize = u64::from(u16_at(bytes, 42)?);
    let phnum = u64::from(u16_at(bytes, 44)?);
    if phsize != 32 || phnum == 0 {
        return Err("invalid ELF32 program headers".into());
    }
    bytes_at(bytes, phoff, phsize * phnum)?;
    let mut segments = vec![];
    for i in 0..phnum {
        let p = phoff + i * phsize;
        if u32_at(bytes, p)? != 1 {
            continue;
        }
        let offset = u64::from(u32_at(bytes, p + 4)?);
        let virtual_address = u64::from(u32_at(bytes, p + 8)?);
        let load_address = u64::from(u32_at(bytes, p + 12)?);
        let file_bytes = u64::from(u32_at(bytes, p + 16)?);
        let memory_bytes = u64::from(u32_at(bytes, p + 20)?);
        let flags = u32_at(bytes, p + 24)?;
        if virtual_address + memory_bytes > 1_u64 << 32 || load_address + file_bytes > 1_u64 << 32 {
            return Err("ELF segment exceeds 32-bit address space".into());
        }
        if file_bytes > memory_bytes {
            return Err("ELF file segment exceeds memory size".into());
        }
        bytes_at(bytes, offset, file_bytes)?;
        if memory_bytes > 0
            && !regions
                .iter()
                .any(|r| r.contains(virtual_address, memory_bytes))
        {
            return Err(format!(
                "ELF memory segment at {virtual_address:#x} exceeds linker regions"
            ));
        }
        if file_bytes > 0 && !regions.iter().any(|r| r.contains(load_address, file_bytes)) {
            return Err(format!(
                "ELF load segment at {load_address:#x} exceeds linker regions"
            ));
        }
        if memory_bytes > 0 {
            for previous in &segments {
                let previous: &LoadSegment = previous;
                if virtual_address < previous.virtual_address + previous.memory_bytes
                    && previous.virtual_address < virtual_address + memory_bytes
                {
                    return Err("overlapping ELF virtual memory segments".into());
                }
                if file_bytes > 0
                    && previous.file_bytes > 0
                    && load_address < previous.load_address + previous.file_bytes
                    && previous.load_address < load_address + file_bytes
                {
                    return Err("overlapping ELF load image segments".into());
                }
            }
            segments.push(LoadSegment {
                file_offset: offset,
                virtual_address,
                load_address,
                file_bytes,
                memory_bytes,
                flags,
            });
        }
    }
    if segments.is_empty() {
        return Err("ELF contains no loadable segments".into());
    }
    let shoff = u64::from(u32_at(bytes, 32)?);
    let shsize = u64::from(u16_at(bytes, 46)?);
    let shnum = u64::from(u16_at(bytes, 48)?);
    let strindex = u64::from(u16_at(bytes, 50)?);
    if shsize != 40 || shnum == 0 || strindex >= shnum {
        return Err("invalid ELF section table".into());
    }
    bytes_at(bytes, shoff, shsize * shnum)?;
    let string_header = shoff + strindex * shsize;
    let strings = bytes_at(
        bytes,
        u64::from(u32_at(bytes, string_header + 16)?),
        u64::from(u32_at(bytes, string_header + 20)?),
    )?;
    let mut vector = None;
    for i in 0..shnum {
        let s = shoff + i * shsize;
        let index = u32_at(bytes, s)? as usize;
        let name = strings
            .get(index..)
            .and_then(|b| b.split(|c| *c == 0).next())
            .ok_or("invalid ELF section name")?;
        if name == b".vector_table" {
            if vector.is_some() {
                return Err("duplicate vector table".into());
            }
            if u32_at(bytes, s + 4)? != 1 || u32_at(bytes, s + 8)? & 2 == 0 {
                return Err("vector table must contain allocated program bytes".into());
            }
            let size = u32_at(bytes, s + 20)?;
            if size < 8 {
                return Err("vector table is missing SP/Reset".into());
            }
            let offset = u64::from(u32_at(bytes, s + 16)?);
            bytes_at(bytes, offset, u64::from(size))?;
            vector = Some((
                u32_at(bytes, s + 12)?,
                offset,
                size,
                u32_at(bytes, offset)?,
                u32_at(bytes, offset + 4)?,
            ));
        }
    }
    let (vector_address, vector_offset, vector_size, initial_sp, reset) =
        vector.ok_or("ELF has no .vector_table")?;
    if initial_sp % 8 != 0
        || !regions.iter().any(|r| {
            r.is_ram()
                && u64::from(initial_sp) > r.origin
                && r.contains(u64::from(initial_sp) - 1, 1)
        })
    {
        return Err("initial stack pointer is unaligned or outside RAM".into());
    }
    if reset & 1 == 0 || entry & 1 == 0 {
        return Err("Reset and entry must be Thumb addresses".into());
    }
    let executable = |address: u32| {
        segments.iter().any(|s| {
            s.flags & 1 != 0
                && u64::from(address & !1) >= s.virtual_address
                && u64::from(address & !1) < s.virtual_address + s.file_bytes
        })
    };
    if !executable(reset) || !executable(entry) {
        return Err("Reset/entry lies outside executable load bytes".into());
    }
    if !segments.iter().any(|s| {
        u64::from(vector_address) >= s.virtual_address
            && u64::from(vector_address) + u64::from(vector_size)
                <= s.virtual_address + s.file_bytes
            && vector_offset.checked_sub(s.file_offset)
                == u64::from(vector_address).checked_sub(s.virtual_address)
    }) {
        return Err("vector table does not match its loadable file segment".into());
    }
    Ok(ElfReport {
        entry,
        initial_sp,
        reset,
        vector_address,
        loaded_bytes: segments.iter().map(|s| s.file_bytes).sum(),
        memory_bytes: segments.iter().map(|s| s.memory_bytes).sum(),
        load_segments: segments,
        silicon_memory_validation: "generated linker regions checked; independent per-core silicon RAM/alias/reservation validation not performed",
    })
}
