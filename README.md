# Reading-for-Leisure: Kubernetes based Continuous Delivery
## Project objectives
1. Create a customized Docker container from the current version of Python that deploys a simple python script.
2. Push image to DockerHub, or Cloud based Container Registery (ECR)
3. Project should deploy automatically to Kubernetes cluster
4. Deployment should be to some form of Kubernetes service (can be hosted like Google Cloud Run or Amazon EKS, etc)
## Introduction
If you want to find some books to read, this project can return a book name for you to have a try.
This actix Microservice has multiple routes:
1. `/`: return "Hello, have something to read!"
2. `/book`: return a random book name to the user
3. `/version`: return the version of the service
## Structure Diagram
## Step 1 Create the Rust project
### 1. Create new Rust project
- install Rust:
```Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```
- create new project
  - type: `cargo new PJ1` (`PJ1` is the project's name)
  ```Rust
  cargo new PJ1
  ```
  - create `main.rs` and `lib.rs` under the `src` folder
  ```Rust
  touch main.rs
  touch lib.rs
  //main.rs: the main project code
  ```
- create a Makefile: a special file that lists a set of rules for compiling a project. These rules include targets, which can be an action make needs to take or the files/objects make will need to build, and the commands that need to be run in order to build that target.
### 2. Compile the code and set up Dockerfile
- Format the code and check errors
  ```Rust
  make format
  make lint
  ```
- compile the code: `cargo build`
  ```Rust
    cargo build
    //- Run under the root directory of the project
    //- This is the command in the Rust programming language that is used to compile a Rust project. 
    //- It compiles the project's source code and its dependencies, and produces an executable binary file.
  ```
- set up `Cargo.toml`, to determine the dependencies and build configuration of the project
```Rust
[package]
name = "find_books"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = "4"
rand = "0.8"
```
- set up `Dockerfile`
```
FROM rust::lastest as builder
ENV APP find_books
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
 
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["find_books"]
```

## Run this project (Microservice)
### 1. Run the project
  ```Rust
  cargo run
  //press Control+C to quit
  ```
### 2. Usage
- Try the URL: //如何能够随时都能launch
- Usage
  - 1. `/`: return "Hello, have something to read!"
  - 2. `/book`: return a random book name to the user
  - 3. `/version`: return the version of the service
### 
  
## Deploy to Kubernetes service (to do)
  
## Reference
## Learn Kubernetes and Docker
### Kubernetes
- Introduction: Kubernetes is a portable, extensible, open source platform for managing containerized workloads and services, that facilitates both declarative configuration and automation.
1.Official Website https://kubernetes.io/docs
2.Kubernetes example: coursera-applied-de-kubernetes-lab, https://github.com/nogibjj/coursera-applied-de-kubernetes-lab
