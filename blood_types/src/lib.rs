use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen, rh_factor) = match s {
            "A+" => (Antigen::A, RhFactor::Positive),
            "A-" => (Antigen::A, RhFactor::Negative),
            "B+" => (Antigen::B, RhFactor::Positive),
            "B-" => (Antigen::B, RhFactor::Negative),
            "AB+" => (Antigen::AB, RhFactor::Positive),
            "AB-" => (Antigen::AB, RhFactor::Negative),
            "O+" => (Antigen::O, RhFactor::Positive),
            "O-" => (Antigen::O, RhFactor::Negative),
            _ => return Err(()),
        };

        Ok(BloodType {
            antigen,
            rh_factor,
        })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };

        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {

        if other.rh_factor == RhFactor::Positive && self.rh_factor == RhFactor::Negative {
            return false
        }

        match self.antigen {
            Antigen::O => matches!(other.antigen, Antigen::O),
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::AB => true,
        }

    }

    pub fn donors(self) -> Vec<Self> {
        ALL.into_iter().filter(|&b| self.can_receive_from(b)).collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        ALL.into_iter().filter(|&b| b.can_receive_from(self)).collect()
    }

    
}

const ALL: [BloodType; 8] = [
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
        ];