# Introduction

This project is a simple Rust-based API that allows users to register and retrieve user data using HTTP requests. It uses the Axum framework for routing and PostgreSQL as the database, specifically the Elephant SQL cloud database. The purpose of this project is to showcase my ability to build a well-structured, efficient, and secure web application using Rust.
Installation

To run this application, you will need Rust installed on your machine. You will also need to create a PostgreSQL database on Elephant SQL and set the DATABASE_URL environment variable to the URL of your database.

Once you have Rust and the environment variable set up, you can clone the repository and run the following command to start the server:


cargo run

The server will be available on http://localhost:3000.

### API Endpoints
POST /insert

This endpoint allows users to register by providing their first name and last name in the request body. The data is stored in the PostgreSQL database.
GET /user

This endpoint allows users to retrieve their data by providing their first name in the request body. The data is retrieved from the PostgreSQL database.
Usage

To use the API, you can send HTTP requests to the respective endpoints using a tool like curl or Postman. Here are some examples:
Register a user


POST /insert HTTP/1.1
Host: localhost:3000
Content-Type: application/json

`{
    "first_name": "John",
    "last_name": "Doe"
}`

### Retrieve user data



GET /user HTTP/1.1
Host: localhost:3000
Content-Type: application/json

`{
    "first_name": "John"
}`

### Project Structure

The project is structured as follows:

    controllers/: contains the endpoint controllers
    models/: contains the data models used in the application
    main.rs: the entry point of the application, where the server is started and the routes are defined.

### Further Improvements

This project can be further improved by implementing additional features such as user authentication, error handling, and input validation. Additionally, more tests can be added to ensure the reliability and stability of the application
