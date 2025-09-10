// Enums

struct User {
    name: String,
    email: String,
    birth: i32,
    active: bool,
    user_role: UserRole,
    website: Website,
}

// Enums = enumeration
enum UserRole {
    BASIC,
    ADMIN,
}

enum Website {
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}


fn main(){

    let mut user = User {
        name: "Julio".to_string(),
        email: String::from("algo@algo.com"),
        birth: 2000,
        active: true,
        user_role: UserRole::BASIC,
        website: Website::INSTAGRAM(String::from("@christer")),
    };

    let access = hasAccess(user.user_role);
    println!("Tiene accesos: {}", access)

}


fn hasAccess( user_role: UserRole) -> bool {
    match user_role {
        UserRole::ADMIN => true,
        UserRole::BASIC => false,

    }
}

// fn goToWebsite( website: Website) {
//     match website {
//         Website::FACEBOOK(),
//         Website::(),
//         Website::FACEBOOK(),
//         Website::FACEBOOK(),
//     }
// }