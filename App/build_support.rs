//! Link policy for the independently linked H7 CM7/CM4 minimal images.
//! All capacities come from the selected, pinned PAC metadata.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Core {
    Cm7,
    Cm4,
}

impl Core {
    pub fn from_chip(chip: &str) -> Result<Option<Self>, String> {
        if !["stm32h745", "stm32h747", "stm32h755", "stm32h757"]
            .iter()
            .any(|prefix| chip.starts_with(prefix))
        {
            return Ok(None);
        }
        match chip.rsplit_once('-').map(|(_, core)| core) {
            Some("cm7") => Ok(Some(Self::Cm7)),
            Some("cm4") => Ok(Some(Self::Cm4)),
            _ => Err(format!(
                "dual-core H7 requires an explicit cm7 or cm4 feature: {chip}"
            )),
        }
    }
}

pub fn validate_time_driver(core: Core, explicit: &[&str]) -> Result<(), String> {
    let expected = match core {
        Core::Cm7 => "TIM5",
        Core::Cm4 => "TIM2",
    };
    if explicit.is_empty() || explicit == [expected] {
        Ok(())
    } else {
        Err(format!(
            "dual-core {core:?} owns {expected}; conflicting explicit time driver(s): {explicit:?}"
        ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region<'a> {
    pub name: &'a str,
    pub origin: u32,
    pub length: u32,
}

impl Region<'_> {
    pub fn end(self) -> Result<u32, String> {
        if self.length == 0 {
            return Err(format!("empty memory region {}", self.name));
        }
        self.origin
            .checked_add(self.length)
            .ok_or_else(|| format!("memory region {} overflows", self.name))
    }
}

#[derive(Debug)]
pub struct DualLayout<'a> {
    pub flash: Region<'a>,
    pub ram: Region<'a>,
    pub shared: Region<'a>,
}

impl<'a> DualLayout<'a> {
    pub fn from_regions(core: Core, regions: &[Region<'a>]) -> Result<Self, String> {
        let find = |name: &str, origin| -> Result<Region<'a>, String> {
            let region = *regions
                .iter()
                .find(|r| r.name == name)
                .ok_or_else(|| format!("missing pinned PAC region {name}"))?;
            if region.origin != origin {
                return Err(format!("unexpected {name} origin 0x{:08x}", region.origin));
            }
            region.end()?;
            Ok(region)
        };
        let bank1 = find("BANK_1", 0x0800_0000)?;
        let bank2 = find("BANK_2", 0x0810_0000)?;
        let axi = find("AXISRAM", 0x2400_0000)?;
        let sram1 = find("SRAM1", 0x3000_0000)?;
        let sram4 = find("SRAM4", 0x3800_0000)?;
        let all = [bank1, bank2, axi, sram1, sram4];
        for (i, region) in all.iter().enumerate() {
            for other in &all[i + 1..] {
                if region.origin < other.end()? && other.origin < region.end()? {
                    return Err(format!("overlapping {} and {}", region.name, other.name));
                }
            }
        }
        if sram4.length < 4096 {
            return Err("SRAM4 cannot hold the reserved 4 KiB shared window".into());
        }
        Ok(Self {
            flash: match core {
                Core::Cm7 => bank1,
                Core::Cm4 => bank2,
            },
            ram: match core {
                Core::Cm7 => axi,
                Core::Cm4 => sram1,
            },
            shared: Region {
                name: "EMBASSY_SHARED",
                origin: sram4.origin,
                length: 4096,
            },
        })
    }

    pub fn memory_script(&self) -> String {
        format!(
            "/* Selected PAC metadata; independent dual-core image partition. */\nMEMORY {{\n  FLASH : ORIGIN = 0x{:08x}, LENGTH = {}\n  RAM : ORIGIN = 0x{:08x}, LENGTH = {}\n  EMBASSY_SHARED : ORIGIN = 0x{:08x}, LENGTH = {}\n}}\n",
            self.flash.origin,
            self.flash.length,
            self.ram.origin,
            self.ram.length,
            self.shared.origin,
            self.shared.length,
        )
    }

    pub fn shared_script(&self) -> String {
        format!(
            "/* NOLOAD prevents either image from overwriting its peer's live shared state. */\nSECTIONS {{\n  .embassy_shared 0x{:08x} (NOLOAD) : ALIGN(32) {{\n    __embodied_shared_start = .;\n    KEEP(*(.embassy_shared))\n    __embodied_shared_end = .;\n  }} > EMBASSY_SHARED\n}} INSERT AFTER .got;\nASSERT(ORIGIN(FLASH) == 0x{:08x}, \"wrong dual-core FLASH partition\");\nASSERT(LENGTH(FLASH) == {}, \"wrong dual-core FLASH capacity\");\nASSERT(ORIGIN(RAM) == 0x{:08x}, \"wrong dual-core private RAM partition\");\nASSERT(LENGTH(RAM) == {}, \"wrong dual-core RAM capacity\");\nASSERT(SIZEOF(.embassy_shared) > 0, \"missing dual-core SharedData\");\nASSERT(SIZEOF(.embassy_shared) <= {}, \"dual-core SharedData exceeds reserved window\");\nASSERT(__embodied_shared_data == __embodied_shared_start, \"SharedData must start at the same fixed address\");\n",
            self.shared.origin,
            self.flash.origin,
            self.flash.length,
            self.ram.origin,
            self.ram.length,
            self.shared.length,
        )
    }
}
