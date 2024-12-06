trait Summary {
    fn summary(&self) -> String;
}

enum Genre {
    Pop,
    Rock,
    Jazz,
    Classical,
    SciFi,
    Fantasy
}
impl Genre {
    fn as_str(&self) -> &'static str {
        match self {
            Genre::Pop => "Pop",
            Genre::Rock => "Rock",
            Genre::Jazz => "Jazz",
            Genre::Classical => "Classical",
            Genre::SciFi => "Science Fiction",
            Genre::Fantasy => "Fantasy"
        }
    }
}

struct Song {
    title: String,
    artist: String,
    duration: u32,
    genres: Vec<Genre>
}

impl Summary for Song {
    fn summary(&self) -> String {
        let mut genres: String = Default::default();
        for genre in &self.genres {
            genres.push_str(genre.as_str());
            genres.push_str(", ");
        }
        genres = genres[0..genres.len()-2].into();
        format!("{} - {}, duration: {}, genres: {}", self.artist, self.title, self.duration, genres)
    }
}

struct Movie {
    title: String,
    director: String,
    duration: u32,
    genres: Vec<Genre>
}

impl Summary for Movie {
    fn summary(&self) -> String {
        let mut genres: String = Default::default();
        for genre in &self.genres {
            genres.push_str(genre.as_str());
            genres.push_str(", ");
        }
        genres = genres[0..genres.len()-2].into();
        format!("{} directed by {}, duration: {}, genres: {}", self.title, self.director, self.duration, genres)
    }
}

fn announce(new_item: &impl Summary) {
    println!("New media dropped! [[ {} ]]", new_item.summary());
}

use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    }
    else {
        s2
    }
}

fn main() {
    println!("Hello, world!");

    let s = Song {
        title: "Never Gonna Give You Up".into(),
        artist: "Rick Astley".into(),
        duration: 180,
        genres: vec![Genre::Pop, Genre::Jazz]
    };

    let m = Movie {
        title: "Justice League".into(),
        director: "Zack Snyder".into(),
        duration: 120,
        genres: vec![Genre::SciFi, Genre::Fantasy]
    };

    announce(&s);
    announce(&m);

    let a = Pair { x: 20.0, y: 45.3 };
    a.cmp_display();

    let str1 = "a short string";
    let str2 = "a slightly longer string";

    println!("The longest string is: '{}'", longest(str1, str2));
}
