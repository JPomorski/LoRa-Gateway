#[derive(Debug, Clone)]
pub enum Status {
    Success
}

pub fn get_response_description_by_params(status: Status) -> String {
    match status {
        Status::Success => "Success".to_string()
    }
}