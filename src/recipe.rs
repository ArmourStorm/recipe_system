pub trait Recipe<T: PartialEq, F: Fn() -> T> {
    fn ingredients(&self) -> &Vec<T>;
    fn satisfied(&self, available_ingredients: &Vec<T>) -> bool {
        for i in self.ingredients() {
            if !available_ingredients.contains(&i) {
                return false;
            }
        }
        return true;
    }
    fn product_factory(&self) -> &F;
    fn create_product(&self) -> T {
        self.product_factory()()
    }
}
