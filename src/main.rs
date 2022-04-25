
fn main() {
    let user1 = User {
        email : String::from("meinnameistvater@gmail.com"),
        username : String::from("PresentWH"),
        active : true,
        sign_in_count : 1,
    };

    let user2 = User {
        active : user1.active,
        username : user1.username,
        email : String::from("1603554587@qq.com"),
        sign_in_count : user1.sign_in_count,
    };

    let user3 = User {
        email : String::from("Xavier72bit@wjx.com"),
        username : String::from("WJX"),
        ..user1
    };
    
    println!("User2: ");
    println!("{}",user2.username);
    println!("{}",user2.email);
    println!("User3: ");
    println!("{}",user3.username);
    println!("{}",user3.email);

}

struct User {
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64,
}