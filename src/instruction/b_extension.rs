use crate::instruction::InstFormat;
use crate::instruction::Opcode;
use core::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum BOpcode {
    BCLR,
    BCLRI,
    BEXT,
    BEXTI,
    BINV,
    BINVI,
    BSET,
    BSETI,
}

impl Display for BOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::BCLR => write!(f, "bclr"),
            Self::BCLRI => write!(f, "bclri"),
            Self::BEXT => write!(f, "bext"),
            Self::BEXTI => write!(f, "bexti"),
            Self::BINV => write!(f, "binv"),
            Self::BINVI => write!(f, "binvi"),
            Self::BSET => write!(f, "bset"),
            Self::BSETI => write!(f, "bseti"),
        }
    }
}

impl Opcode for BOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            Self::BCLR | Self::BEXT | Self::BINV | Self::BSET => InstFormat::RFormat,
            Self::BCLRI | Self::BEXTI | Self::BINVI | Self::BSETI => InstFormat::BExtShamtFormat,
        }
    }
}