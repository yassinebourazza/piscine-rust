pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut vector = Vec::new();
    for str in names {
        let coll : Vec<&str> = str.split(" ").collect();
        vector.push(format!("{}. {}.", coll[0].chars().nth(0).unwrap(), coll[1].chars().nth(0).unwrap() ));
    }
    vector
}