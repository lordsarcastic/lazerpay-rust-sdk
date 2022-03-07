/// The customer information
#[derive(Default, Debug)]
pub struct Customer {
    pub name: String,
    pub email: String,
}

impl Customer {
    /// This function creates a new instance of Customer.
    pub fn new<S: Into<String>>(name: S, email: S) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
        }
    }
}
