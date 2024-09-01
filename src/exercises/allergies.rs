pub struct Allergies {
    score: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

const ALLERGENS: [(Allergen, u32); 8] = [
    (Allergen::Eggs, 1),
    (Allergen::Peanuts, 2),
    (Allergen::Shellfish, 4),
    (Allergen::Strawberries, 8),
    (Allergen::Tomatoes, 16),
    (Allergen::Chocolate, 32),
    (Allergen::Pollen, 64),
    (Allergen::Cats, 128),
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        let all_score = ALLERGENS.iter().fold(0u32, |acc, (_, x)| acc + x);
        let s = if score > all_score {
            score - all_score - 1
        } else {
            score
        };
        Self { score: s }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergies = self.allergies();
        allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let (_, mut result) =
            ALLERGENS
                .iter()
                .rev()
                .fold((self.score, Vec::new()), |acc, (allergen, score)| {
                    let (s, mut res) = acc;

                    if *score <= s {
                        res.push(*allergen);
                        return (s - score, res);
                    }

                    (s, res)
                });

        result.reverse();
        result
    }
}
