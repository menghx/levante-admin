

//pub struct CreateUser {
//    pub name: String,
//}
//
//impl Message for CreateUser {
//    type Result = Result<models::User, Error>;
//}
//
//impl Handler<CreateUser> for manager::DbExecutor {
//    type Result = Result<models::User, Error>;
//    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
//        use self::schema::users::dsl::*;
//        let uuid = format!("{}", uuid::Uuid::new_v4());
//        let new_user = models::NewUser {
//            id: &uuid,
//            name: &msg.name,
//        };
//        let conn: &SqliteConnection = &self.0.get().unwrap();
//        diesel::insert_into(users)
//            .values(&new_user)
//            .execute(conn)
//            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;
//        let mut items = users
//            .filter(id.eq(&uuid))
//            .load::<models::User>(conn)
//            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;
//
//        Ok(items.pop().unwrap())
//    }
//}