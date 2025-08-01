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
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut pro = 0.0;
    let mut fats = 0.0;

    for i in foods {
        let cal = cnv_cal(i.calories.1.clone());

        cals += cal * i.nbr_of_portions;
        carbs += i.carbs * i.nbr_of_portions;
        pro += i.proteins * i.nbr_of_portions;
        fats += i.fats * i.nbr_of_portions;
    }

    data["cals"] = round_two_decimals(cals).into();
    data["carbs"] = round_two_decimals(carbs).into();
    data["proteins"] = round_two_decimals(pro).into();
    data["fats"] = round_two_decimals(fats).into();
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
fn round_two_decimals(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}
