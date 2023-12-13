use firebase_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

//////////////
// Set_User //
//////////////
#[tokio::main]
async fn set_user(firebase: &Firebase, user: &User) -> Response {
    let firebase = firebase.at("users"); // <<users>> :: collections
    let _user = firebase.set::<User>(&user).await;
    return string_to_response(&_user.unwrap().data); // <<.data>> :: inner data field or property within that structure.
}

//////////////
// Get_User //
//////////////
#[tokio::main]
async fn get_user(firebase: &Firebase, name: &str) -> User {
    let firebase = firebase.at("users").at(&name); // <<users>> :: collections
    let _user = firebase.get::<User>().await;
    return _user.unwrap();
}

//////////////
// Get_Users //
//////////////
#[tokio::main]
async fn get_users(firebase: &Firebase) -> HashMap<String, User> {
    let firebase = firebase.at("users"); // <<users>> :: collections
    // The values wiil be returned in from of "HashMap" with `string key` and `user values`
    let _users = firebase.get::<HashMap<String, User>>().await; 
    println!("{:?}", _users);
    return _users.unwrap();
}

/////////////////
// Update_User //
/////////////////
#[tokio::main]
async fn update_user(firebase: &Firebase, name:&str, user: &User) -> User {
    // <<users>> :: collections <<&name>> :: to search a particular data
    let firebase = firebase.at("users").at(&name); 
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

/////////////////
// Delete_User //
/////////////////
#[tokio::main]
async fn delete_user(firebase: &Firebase, name: String) -> Response {
    let firebase = firebase.at("users").at(&name);
    let result = firebase.delete::<>().await;
    return string_to_response(&result.unwrap().data);
}


#[tokio::main]
async fn main() {
    let  user = User {
        name: "PsychoPunkSage".to_string(),
        age: 20,
        email: "ap@gmail.com".to_string(),
    };

    // Establish Connection with FIREBASE
    let firebase = Firebase::new("https://realtime-db-edd53-default-rtdb.firebaseio.com/").unwrap();

    let response = set_user(&firebase, &user);
    let mut user1 = get_user(&firebase, &response.name);
    let users = get_users(&firebase);
    println!("{:?}", users);
    
    user1.email = "updated@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user1);
    
    println!("{:?}", updated_user);
    
    delete_user(&firebase, response.name);
    println!("User Deleted");
}

/////////////////
// Helper func //
/////////////////

//DESERIALISE//
// String -> Response
fn string_to_response(s: &str) ->  Response {
    serde_json::from_str(s).unwrap()
}

// String -> User
fn string_to_user(s: &str) -> User{
    serde_json::from_str(s).unwrap()
}