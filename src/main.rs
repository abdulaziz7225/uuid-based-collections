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

fn main() -> std::io::Result<()> {
    let users_count: u32 = 5;

    let mut user_ids_list: Vec<Uuid> = Vec::new();
    generate_uuids(users_count, &mut user_ids_list);

    let mut users_collection = HashMap::new();
    generate_users(&user_ids_list, &mut users_collection);

    // === Write Users to File ===
    let user_ids_file = File::create("user_ids_list.txt")?;
    let mut users_writer = BufWriter::new(user_ids_file);
    for (index, uuid) in user_ids_list.iter().enumerate() {
        writeln!(users_writer, "UUID {index}: {uuid}")?;
    }

    let users_collection_file = File::create("users_collection.txt")?;
    let mut users_writer = BufWriter::new(users_collection_file);
    for (user_id, user) in &users_collection {
        writeln!(users_writer, "User ID: {user_id}")?;
        writeln!(users_writer, "{user:#?}\n")?;
    }

    // Experiment with jobs
    let jobs_count: u32 = 10;

    let mut job_ids_list: Vec<Uuid> = Vec::new();
    generate_uuids(jobs_count, &mut job_ids_list);

    let mut jobs_collection = HashMap::new();
    generate_jobs(&job_ids_list, &mut jobs_collection);

    // === Write Jobs to File ===
    let job_ids_file = File::create("job_ids_list.txt")?;
    let mut jobs_writer = BufWriter::new(job_ids_file);

    for (index, uuid) in job_ids_list.iter().enumerate() {
        writeln!(jobs_writer, "UUID {index}: {uuid}")?;
    }

    let jobs_collection_file = File::create("jobs_collection.txt")?;
    let mut jobs_writer = BufWriter::new(jobs_collection_file);

    for (job_id, job) in &jobs_collection {
        writeln!(jobs_writer, "Job ID: {job_id}")?;
        writeln!(jobs_writer, "{job:#?}\n")?;
    }
    Ok(())
}
