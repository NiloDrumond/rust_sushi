use std::fmt;

#[derive(Eq, PartialEq)]
#[repr(u8)]
pub enum CustomerStatus {
    Arriving,
    InQueue,
    Entered,
    Left,
}

impl fmt::Display for CustomerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomerStatus::Arriving => write!(f, "Chegando"),
            CustomerStatus::InQueue => write!(f, "Na fila"),
            CustomerStatus::Entered => write!(f, "Dentro"),
            CustomerStatus::Left => write!(f, "Saiu"),
        }
    }
}

impl TryFrom<u8> for CustomerStatus {
    type Error = ();

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            0 => Ok(CustomerStatus::Arriving),
            1 => Ok(CustomerStatus::InQueue),
            2 => Ok(CustomerStatus::Entered),
            3 => Ok(CustomerStatus::Left),
            _ => Err(()),
        }
    }
}
