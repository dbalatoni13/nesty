use crate::nes::Powerable;

use bitfield_struct::bitfield;

#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
enum RegPPUCtrlNametableSelect {
    Address2000,
    Address2400,
    Address2800,
    Address2C00,
}

impl RegPPUCtrlNametableSelect {
    // This has to be a const fn
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Address2000,
            1 => Self::Address2400,
            2 => Self::Address2800,
            3 => Self::Address2C00,
            _ => panic!("Failed to parse enum"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum RegPPUIncrementMode {
    AddOneGoingAcross,
    Add32GoingDown,
}
impl RegPPUIncrementMode {
    // This has to be a const fn
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::AddOneGoingAcross,
            1 => Self::Add32GoingDown,
            _ => panic!("Failed to parse enum"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum RegPPUSprTileSelect {
    Address0000,
    Address1000,
}

impl RegPPUSprTileSelect {
    // This has to be a const fn
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Address0000,
            1 => Self::Address1000,
            _ => panic!("Failed to parse enum"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum RegPPUBgTileSelect {
    Address0000,
    Address1000,
}

impl RegPPUBgTileSelect {
    // This has to be a const fn
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Address0000,
            1 => Self::Address1000,
            _ => panic!("Failed to parse enum"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum RegPPUSprHeight {
    EightByEight,
    EightBySixteen,
}

impl RegPPUSprHeight {
    // This has to be a const fn
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::EightByEight,
            1 => Self::EightBySixteen,
            _ => panic!("Failed to parse enum"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum RegPPUMasterSlave {
    ReadBackdropFromEXT,
    OutputColorOnEXT,
}

impl RegPPUMasterSlave {
    // This has to be a const fn
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::ReadBackdropFromEXT,
            1 => Self::OutputColorOnEXT,
            _ => panic!("Failed to parse enum"),
        }
    }
}

#[bitfield(u8)]
struct RegPPUCtrl {
    #[bits(2)]
    nametable_select: RegPPUCtrlNametableSelect,
    #[bits(1)]
    increment_mode: RegPPUIncrementMode,
    #[bits(1)]
    spr_tile_select: RegPPUSprTileSelect,
    #[bits(1)]
    bg_tile_select: RegPPUBgTileSelect,
    #[bits(1)]
    spr_height: RegPPUSprHeight,
    #[bits(1)]
    ppu_master_slave: RegPPUMasterSlave,
    #[bits(1)]
    nmi_enable: bool,
}

#[bitfield(u8)]
struct RegPPUMask {
    #[bits(1)]
    greyscale: bool,
    #[bits(1)]
    bg_left_col_enable: bool,
    #[bits(1)]
    spr_left_col_enable: bool,
    #[bits(1)]
    bg_enable: bool,
    #[bits(1)]
    spr_enable: bool,
    #[bits(1)]
    emphasize_red: bool,
    #[bits(1)]
    emphasize_green: bool,
    #[bits(1)]
    emphasize_blue: bool,
}

#[bitfield(u8)]
struct RegPPUStatus {
    #[bits(5)]
    open_bus: u8,
    #[bits(1)]
    spr_overflow: bool,
    #[bits(1)]
    spr_0_hit: bool,
    #[bits(1)]
    in_vblank: bool,
}

const VRAM_SIZE: usize = 2 * 1024;

#[derive(Default)]
pub struct PPU {
    memory: Vec<u8>,

    reg_PPUCTRL: RegPPUCtrl,
    reg_PPUMASK: RegPPUMask,
    reg_PPUSTATUS: RegPPUStatus,
    reg_OAMADDR: u8,
    reg_OAMDATA: u8,
    reg_PPUSCROLL: u8,
    reg_PPUADDR: u8,
    reg_PPUDATA: u8,
    reg_OAMDMA: u8,
}

impl PPU {
    pub fn read_reg(&self, reg: u8) -> u8 {
        match reg {
            0 => self.reg_PPUCTRL.into_bits(),
            1 => self.reg_PPUMASK.into_bits(),
            2 => self.reg_PPUSTATUS.into_bits(),
            3 => self.reg_OAMADDR,
            4 => self.reg_OAMDATA,
            5 => self.reg_PPUSCROLL,
            6 => self.reg_PPUADDR,
            7 => self.reg_PPUDATA,
            _ => panic!("Failed to read reg"),
        }
    }
    pub fn write_reg(&mut self, reg: u8, value: u8) {
        match reg {
            0 => self.reg_PPUCTRL = RegPPUCtrl::from_bits(value),
            1 => self.reg_PPUMASK = RegPPUMask::from_bits(value),
            2 => self.reg_PPUSTATUS = RegPPUStatus::from_bits(value),
            3 => self.reg_OAMADDR = value,
            4 => self.reg_OAMDATA = value,
            5 => self.reg_PPUSCROLL = value,
            6 => self.reg_PPUADDR = value,
            7 => self.reg_PPUDATA = value,
            _ => panic!("Failed to write reg"),
        }
    }
}

impl Powerable for PPU {
    fn power_on(&mut self) {
        self.memory = vec![0; VRAM_SIZE];

        self.reg_PPUCTRL = RegPPUCtrl::from_bits(0);
        self.reg_PPUMASK = RegPPUMask::from_bits(0);
        self.reg_PPUSTATUS.set_spr_0_hit(false);
        self.reg_OAMADDR = 0;
        self.reg_PPUSCROLL = 0;
        self.reg_PPUADDR = 0;
        self.reg_PPUDATA = 0;
    }
    fn reset(&mut self) {
        self.memory = vec![0; VRAM_SIZE];

        self.reg_PPUCTRL = RegPPUCtrl::from_bits(0);
        self.reg_PPUMASK = RegPPUMask::from_bits(0);
        self.reg_PPUSCROLL = 0;
        self.reg_PPUDATA = 0;
    }
}
