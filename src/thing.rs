use super::models::schema::school;

#[derive(Queryable, Debug)]
pub struct School {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="school"]
pub struct NewSchool<'a> {
    pub name: &'a str,
}
