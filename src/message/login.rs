use super::*;

#[derive(Debug)]
pub struct Login {
    pub username: String,
    pub secret: String,
}
impl MessageDetails for Login {
    fn get_name(&self) -> String {
        "login".to_string()
    }

    fn get_body(&self) -> String {
        format!("Username: {}\nSecret: {}\n", self.username, self.secret)
    }
}
