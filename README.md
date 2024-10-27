Prerequisites
Rust installed
MySQL database setup
.env file created for environment configuration
Project Setup
Clone the repository:

bash
Copy code
git clone <repository_url>
cd actix-web-user-management
Set up environment variables:

Create a .env file in the root directory of the project and add the following environment variables:

dotenv
Copy code
ADDRESS=127.0.0.1
PORT=8080
DATABASE_URL=mysql://username:password@host/database_name
Replace the values accordingly:

username: MySQL database username
password: MySQL database password
host: MySQL server host (e.g., localhost)
database_name: Name of the MySQL database
Database Setup and Migrations:

Once your database is set up and connected, run the following command to apply migrations to the database:

bash
Copy code
```
cargo run --up
```
This will ensure all necessary tables and initial data structures are created in the database.

Run the Application Locally:

To start the server, use:

bash
Copy code
```
cargo run
```

The API will be accessible at http://127.0.0.1:8080.

# API Endpoints

## User Routes

| Method | Endpoint         | Description              |
|--------|------------------|--------------------------|
| GET    | /getUsers        | Fetch all users          |
| POST   | /createUser      | Create a new user        |
| GET    | /getUser/{id}    | Fetch User with that Id  |
| DELETE | /deleteUser/{id} | Delete User with that Id |


## Test Routes

| Method | Endpoint       | Description                  |
|--------|----------------|------------------------------|
| GET    | /test          | Test API route               |
| GET    | /hello/{name}  | Personalized greeting route  |


## Database Setup

Ensure you have a MySQL database configured with the name specified in your `.env` file. You can create a database by running the following MySQL command:

```sql
CREATE DATABASE your_database_name;


sql
Copy code
CREATE DATABASE database_name;
Replace database_name with the name of your choice. Update this in the DATABASE_URL variable in your .env file.

License
This project is licensed under the MIT License.