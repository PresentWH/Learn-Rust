fn main() {
    let mut user1 = User {
        email: String::from("meinnameistvater@gmail.com"),
        username: String::from("PresentWH"),
        active: true,
        sign_in_count: 1,
    };
    show_user_information(&user1);
    println!("Now change the User to Xavier72bit: ");
    change_user(&mut user1,String::from("Xavier72bit"),String::from("shabi@wjx.com"),1);
    show_user_information(&user1);
    
}

struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

fn change_user_name(user:&mut User,new_name:String) {
    user.username = new_name;
}

fn change_user_email(user:&mut User,new_emial:String) {
    user.email = new_emial;
}

fn change_user_sign(user:&mut User,new_count:u64) {
    user.sign_in_count = new_count;
}

fn change_user(user:&mut User,name:String,email:String,count:u64) {
    change_user_name(user,name);
    change_user_email(user,email);
    change_user_sign(user,count);
}

fn show_user_information(user:&User) {
    println!("User: {}",user.username);
    println!("Is active: {}",user.active);
    println!("sign in count: {}",user.sign_in_count);
    println!("email: {}",user.email);
}