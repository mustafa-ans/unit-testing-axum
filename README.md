# Introduction

This project is a Rust-based API that allows users to register, retrieve, update, and delete user data using HTTP requests. It utilizes the Axum framework for routing and interacts with a PostgreSQL database. The purpose of this project is to demonstrate how to build a well-structured, efficient, and secure web application using Rust and to showcase unit testing practices within the Axum framework.

## Installation

To run this application, please follow these steps:

1. Make sure you have Rust installed on your machine. You can install Rust by following the official installation instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started).

2. Create a PostgreSQL database. You can use a service like [Elephant SQL](https://www.elephantsql.com/) or set up a PostgreSQL database locally.

3. Set the `DATABASE_URL` environment variable to the URL of your PostgreSQL database. You can set this environment variable in your system or create a `.env` file in your project directory and define it there.

4. Verify that the required PostgreSQL driver is included in your project's dependencies. You can check the `Cargo.toml` file, which is located in the project's root directory, to ensure that the necessary dependencies are listed.

With these steps completed, you'll be ready to run the application.

Once you have Rust and the environment variable set up, you can clone the repository and run the following command to start the server:

```bash
cargo run
```

The server will be available at http://localhost:3000.

## API Endpoints

- **POST /add-users**

  This endpoint allows users to register by providing their first name, last name, email, and birthdate in the request body. The data is stored in the PostgreSQL database.

- **GET /get-users**

  This endpoint allows users to retrieve their data by providing their username in the request body. The data is retrieved from the PostgreSQL database.

- **PUT /update-users**

  This endpoint allows users to update their user data by providing their username and the fields they want to update in the request body. The data is updated in the PostgreSQL database.

- **DELETE /delete-users**

  This endpoint allows users to delete their account by providing their username in the request body. The user's data is removed from the PostgreSQL database.

## Usage

To use the API, you can send HTTP requests to the respective endpoints using tools like curl, Postman, or any HTTP client. Here are some examples:

**Register a user:**

```http
POST /add-users HTTP/1.1
Host: localhost:3000
Content-Type: application/json

{
    "username": "john_doe",
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "birthdate": "1990-01-15"
}
```

**Retrieve user data:**

```http
GET /get-users HTTP/1.1
Host: localhost:3000
Content-Type: application/json

{
    "username": "john_doe"
}
```

**Update user data:**

```http
PUT /update-users HTTP/1.1
Host: localhost:3000
Content-Type: application/json

{
    "username": "john_doe",
    "first_name": "John",
    "email": "john.doe@updated.com",
    "birthdate": "2020-01-01"
}
```

**Delete a user:**

```http
DELETE /delete-users?username=john_doe HTTP/1.1
Host: localhost:3000
Content-Type: application/json
```

### Project Structure

The project is structured as follows:

    controllers/: contains the endpoint controllers
    models/: contains the data models used in the application
    main.rs: the entry point of the application, where the server is started and the routes are defined.

### Further Improvements

This project can be further improved by implementing additional features such as user authentication, error handling, and input validation.