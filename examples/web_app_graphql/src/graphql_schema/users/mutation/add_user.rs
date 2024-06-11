use async_graphql::{Context, InputObject, Object, Result};
use diesel_async::RunQueryDsl;
use validator::Validate;

use crate::{
    models::{NewUser, User},
    validate_input, GraphQlData,
};

#[derive(Default)]
pub struct AddUser;

#[derive(InputObject, Validate)]
pub struct IAddUser {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
}

#[Object]
impl AddUser {
    ///Resolver for creating a new user and storing that data in the database
    ///
    /// The mutation can be run as follows
    /// ```gpl
    /// mutation AddUser{
    ///     users {
    ///         signup(input: {firstName: "",lastName: "",email: ""}){
    ///             id
    ///             firstName
    ///             lastName
    ///             email
    ///         }
    ///     }
    /// }
    pub async fn signup(&self, ctx: &Context<'_>, input: IAddUser) -> Result<User> {
        validate_input(&input)?;

        use crate::schema::users::dsl::users;

        let GraphQlData { pool, .. } = ctx.data().map_err(|e| {
            log::error!("Failed to get app data: {:?}", e);
            e
        })?;

        let mut connection = pool.get().await?;

        let value = NewUser {
            first_name: input.first_name,
            last_name: input.last_name,
            email: input.email,
        };

        let result = diesel::insert_into(users)
            .values(&value)
            .get_result::<User>(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Could not create new user: {:#?}", e);
                e
            })?;

        Ok(result)
    }
}
