mod jobs;
mod users;

use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use uuid::Uuid;

use fake::Fake;
use fake::faker::{
    address::raw::CountryName, company::raw::Profession, job::raw::*, name::raw::Name,
};
use fake::locales::EN;

use jobs::Job;
use users::User;

fn generate_uuids(count: u32, uuids_list: &mut Vec<Uuid>) {
    for _ in 1..=count {
        let id = Uuid::new_v4();
        uuids_list.push(id);
    }
}

fn generate_users(user_ids_list: &Vec<Uuid>, users_collection: &mut HashMap<Uuid, User>) {
    for id in user_ids_list {
        let name = Name(EN).fake();

        let mut rng = rand::rng();
        let age = rng.random_range(18..=65);

        let profession: String = Profession(EN).fake();
        let country: String = CountryName(EN).fake();

        let user = User::new(*id, name, age, profession, country);
        users_collection.insert(*id, user);
    }
}

fn generate_jobs(job_ids_list: &Vec<Uuid>, jobs_collection: &mut HashMap<Uuid, Job>) {
    for id in job_ids_list {
        let title: String = Title(EN).fake();
        let position: String = Position(EN).fake();

        let seniority: String = Seniority(EN).fake();
        let field: String = Field(EN).fake();

        let job = Job::new(*id, title, position, seniority, field);
        jobs_collection.insert(*id, job);
    }
}

fn writer_results_to_file<T>(
    count: u32,
    generate_collection: fn(&Vec<Uuid>, &mut HashMap<Uuid, T>),
    ids_file_name: &str,
    collection_file_name: &str,
) -> std::io::Result<()>
where
    T: std::fmt::Debug,
{
    let mut ids_list: Vec<Uuid> = Vec::new();
    generate_uuids(count, &mut ids_list);

    let mut collection = HashMap::new();
    generate_collection(&ids_list, &mut collection);

    // Write UUIDs
    let ids_file = File::create(ids_file_name)?;
    let mut ids_writer = BufWriter::new(ids_file);
    for (index, uuid) in ids_list.iter().enumerate() {
        writeln!(ids_writer, "UUID {index}: {uuid}")?;
    }

    let collection_file = File::create(collection_file_name)?;
    let mut collection_writer = BufWriter::new(collection_file);
    for (uuid, collection_instance) in &collection {
        writeln!(collection_writer, "UUID: {uuid}")?;
        writeln!(collection_writer, "{collection_instance:#?}\n")?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    writer_results_to_file(5, generate_users, "user_ids_list.txt", "users_collection.txt")?;
    writer_results_to_file(10, generate_jobs, "job_ids_list.txt", "jobs_collection.txt")?;
    Ok(())
}
