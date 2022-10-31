use super::*;

#[derive(Debug)]
pub struct ShowEndpoints;
impl MessageDetails for ShowEndpoints {
    fn get_name(&self) -> String {
        "PJSIPShowEndpoints".to_string()
    }
    fn get_body(&self) -> String {
        "".to_string()
    }
}
