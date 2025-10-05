pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
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
    pub fn iterator() -> impl Iterator<Item = Allergen> {
        [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
        .iter()
        .copied()
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergens = Vec::new();
        for allergen in Allergen::iterator() {
            // Each allergen corresponds to a power of 2, starting from 1 (2^0) for Eggs
            let allergen_score = 1 << (allergen as u32);
            if score & allergen_score != 0 {
                allergens.push(allergen);
            }
        }
        Allergies { allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
