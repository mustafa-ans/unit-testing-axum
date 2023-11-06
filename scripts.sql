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



-- Query to add mock data to the table
INSERT INTO Users (ID, Username, FirstName, LastName, Email, BirthDate, CreatedAt, UpdatedAt)
VALUES
    (1, 'johndoe87', 'John', 'Doe', 'john.doe@example.com', '1987-03-15', '2023-11-01 08:00:00', '2023-11-01 08:00:00'),
    (2, 'alicesmith92', 'Alice', 'Smith', 'alice.smith@example.com', '1992-07-22', '2023-11-01 09:15:00', '2023-11-01 09:15:00'),
    (3, 'robertjames78', 'Robert', 'James', 'robert.james@example.com', '1978-12-10', '2023-11-01 10:30:00', '2023-11-01 10:30:00'),
    (4, 'sarahwilson85', 'Sarah', 'Wilson', 'sarah.wilson@example.com', '1985-05-05', '2023-11-01 11:45:00', '2023-11-01 11:45:00'),
    (5, 'michaelbrown89', 'Michael', 'Brown', 'michael.brown@example.com', '1989-09-18', '2023-11-01 12:00:00', '2023-11-01 12:00:00');
