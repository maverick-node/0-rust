use json::{ JsonValue, object };
#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut data = object! {};

    for i in foods {
        let cal = cnv_cal(i.calories.1.clone());

        let value = cal * i.nbr_of_portions;
        let value_carbs = i.carbs * i.nbr_of_portions;
        let value_pro = i.proteins * i.nbr_of_portions;
        let value_fat = i.fats * i.nbr_of_portions;

        let round_value = (value * 100.0).round() / 100.0;
        let round_carbs = (value_carbs * 100.0).round() / 100.0;
        let round_pro = (value_pro * 100.0).round() / 100.0;
        let round_fat = (value_fat * 100.0).round() / 100.0;

        data["cals"] = round_value.into();
        data["carbs"] = round_carbs.into();
        data["proteins"] = round_pro.into();
        data["fats"] = round_fat.into();
    }

    data
}

fn cnv_cal(s: String) -> f64 {
    let mut res = String::new();
    for i in s.chars() {
        if i.is_alphabetic() {
            break;
        }
        res.push(i);
    }
    res.parse::<f64>().unwrap()
}
