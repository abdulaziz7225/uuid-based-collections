use uuid::Uuid;

#[derive(Debug)]
pub struct Job {
    id: Uuid,
    title: String,
    position: String,
    seniority: String,
    field: String,
}

impl Job {
    pub fn new(id: Uuid, title: String, position: String, seniority: String, field: String) -> Job {
        Job {
            id,
            title,
            position,
            seniority,
            field,
        }
    }
}
