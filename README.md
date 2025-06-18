# Implement UUID-based collections as Hashes in Rust

## Objective

Create a program that generates UUID v4 values and demonstrates their use as unique identifiers or hashes.

### Instructions

1. **Add the UUID Dependency**

   - Open `Cargo.toml` and add the `uuid` crate as a dependency.

2. **Generate UUID v4**

   - Write a program that generates a specified number of UUID v4 values (e.g., 5).
   - Print each generated UUID v4 to the console.

3. **Use UUIDs as Hashes**

   - Create a simple data structure (e.g., a struct representing a user or an item) that includes a UUID v4 field as a unique identifier.
   - Implement a function that creates an instance of this data structure and assigns a newly generated UUID v4 as its identifier.

4. **Store and Retrieve Data**

   - Use a `HashMap` to store multiple instances of your data structure, using the UUID v4 as the key.
   - Write a function to retrieve an instance from the `HashMap` using its UUID v4.

5. **Experiment**
   - Modify the program to generate UUIDs for different types of data (e.g., products, sessions) and demonstrate how they can be used to uniquely identify and retrieve these items.
   - Consider the implications of using UUIDs as hashes in terms of collision resistance and uniqueness.
