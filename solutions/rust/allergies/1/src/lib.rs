pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    fn value(&self) -> u32 {
        match self {
            Self::Eggs => 1,
            Self::Peanuts => 2,
            Self::Shellfish => 4,
            Self::Strawberries => 8,
            Self::Tomatoes => 16,
            Self::Chocolate => 32,
            Self::Pollen => 64,
            Self::Cats => 128,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & allergen.value() != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        const ALLERGENES: [Allergen; 8] = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        ALLERGENES
            .iter()
            .filter(|&a| self.is_allergic_to(a))
            .cloned()
            .collect()
    }
}
