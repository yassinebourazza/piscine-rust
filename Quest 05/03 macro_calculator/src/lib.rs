use json::object;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {


    let mut res: (f64,f64,f64,f64) = (0.0,0.0,0.0,0.0);
    for food in foods.iter(){
        res.0 += food.calories.1.replace("kcal","").parse::<f64>().unwrap() * food.nbr_of_portions;
        res.1 += ( food.carbs as f64 ) * food.nbr_of_portions;
        res.2 += ( food.proteins as f64 ) * food.nbr_of_portions;
        res.3 += ( food.fats as f64 )  * food.nbr_of_portions;
    }

    json::object! {
        "cals": (res.0 * 100.0).round() /100.0,
        "carbs": (res.1 * 100.0).round() /100.0,
        "proteins": (res.2 * 100.0).round() /100.0,
        "fats": (res.3 * 100.0).round() /100.0
    }
}