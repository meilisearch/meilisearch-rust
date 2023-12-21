pub mod add_user;

use add_user::AddUser;
use async_graphql::MergedObject;

//Combines user queries into one struct
#[derive(Default, MergedObject)]
pub struct UsersMut(pub AddUser);
