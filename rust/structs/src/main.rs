struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    print!("User: {}\nEmail: {}\nActivity: {}, count: {}\n", user.username, user.email, user.active, user.sign_in_count);
}


struct Color(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct Point(i32, i32, i32);

fn calc_midpoint(p1: &Point, p2: &Point) -> Point {
    Point((p1.0+p2.0)/2, (p1.1+p2.1)/2, (p1.2+p2.2)/2)
}

#[derive(Debug)]
struct Box {
    length: u64,
    width: u64,
    height: u64
}

impl Box {
    fn volume(&self) -> u64 {
        self.length * self.width * self.height
    }

    fn can_hold(&self, other: &Box) -> bool {
        self.length > other.length && self.width > other.width && self.height > other.height
    }

}


fn main() {
    let mut user = User {
        active: true,
        username: String::from("nimitz"),
        email: String::from("alex@mail.com"),
        sign_in_count: 1
    };
    user.sign_in_count += 1;

    print_user(&user);

    let mut user2 = build_user(String::from("utkarsh@mail.com"), String::from("utkarsh"));
    print_user(&user2);
    user2.sign_in_count += 10;

    user.active = false;
    print_user(&user);
    print_user(&user2);

    let user3 = User {
        username: String::from("alex backup"),
        email: String::from("alex2@mail.com"),
        ..user
    };

    // if non-primitives are destructed from user, then those fields are invalid unless they are
    // reassigned as the values are moved from user to user3 instead of copied
    // user.email = String::from("alexnew@mail.com");
    print_user(&user3);
    print_user(&user);

    let _black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let mut other_point = Point(25, 0, -12);
    other_point.1 = 250;

    println!("{} | {} | {}", other_point.0, other_point.1, other_point.2);

    let _subject = AlwaysEqual;

    let midpoint = calc_midpoint(&origin, &other_point);
    println!("{} | {} | {}", midpoint.0, midpoint.1, midpoint.2);
    println!("Using debug trait: {:?}", midpoint);

    dbg!(&midpoint); // outputs to stderr
    
    let rect1 = Box {
        length: 20,
        width: 40,
        height: 20
    };

    let rect2 = Box { length: 10, width: 20, height: 10 };

    println!("{}", rect1.volume());
    println!("{}", rect1.can_hold(&rect2));
}
