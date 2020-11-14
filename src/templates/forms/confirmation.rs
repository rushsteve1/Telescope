use crate::{
    models::confirmations::Confirmation,
    web::Template
};
use crate::templates::forms::common::password::PasswordField;

/// The template for new account confirmations.
/// The user is prompted to input a name and password to seed their account.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUserConf {
    /// The confirmation that spawned this form.
    invite: Confirmation,
    /// The name previously entered into this form if there was one.
    name: Option<String>,
    /// The user's new password.
    pub password: PasswordField,
    /// The password again. Should match the other password field.
    pub confirm_password: PasswordField,
}

impl NewUserConf {
    /// Create a new user confirmation template.
    pub fn new(conf: Confirmation) -> Self {
        Self {
            invite: conf,
            name: None,
            // these last two need to match the format of the form structure in
            // web/services/confirm.rs
            password: PasswordField::new("password"),
            confirm_password: PasswordField::new("confirm-password")
                .map_common(|c| c.name("confirm"))
        }
    }

    /// Builder style function to set the name on this form.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}



impl Template for NewUserConf {
    const TEMPLATE_NAME: &'static str = "forms/confirm/new_user";
}

/// An email confirmed for an existing user.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExistingUserConf {
    /// The invite that spawned this page.
    invite: Confirmation,
    /// An error message if an error occurred.
    error_message: Option<String>,
}

impl ExistingUserConf {
    /// Create a new existing user confirmation page.
    ///
    /// Panics if conf is not for an existing user.
    pub fn new(conf: Confirmation, err: Option<String>) -> Self {
        if conf.creates_user() {
            panic!("Cannot make ExistingUserConfirmation template for new user.")
        }
        Self {
            invite: conf,
            error_message: err
        }
    }
}

impl Template for ExistingUserConf {
    const TEMPLATE_NAME: &'static str = "forms/confirm/existing_user";
}