use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    env,
    hash::{Hash, Hasher},
};

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .take(1)
        .inspect(|ins| {
            let mut site = ins.replace(".", "-").replace("_", "-").replace("/", "-");
            let mut hasher = DefaultHasher::new();
            let mut hm: BTreeMap<String, String> = BTreeMap::new();
            site.hash(&mut hasher);
            hm.insert(
                "color".to_string(),
                "#".to_string()
                    + &format!("{:x}", hasher.finish())
                        .get_mut(0..6)
                        .map(|s| &*s)
                        .unwrap()
                        .to_string(),
            );
            hm.insert("id".to_string(), site.to_string().to_lowercase());
            hm.insert(
                "name".to_string(),
                site.to_string().to_uppercase().replace("-", "_"),
            );
            hm.insert(
                "description".into(),
                "Site for the ".to_string()
                    + &site.to_string().to_uppercase().replace("-", "/")
                    + &" Environment".to_string(),
            );
            site.truncate(2);
            hm.insert("initials".to_string(), site.to_uppercase().to_string());
            let j = serde_json::to_string(&hm).unwrap();
            println!("{}", j);
        })
        .collect();
    if args.is_empty() {
        panic!("You must specify at least one environment")
    }
}
