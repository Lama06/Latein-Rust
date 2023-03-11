#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Kasus {
    Nominativ,
    Genitiv,
    Dativ,
    Akkusativ,
    Ablativ,
    Vokativ,
}

impl Kasus {
    pub const ALLE: [Self; 6] = [
        Self::Nominativ,
        Self::Genitiv,
        Self::Dativ,
        Self::Akkusativ,
        Self::Ablativ,
        Self::Vokativ,
    ];
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Numerus {
    Singular,
    Plural,
}

impl Numerus {
    pub const ALLE: [Self; 2] = [Self::Singular, Self::Plural];
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Genus {
    Maskulinum,
    Femininum,
    Neutrum,
}

impl Genus {
    pub const ALLE: [Self; 3] = [Self::Maskulinum, Self::Femininum, Self::Neutrum];

    pub fn get_letter(self) -> char {
        match self {
            Self::Maskulinum => 'm',
            Self::Femininum => 'f',
            Self::Neutrum => 'n',
        }
    }
}
