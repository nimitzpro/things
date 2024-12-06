pub struct Vegetable {
    pub name: String,
    pub mood: String
}

impl Vegetable {
    pub fn amazing(&self) {
        println!("I AM {}. FEAR ME. Mood: {}", self.name, self.mood);
    }
}
