use recipe::Recipe;

pub mod recipe;

trait Item: PartialEq {
    fn name(&self) -> &str;
}

#[derive(PartialEq, Clone)]
struct Sword {
    name: &'static str,
}

impl Item for Sword {
    fn name(&self) -> &str {
        &self.name
    }
}

struct ItemRecipe<T: Item, F: Fn() -> T> {
    name: &'static str,
    ingredients: Vec<T>,
    product_factory: F,
}

impl<T: Item, F: Fn() -> T> ItemRecipe<T, F> {
    pub fn new(name: &'static str, ingredients: Vec<T>, product_factory: F) -> Self {
        Self {
            name,
            ingredients,
            product_factory,
        }
    }
}

impl<T: Item + PartialEq, F: Fn() -> T> Recipe<T, F> for ItemRecipe<T, F> {
    fn ingredients(&self) -> &Vec<T> {
        &self.ingredients
    }

    fn product_factory(&self) -> &F {
        &self.product_factory
    }
}

const RUSTY_SWORD: Sword = Sword {
    name: "Rusty Iron Sword",
};
const BRONZE_SWORD: Sword = Sword {
    name: "Bronze Sword",
};

use once_cell::sync::Lazy;

// Define your recipes as static
static RECIPES: Lazy<Vec<ItemRecipe<Sword, fn() -> Sword>>> = Lazy::new(|| {
    vec![ItemRecipe::new(
        "Enchanted Rusty Sword",
        vec![RUSTY_SWORD.clone()],
        || Sword {
            name: "Enchanted Rusty Sword",
        },
    )]
});

// Use them like this
fn get_recipes() -> &'static Vec<ItemRecipe<Sword, fn() -> Sword>> {
    &RECIPES
}
