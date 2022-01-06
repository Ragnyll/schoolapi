use super::schema::school;
use rocket::serde::{Serialize, Deserialize};
use rocket::request::{self, Request};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{Status, ContentType};

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "school"]
pub struct School {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "school"]
pub struct NewSchool<'r> {
    pub name: &'r str,
}

#[derive(Debug)]
pub enum Error {
    TooLarge,
    NoColon,
    Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewSchool<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let json_ct = ContentType::new("application", "json");
        if req.content_type() != Some(&json_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'person' or fallback to default.
        let limit = req.limits().get("school").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);

        // Split the string into two pieces at '='.
        let name = match string.find('=') {
            Some(i) => &string[..i],
            None => return Failure((Status::UnprocessableEntity, NoColon)),
        };

        Success(NewSchool { name })
    }
}
