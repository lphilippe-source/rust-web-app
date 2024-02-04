# web vanilla Rust

**The intention of this project is to handle inner particularities of Rust language and its ecosystem such as null with Option, conditional return type with Enum and so on.
What you can find here: handling entity, pattern matching, error handling, logs, modules.
Work is based on a simple web app page which can be found here:**
[GitHub FrancesoXX](https://github.com/FrancescoXX/fullstack-rust-nextjs)



## prerequisites

---
- Rust
- Cargo
- docker

## how to run

---
1- ***fastest way*** 

<sub> comment the service rustapp in compose.yml and run db with:</sub> 

> `docker-compose up`
 
<sub> then run the rust app with:</sub>

> `cargo run`
---

2- **build and release with docker**

> `docker-compose up`
