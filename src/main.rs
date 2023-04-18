#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use std::collections::HashMap;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

#[derive(serde::Deserialize, Clone)]
struct Material{
    name: String,
    quantity: u32,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Building {
    building: String,
    workers: u32,
    power: f32,
    water: f32,
    inputs: Vec<HashMap<String, f32>>,
    outputs: Vec<HashMap<String, f32>>,
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let contents = use_future(cx, (), |_| async move {
        reqwest::get("https://raw.githubusercontent.com/lestofante/wrsr-calc/development/building.json")
            .await
            .unwrap()
            .text()
            .await
    }).value();

    if contents.is_none(){
        return cx.render(rsx!(div{"waiting data"}));
    }
    let contents = contents.expect("valid");
    let vec: Vec<Building> = match serde_json::from_str(
        match contents{
            Ok(val) =>val,
            Err(_) => return cx.render(rsx!(div{"no data2"})),
        }) {
        Ok(it) => it,
        Err(err) =>{
            println!("{err}");
            assert!(false);
            return cx.render(rsx!(div{"bad parse"}));
        }
    };
    let ris = vec.iter().map(|item: &Building| {
        rsx!(div{"building name is: {item.building}"})
    });
    
    cx.render(rsx!(div{"data"}, ris))
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs;

    #[test]
    fn test_add() {
        let contents = reqwest::blocking::get("https://raw.githubusercontent.com/lestofante/wrsr-calc/development/building.json").expect("asd").text().expect("dsa");
        //let contents = fs::read_to_string("building.json").expect("Should have been able to read the file");
        println!("got1 {}", contents);
        let vec: Vec<Building> = match serde_json::from_str(&contents) {
            Ok(it) => it,
            Err(err) =>{
                println!("{err}");
                assert!(false);
                return;
            }
        };
        println!("got2 {}", vec.len());
        assert_ne!(vec.len(), 0);
    }
}
