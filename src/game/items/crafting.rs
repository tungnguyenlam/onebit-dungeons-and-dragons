/// Crafting system - recipes and crafting logic.
use crate::data::types::{RecipeDef, RecipeIngredient};
use crate::game::items::inventory::Inventory;
use std::collections::HashMap;

pub struct CraftingSystem {
    pub recipes: HashMap<String, RecipeDef>,
}

impl CraftingSystem {
    pub fn new(recipes: HashMap<String, RecipeDef>) -> Self {
        Self { recipes }
    }

    pub fn can_craft(&self, recipe_id: &str, inventory: &Inventory) -> bool {
        let Some(recipe) = self.recipes.get(recipe_id) else {
            return false;
        };
        self.has_ingredients(&recipe.ingredients, inventory)
    }

    fn has_ingredients(&self, ingredients: &[RecipeIngredient], inventory: &Inventory) -> bool {
        ingredients
            .iter()
            .all(|ing| inventory.count(&ing.item_id) >= ing.quantity)
    }

    pub fn craft(&self, recipe_id: &str, inventory: &mut Inventory) -> Option<String> {
        let Some(recipe) = self.recipes.get(recipe_id) else {
            return None;
        };

        if !self.has_ingredients(&recipe.ingredients, inventory) {
            return None;
        }

        for ing in &recipe.ingredients {
            inventory.remove(&ing.item_id, ing.quantity);
        }

        inventory.add(&recipe.result_item, recipe.result_quantity);

        Some(recipe.result_item.clone())
    }

    pub fn get_available_recipes(&self, inventory: &Inventory) -> Vec<&RecipeDef> {
        self.recipes
            .values()
            .filter(|r| self.has_ingredients(&r.ingredients, inventory))
            .collect()
    }

    pub fn get_all_recipes(&self) -> Vec<&RecipeDef> {
        self.recipes.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::{RecipeDef, RecipeIngredient};

    fn create_test_recipes() -> HashMap<String, RecipeDef> {
        let mut recipes = HashMap::new();

        let recipe = RecipeDef {
            id: "test_recipe".to_string(),
            name: "Test Item".to_string(),
            description: "A test recipe".to_string(),
            result_item: "test_item".to_string(),
            result_quantity: 1,
            ingredients: vec![
                RecipeIngredient {
                    item_id: "ingredient_a".to_string(),
                    quantity: 2,
                },
                RecipeIngredient {
                    item_id: "ingredient_b".to_string(),
                    quantity: 1,
                },
            ],
            skill_check: None,
        };

        recipes.insert("test_recipe".to_string(), recipe);
        recipes
    }

    #[test]
    fn can_craft_with_sufficient_ingredients() {
        let recipes = create_test_recipes();
        let system = CraftingSystem::new(recipes);
        let mut inventory = Inventory::default();
        inventory.add("ingredient_a", 2);
        inventory.add("ingredient_b", 1);

        assert!(system.can_craft("test_recipe", &inventory));
    }

    #[test]
    fn cannot_craft_without_sufficient_ingredients() {
        let recipes = create_test_recipes();
        let system = CraftingSystem::new(recipes);
        let mut inventory = Inventory::default();
        inventory.add("ingredient_a", 1);
        inventory.add("ingredient_b", 1);

        assert!(!system.can_craft("test_recipe", &inventory));
    }

    #[test]
    fn craft_removes_ingredients_and_adds_result() {
        let recipes = create_test_recipes();
        let system = CraftingSystem::new(recipes);
        let mut inventory = Inventory::default();
        inventory.add("ingredient_a", 2);
        inventory.add("ingredient_b", 1);

        let result = system.craft("test_recipe", &mut inventory);

        assert_eq!(result, Some("test_item".to_string()));
        assert_eq!(inventory.count("ingredient_a"), 0);
        assert_eq!(inventory.count("ingredient_b"), 0);
        assert_eq!(inventory.count("test_item"), 1);
    }
}
