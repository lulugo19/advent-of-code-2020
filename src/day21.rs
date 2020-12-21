use regex::Regex;
use std::collections::{HashMap, HashSet};

type Food = (HashSet<String>, HashSet<String>);

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<Food> {
  let regex = Regex::new(r"contains (\w+(, \w+)*)").unwrap();
  input
    .lines()
    .map(|line| {
      let mut ingredients_allergens = line.split(" (");
      let ingredients = ingredients_allergens
        .next()
        .unwrap()
        .split(" ")
        .map(|ingr| ingr.to_string())
        .collect();
      let allergens = ingredients_allergens
        .next()
        .map(|allergens| {
          regex
            .captures(allergens)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|alg| alg.to_string())
            .collect()
        })
        .unwrap_or_default();
      (ingredients, allergens)
    })
    .collect()
}

#[aoc(day21, part1)]
pub fn solve_part1(food: &[Food]) -> usize {
  let ingredients_with_allergenes = get_ingredients_with_allergenes(food);
  let appear_count: usize = food
    .iter()
    .map(|(ingredients, _)| {
      ingredients
        .iter()
        .filter(|ingr| !ingredients_with_allergenes.contains_key(*ingr))
        .count()
    })
    .sum();

  appear_count
}

#[aoc(day21, part2)]
pub fn solve_part2(food: &[Food]) -> String {
  let mut ingredients_with_allergenes = get_ingredients_with_allergenes(food)
    .into_iter()
    .collect::<Vec<_>>();
  ingredients_with_allergenes.sort_by_key(|(_, allergene)| allergene.clone());

  ingredients_with_allergenes
    .into_iter()
    .map(|(ingr, _)| ingr)
    .collect::<Vec<_>>()
    .join(",")
}

fn get_ingredients_with_allergenes(food: &[Food]) -> HashMap<String, String> {
  // foreach allergen create a set of the possible food
  let mut allergens_to_ingr: HashMap<String, HashSet<String>> = HashMap::new();

  for (ingredients, allergens) in food.iter() {
    for a in allergens.iter() {
      let ingr_set = allergens_to_ingr
        .entry(a.clone())
        .or_insert(ingredients.clone());
      *ingr_set = ingr_set.intersection(&ingredients).cloned().collect();
    }
  }

  let mut determined_ingredients = allergens_to_ingr
    .values()
    .filter_map(|ingr_set| {
      if ingr_set.len() == 1 {
        Some(ingr_set.iter().next().unwrap().clone())
      } else {
        None
      }
    })
    .collect::<HashSet<String>>();

  loop {
    let mut next_determined_ingredients = HashSet::<String>::new();
    for ingr in determined_ingredients.iter() {
      for ingr_set in allergens_to_ingr.values_mut() {
        if ingr_set.len() == 1 {
          continue;
        }
        ingr_set.remove(ingr);
        if ingr_set.len() == 1 {
          next_determined_ingredients.insert(ingr_set.iter().next().unwrap().clone());
        }
      }
    }

    if next_determined_ingredients.len() == 0 {
      break;
    }

    determined_ingredients = next_determined_ingredients;
  }

  return allergens_to_ingr
    .into_iter()
    .map(|(allergene, ingr_s)| (ingr_s.into_iter().next().unwrap(), allergene))
    .collect();
}

#[cfg(test)]
mod test {
  use super::{input_generator, solve_part1, solve_part2};

  const TEST_INPUT: &'static str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
	trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
	sqjhc fvjkl (contains soy)\n\
	sqjhc mxmxvkd sbzzf (contains fish)";

  #[test]
  fn test_day21_part1() {
    let food = input_generator(TEST_INPUT);
    assert_eq!(solve_part1(&food), 5);
  }

  #[test]
  fn test_day21_part2() {
    let food = input_generator(TEST_INPUT);
    assert_eq!(solve_part2(&food), String::from("mxmxvkd,sqjhc,fvjkl"));
  }
}
