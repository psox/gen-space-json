use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    env,
    hash::{Hash, Hasher},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).take(1).collect();
    if args.is_empty() {
        panic!("You must specify at least one environment")
    }
    let _ = args
        .iter()
        .inspect(|site| {
            let mut hasher = DefaultHasher::new();
            let mut hm: BTreeMap<String, String> = BTreeMap::new();
            site.hash(&mut hasher);
            let mut hash_str = format!("{:x}", hasher.finish());
            hm.insert(
                "id".into(),
                site.to_string().to_lowercase().replace(".", "-"),
            );
            hm.insert(
                "name".into(),
                site.to_string().to_uppercase().replace(".", "_"),
            );
            hm.insert(
                "description".into(),
                "Site for the ".to_string()
                    + &site.to_string().to_uppercase().replace(".", "/")
                    + &" Environment".to_string(),
            );
            hm.insert(
                "initials".to_string(),
                site.to_uppercase()
                    .get_mut(0..2)
                    .map(|s| &*s)
                    .unwrap()
                    .to_string(),
            );
            hm.insert(
                "color".to_string(),
                "#".to_string() + &hash_str.get_mut(0..6).map(|s| &*s).unwrap().to_string(),
            );
            let j = serde_json::to_string(&hm).unwrap();
            println!("{}", j);
        })
        .collect::<Vec<&String>>();
}
