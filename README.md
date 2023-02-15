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
2. `/books`: return a random book name to the user
3. `/version`: return the version of the service
## Structure Diagram
## Steps
### 1. Create new Rust project
- install Rust:
```Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```
- create new project
  - type: `cargo new src` (`src` is the project's name)
  - create `main.rs` and `lib.rs` under the `src` folder
  ```Rust
  main.rs: the main project code
  ```
  - compile the code: `cargo build`
  ```Rust
    cargo build
    - Run under the root directory of the project
    - This is the command in the Rust programming language that is used to compile a Rust project. It compiles the project's source code and its dependencies, and produces an executable binary file.
  ```
- set up `Cargo.toml`, to determine the dependencies and build configuration of the project
- set up Dockerfile
- create a Makefile: a special file that lists a set of rules for compiling a project. These rules include targets, which can be an action make needs to take or the files/objects make will need to build, and the commands that need to be run in order to build that target.

## Run this project
### 1. Format the code and check errors
  ```Rust
  make format
  make lint
  ```
### 2. Run the prohect
  ```Rust
  cargo run
  press Control+C to quit
  ```
### 3. Usage
- Try the URL: 
- Usage
  - 1. `/`: return "Hello, have something to read!"
  - 2. `/books`: return a random book name to the user
  - 3. `/version`: return the version of the service
  
## Deploy to Kubernetes service (to do)
  
