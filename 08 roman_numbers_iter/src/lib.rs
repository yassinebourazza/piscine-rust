use crate::RomanDigit::*;

impl RomanNumber {
    fn value(d: RomanDigit) -> u32 {
        match d {
            Nulla => 0,
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
        }
    }

    fn to_decimal(&self) -> u32 {
        let digits = &self.0;
        let mut total = 0;
        let mut i = 0;

        while i < digits.len() {
            let current = Self::value(digits[i]);

            if i + 1 < digits.len() {
                let next = Self::value(digits[i + 1]);

                if current < next {
                    total += next - current;
                    i += 2;
                    continue;
                }
            }

            total += current;
            i += 1;
        }

        total
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.to_decimal() + 1;
        *self = RomanNumber::from(value);
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