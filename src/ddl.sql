-- Query to create the table used in the API
CREATE TABLE Users (
    ID INT PRIMARY KEY,
    Username VARCHAR(50) NOT NULL,
    FirstName VARCHAR(50),
    LastName VARCHAR(50),
    Email VARCHAR(100) UNIQUE NOT NULL,
    BirthDate DATE,
    CreatedAt TIMESTAMP NOT NULL,
    UpdatedAt TIMESTAMP
);

