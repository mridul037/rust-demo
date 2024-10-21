use auth_service::{ database::Credentials,auth};




fn main() {
    let cred = Credentials{
        username:String::from("MRIUDLSHUKLA"),
        password:String::from("password")
    };
    auth(cred);
}
