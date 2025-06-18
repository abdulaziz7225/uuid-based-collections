use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: Uuid,
    name: String,
    age: u32,
    profession: String,
    country: String,
}

impl User {
    pub fn new(id: Uuid, name: String, age: u32, profession: String, country: String) -> User {
        User {
            id,
            name,
            age,
            profession,
            country,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn get_profession(&self) -> &str {
        &self.profession
    }

    pub fn get_country(&self) -> &str {
        &self.country
    }
}