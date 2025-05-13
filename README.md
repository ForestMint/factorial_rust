# The aim of this program is to know which is faster between calculating factorial and Fibonacci with a loop or recursively

## Run the Rust tests

```sh
$ cargo test
```

## Run the Robot Framework tests

### Prerequisites

Have Robot Framework installed.
If not, run the following command :

```sh
$ pip install robotframework
```

### Compile the program

```sh
$ cargo build
```

### Run the tests

```sh
$ python -m robot test_my_functions.robot
```

## Run the Robot Framework tests using Jenkins

### Prerequisites

```sh
$ apt install -y wget apt-transport-https gpg
$ wget -qO - https://packages.adoptium.net/artifactory/api/gpg
$ wget -O - https://packages.adoptium.net/artifactory/api/gpg/key/public | sudo apt-key add -
$ echo "deb https://packages.adoptium.net/artifactory/deb $(awk -F= '/^VERSION_CODENAME/{print$2}' /etc/os-release) main" | sudo tee /etc/apt/sources.list.d/adoptium.list
$ apt update
$ apt install temurin-17-jdk
```

## Compile the program

```sh
$ cargo build --release
```

## Execute the program

```sh
$ ./target/debug/main
```

## Clean the repository from generated files

```sh
$ ./clean.sh
```