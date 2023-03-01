# Reading-for-Leisure: Kubernetes based Continuous Delivery
## Project objectives
1. Create a customized Docker container
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
- Try the URL: https://liusuen-miniature-giggle-59w4rr4xwvgc7xv4-8080.preview.app.github.dev/
//如何能够随时都能launch
- Usage
  - 1. `/`: return "Hello, have something to read!"
  - 2. `/book`: return a random book name to the user
  - 3. `/version`: return the version of the service
### 
  
## Deploy to automatic deployment platform
### 01. AWS APP Runner
1. On AWS Could9, create a new environment
- AWS Could9: https://us-east-1.console.aws.amazon.com/cloud9control/home?region=us-east-1#/product
- Go to AWS Could9, and click "Create Environment" 
- In the teminal
  ```
  git clone
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
2. Create a private repository on AWS Elastic Container
- AWS ECR: https://us-east-1.console.aws.amazon.com/ecr/home?region=us-east-1
- Go to AWS Elastic Container Registry, and click "Get Started"
- Follow the steps to create a private repository
- Go to "Images", click "View push commands" and you can see the push commands,  
then copy and run the code on AWS Cloud9
```
//This is just an example of push commands, the AWS ECR will show the push commands automatically

```
- Tips: sometimes you may meet an error when building docker, you can tru:
  ```
  curl -s https://gist.githubusercontent.com/wongcyrus/a4e726b961260395efa7811cab0b4516/raw/6a045f51acb2338bb2149024a28621db2abfcaab/resize.sh | bash /dev/stdin 60
  ```
3. Create a new service on AWS App Runner
- AWS App Runner: 
- Go to AWS App Runner, and clikc "Create Service"
- Follow the steps to deploy on the App Runner
- Make sure the event status is "Succeed", otherwise you need to fix the problems.
Now, you have successfully created a customized Docker container, pushed it to cloud based ECR, and deployed the project to Kubernetes cluster   
### 02. DockerHub and Minikube
1. Push container to DockerHub
  ```
  docker login -u
  docker build . -t 
  docker push 
  ```
2. Run Minikube
  ```
  minikube start //start cluster
  minikube dashboard --url //view dashboard in a new terminal
  //go to 'Ports' to find 36775, open the link and add 'api' in the end of the link
  //follow link
  kubectl create deployment hello-minikube --image=registry.hub.docker.com/liusuen/reading-for-leisure //create a deployment
  kubectl get deployments //view deployment
  kubectl get pods //view pods
  kubectl expose deployment hello-minikube --type=LoadBalancer --port=8080//create service and expose it
  //view services
  kubectl get services hello-minikube
  minikube service hello-minikube --url
  curl http://192.168.49.2:31839 //Curl the URL shown
  ```
  Curl the URL shown, for example `curl http://192.168.49.2:31839` or use your own URL
  clean up
  ```
  kubectl delete service hello-node
  kubectl delete deployment hello-node
  minikube stop
  ```
## Reference
**1. Kubernetes**
- Introduction: Kubernetes is a portable, extensible, open source platform for managing containerized workloads and services, that facilitates both declarative configuration and automation.
  - 1. Official Website: https://kubernetes.io/docs
  - 2. Get Started with K8s: https://kubernetes.io/blog/2019/07/23/get-started-with-kubernetes-using-python/
  - 3. Kubernetes example - Professor's tutorial: coursera-applied-de-kubernetes-lab, https://github.com/nogibjj/coursera-applied-de-kubernetes-lab
  - 4. Hello Minikube: https://kubernetes.io/docs/tutorials/hello-minikube/
**2. Docker**
- Introduction:
- 1. Official Website: https://hub.docker.com/
- 2. FastAPI Docker docs: https://github.com/tiangolo/uvicorn-gunicorn-fastapi-docker
**3.Other resources**
- Thanks to Jiaxin for her project and detailed ducumentation: https://github.com/nogibjj/Jiaxin-P2-Rust-Minikube
