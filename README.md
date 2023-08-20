# WAREHOUSE SERVICE

# How To Run
There are two ways to run this project. You can run it locally or using docker.
Docker is the preferred option because the service runs in an isolated and controlled environment.

# Using Docker (Recommended)
Navigate to the root of the project by entering the following command
```shell
$ cd $PROJECT_PATH
```
Build the docker image, and run the container with:
```shell
$ docker build .
$ docker run
```

Using your preferred client, connect to the running service and make API calls
```shell
$ curl -i -H "Accept: application/json" -H "Content-Type: application/json" -X GET http://localhost/8000/url
```

## Running locally (Not Recommended)
You would need to install the rust compiler

Install rust on Unix systems (MacOS, Linux) https://www.rust-lang.org/learn/get-started

Install rust on Windows: https://forge.rust-lang.org/infra/other-installation-methods.html

The rust compiler installs the cargo package manager for rust and its dependencies

After installation, navigate to the root of the project by entering the following command
```shell
$ cd $PROJECT_PATH
```

Build the project with `cargo`.
Building the project creates a binary for easy shipping or testing on other machines/platforms.
```shell
$ cargo build
```
Then run the built executable on your machine
```shell
$ ./warehouse_service
```

Alternatively, you can build and run the executable in one line with
```shell
$ cargo run
```
Using your preferred client, connect to the running service and make API calls
```shell
$ curl -i -H "Accept: application/json" -H "Content-Type: application/json" -X GET http://localhost/8000/url

```