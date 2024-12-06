use std::{collections::HashMap};

fn main() {
	println!("hello world");

	let mut 爆発 = String::from("ヤっはろー");
	println!("{}", 爆発);

	クリア(&mut 爆発);
	if 爆発.len() > 0 {
		panic!("explosion");
	}

	let a = String::from("miiro");
	let b = String::from("海色");
	// b := "みいろ"
	println!("{}", eq(a, b));
}
fn romaji(kana: String) -> String {
	let 平仮名マップ = HashMap::from([
("あ", "a"), ("い", "i"), ("ろ", "ro"), ("み", "mi"), ("海", "mi"), ("色", "iro")]);
    // kanji_map := map[string]*[]string{"海": ["umi", "kai", "mi"]}
	let mut romanised: String = String::from("");
    println!("{:?}", 平仮名マップ);
	for i in kana.chars() {
		println!("got to here: {}", i);
		romanised += 平仮名マップ.get(i.to_string().as_str()).unwrap();
	};

	println!("pain, {}" , romanised);
	return romanised;
}

fn eq(a: String, b: String) -> bool {
	return a == romaji(b);
}

fn クリア(あ: &mut String) {
	*あ = "".to_string();
}
