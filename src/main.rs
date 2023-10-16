use bevy::utils::HashMap;

fn main() {
    // panic!("here");
    let querystring = web_sys::window()
        .expect("expect a window")
        .location()
        .search()
        .ok();
    let query = querystring.map(|s| {
        s.trim_start_matches("?")
            .split("&")
            .map(|keyvalue| {
                let mut it = keyvalue.split("=");
                let key = it.next().unwrap();
                let value = it.next().unwrap();
                (key.to_string(), value.to_string())
            })
            .collect::<HashMap<String, String>>()
    }).expect("should be at least an empty hashmap at this point");

    let Some(example) = query.get("example") else {
        panic!("no example!");
    };
    println!("example is: {example}");
    yt_raymarch_2d::examples(example.clone())
}
