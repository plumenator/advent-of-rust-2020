/*

--- Day 21: Allergen Assessment ---

You reach the train's last stop and the closest you can get to your
vacation island without getting wet. There aren't even any boats here,
but nothing can stop you now: you build a raft. You just need a few
days' worth of food for your journey.

You don't speak the local language, so you can't read any ingredients
lists. However, sometimes, allergens are listed in a language you do
understand. You should be able to use this information to determine
which ingredient contains which allergen and work out which foods are
safe to take with you on your trip.

You start by compiling a list of foods (your puzzle input), one food
per line. Each line includes that food's ingredients list followed by
some or all of the allergens the food contains.

Each allergen is found in exactly one ingredient. Each ingredient
contains zero or one allergen. Allergens aren't always marked; when
they're listed (as in (contains nuts, shellfish) after an ingredients
list), the ingredient that contains each listed allergen will be
somewhere in the corresponding ingredients list. However, even if an
allergen isn't listed, the ingredient that contains that allergen
could still be present: maybe they forgot to label it, or maybe it was
labeled in a language you don't know.

For example, consider the following list of foods:

mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)

The first food in the list has four ingredients (written in a language
you don't understand): mxmxvkd, kfcds, sqjhc, and nhms. While the food
might contain other allergens, a few allergens the food definitely
contains are listed afterward: dairy and fish.

The first step is to determine which ingredients can't possibly
contain any of the allergens in any food in your list. In the above
example, none of the ingredients kfcds, nhms, sbzzf, or trh can
contain an allergen. Counting the number of times any of these
ingredients appear in any ingredients list produces 5: they all appear
once each except sbzzf, which appears twice.

Determine which ingredients cannot possibly contain any of the
allergens in your list. How many times do any of those ingredients
appear?

 */

use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl std::str::FromStr for Food {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split('(').collect();

        let (ingredients_str, allergens_str) = (fields[0], fields[1]);
        let ingredient_strs = ingredients_str.split_whitespace();
        let mut ingredients = Vec::new();
        for ingredient_str in ingredient_strs {
            ingredients.push(ingredient_str.to_string());
        }
        let allergen_strs = allergens_str.split_whitespace().skip(1);
        let mut allergens = Vec::new();
        for allergen_str in allergen_strs {
            allergens.push(allergen_str[..allergen_str.len() - 1].to_owned());
        }
        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

pub fn part1() -> usize {
    let path = Path::new("day21-input.txt");
    let input = std::fs::read_to_string(path).expect("read");
    let mut all_ingredients = HashSet::new();
    let mut ingredient_counts = HashMap::new();
    let mut allergenic_ingredients = HashMap::new();
    for Food {
        ingredients,
        allergens,
    } in input.as_str().lines().map(|l| l.parse().expect("parse"))
    {
        let ingredients: HashSet<_> = ingredients.into_iter().collect();
        for ingredient in &ingredients {
            *ingredient_counts.entry(ingredient.clone()).or_insert(1) += 1;
        }
        all_ingredients.extend(ingredients.clone());
        for allergen in allergens {
            let allergenic = allergenic_ingredients
                .entry(allergen)
                .or_insert_with(|| ingredients.clone());
            *allergenic = allergenic.intersection(&ingredients).cloned().collect();
        }
    }
    let mut safe_ingredients = all_ingredients.clone();
    for (_, allergenic) in allergenic_ingredients {
        safe_ingredients = safe_ingredients.difference(&allergenic).cloned().collect();
    }
    safe_ingredients
        .iter()
        .map(|i| ingredient_counts.get(i).expect("count") - 1)
        .sum()
}

/*

--- Part Two ---
Now that you've isolated the inert ingredients, you should have enough information to figure out which ingredient contains which allergen.

In the above example:

mxmxvkd contains dairy.
sqjhc contains fish.
fvjkl contains soy.
Arrange the ingredients alphabetically by their allergen and separate them by commas to produce your canonical dangerous ingredient list. (There should not be any spaces in your canonical dangerous ingredient list.) In the above example, this would be mxmxvkd,sqjhc,fvjkl.

Time to stock your raft with supplies. What is your canonical dangerous ingredient list?

 */

pub fn part2() -> String {
    let path = Path::new("day21-input.txt");
    let input = std::fs::read_to_string(path).expect("read");
    let mut all_ingredients = HashSet::new();
    let mut ingredient_counts = HashMap::new();
    let mut allergenic_ingredients = HashMap::new();
    for Food {
        ingredients,
        allergens,
    } in input.as_str().lines().map(|l| l.parse().expect("parse"))
    {
        let ingredients: HashSet<_> = ingredients.into_iter().collect();
        for ingredient in &ingredients {
            *ingredient_counts.entry(ingredient.clone()).or_insert(1) += 1;
        }
        all_ingredients.extend(ingredients.clone());
        for allergen in allergens {
            let allergenic = allergenic_ingredients
                .entry(allergen)
                .or_insert_with(|| ingredients.clone());
            *allergenic = allergenic.intersection(&ingredients).cloned().collect();
        }
    }
    let mut sorted = Vec::new();
    while !allergenic_ingredients.is_empty() {
        for (current_allergen, ingredients) in allergenic_ingredients.clone() {
            if ingredients.len() == 1 {
                allergenic_ingredients.remove(&current_allergen);
                sorted.push((
                    current_allergen,
                    ingredients.into_iter().next().expect("ingredient"),
                ));
            } else {
                let current_ingredients = allergenic_ingredients
                    .get_mut(&current_allergen)
                    .expect("current_ingredient");
                for (_, ingredient) in &sorted {
                    current_ingredients.remove(ingredient);
                }
            }
        }
    }
    sorted.sort_unstable();
    let mut string = sorted
        .iter()
        .cloned()
        .map(|t| t.1)
        .fold(String::new(), |s, arg| s + &arg + ",");
    string.pop();
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2635, part1())
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            "xncgqbcp,frkmp,qhqs,qnhjhn,dhsnxr,rzrktx,ntflq,lgnhmx",
            part2()
        )
    }
}
