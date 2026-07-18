use crate::RomanDigit::*;
impl RomanNumber {
    fn to_decimal(&self) -> u32 {
        let mut total = 0;

        for digit in &self.0 {
            total += match digit {
                RomanDigit::Nulla => 0,
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1000,
            };
        }

        total
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.to_decimal();

        *self = RomanNumber::from(value + 1);

        Some(self.clone())
    }
}














#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
	fn from(value: u32) -> Self {
		let mut v = value.clone();
		let mut vec: Vec<RomanDigit> = Vec::new();
		if v ==0 {
			vec.push(Nulla);
			return RomanNumber(vec)
		}
		while v > 0 {
            match v {
                1..=3 => {
                    vec.push(I);
                    v -= 1;
                },
                4 => {
                    vec.push(I);
                    vec.push(V);
                    v -= 4;
                },
                5..=8 => {
                    vec.push(V);
                    v -= 5;
                },
                9 => {
                    vec.push(I);
                    vec.push(X);
                    v -= 9;
                },
                10..=39 => {
                    vec.push(X);
                    v -= 10;
                },
                40..=49 => {
                    vec.push(X);
                    vec.push(L);
                    v -= 40;
                },
                50..=89 => {
                    vec.push(L);
                    v -= 50;
                },
                90..=99 => {
                    vec.push(X);
                    vec.push(C);
                    v -= 90;
                },
                100..=399 => {
                    vec.push(C);
                    v -= 100;
                },
                400..=499 => {
                    vec.push(C);
                    vec.push(D);
                    v -= 400;
                },
                500..=899 => {
                    vec.push(D);
                    v -= 500;
                },
                900..=999 => {
                    vec.push(C);
                    vec.push(M);
                    v -= 900;
                },
                1000.. => {
                    vec.push(M);
                    v -= 1000;
                },
                 0_u32 => todo!()
            }
        }
        
        return RomanNumber(vec)

    }
}