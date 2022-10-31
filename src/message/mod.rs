mod login;
mod show_endpoints;
pub use login::Login;
pub use show_endpoints::ShowEndpoints;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Message<T>
where
    T: MessageDetails + Debug,
{
    Action(T),
    Response(T),
    Event(T),
}

impl<T> Message<T>
where
    T: MessageDetails + Debug,
{
    pub fn as_bytes(&self) -> Vec<u8> {
        let msg = match self {
            Message::Action(m) | Message::Response(m) | Message::Event(m) => m,
        };
        //TODO: Normalize the number of newlines before we send the message. We want to terminate with exactly 2
        format!("Action: {}\n{}\n", msg.get_name(), msg.get_body()).into_bytes()
    }
}
//MessageDetails represents information about a message
// so that we can format it before sending, or after receiving it
pub trait MessageDetails {
    fn get_name(&self) -> String;
    fn get_body(&self) -> String;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_login_as_bytes() {
        let login = Login {
            username: "admin".to_owned(),
            secret: "2f35413cc35f27e96b1cd19c6930e4a4".to_owned(),
        };

        let msg = Message::Action(login);
        let bytes = msg.as_bytes();

        assert_eq!(
            bytes,
            vec![
                65, 99, 116, 105, 111, 110, 58, 32, 108, 111, 103, 105, 110, 10, 85, 115, 101, 114,
                110, 97, 109, 101, 58, 32, 97, 100, 109, 105, 110, 10, 83, 101, 99, 114, 101, 116,
                58, 32, 50, 102, 51, 53, 52, 49, 51, 99, 99, 51, 53, 102, 50, 55, 101, 57, 54, 98,
                49, 99, 100, 49, 57, 99, 54, 57, 51, 48, 101, 52, 97, 52, 10, 10
            ]
        );
    }

    #[test]
    fn login_message_get_body() {
        let login = Login {
            username: "admin".to_owned(),
            secret: "2f35413cc35f27e96b1cd19c6930e4a4".to_owned(),
        };

        let body = login.get_body();
        assert_eq!(
            body,
            "Username: admin\nSecret: 2f35413cc35f27e96b1cd19c6930e4a4\n"
        )
    }
}
