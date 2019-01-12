// Code goes here
#[macro_use(hashmap)]
extern crate maplit;
pub fn maplit_example() {
    let map = hashmap! {
        "a" => 1,
        "b" => 2,
    };
    println!("map = {:?}", map);
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let v = vec![1, 2, 3, 4, 5];

        //let does_not_exist = &v[100];
        let _does_not_exist2 = v.get(100);
        super::maplit_example();
    }
    #[test]
    fn access_keys() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        //let team_name = String::from("Blue");
        //let score = scores.get(&team_name);
        let _score = scores.get("Blue");
        let _score2 = scores.get("Blue_XXX");
    }
}
