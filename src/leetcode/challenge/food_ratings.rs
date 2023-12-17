use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Default)]
struct FoodInfo {
    name: String,
    cuisine: usize,
    rating: i32,
}

struct FoodRatings {
    food_indices: HashMap<String, usize>,
    food_infos: Vec<FoodInfo>,
    cuisine_index: HashMap<String, usize>,
    cuisine_info: Vec<BinaryHeap<(i32, usize)>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cuisine_index = HashMap::new();
        let mut food_infos = Vec::with_capacity(foods.len());
        let mut cuisine_info = Vec::new();
        let mut food_indices = HashMap::with_capacity(foods.len());

        for ((food, cuisine), rating) in foods.into_iter().zip(cuisines).zip(ratings) {
            let cuisine = *cuisine_index.entry(cuisine).or_insert_with(|| {
                cuisine_info.push(BinaryHeap::new());
                cuisine_info.len() - 1
            });

            food_infos.push(FoodInfo {
                name: food,
                cuisine,
                rating,
            });
        }

        food_infos.sort_unstable_by(|i1, i2| i1.name.cmp(&i2.name).reverse());

        for (i, info) in food_infos.iter().enumerate() {
            cuisine_info[info.cuisine].push((info.rating, i));
            food_indices.insert(info.name.clone(), i);
        }

        Self {
            cuisine_index,
            food_infos,
            cuisine_info,
            food_indices,
        }
    }

    fn change_rating<A: AsRef<str>>(&mut self, food: A, new_rating: i32) {
        let food_index = self.food_indices[food.as_ref()];
        let info = &mut self.food_infos[food_index];
        info.rating = new_rating;
        self.cuisine_info[info.cuisine].push((new_rating, food_index));
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let h = &mut self.cuisine_info[self.cuisine_index[&cuisine]];
        let food_infos = &mut self.food_infos;
        while h
            .peek()
            .filter(|(rating, i)| food_infos[*i].rating == *rating)
            .is_none()
        {
            h.pop();
        }

        h.peek()
            .map(|(_, i)| food_infos[*i].name.clone())
            .unwrap_or_default()
    }
}
