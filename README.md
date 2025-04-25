# Rust Web Marketplace

A full-featured web marketplace built with Rust (Rocket, SeaORM) that supports item listing, user authentication, comments, and admin management tools.

---

## Features

### Item Management
- Users can add, update, delete, and view their items
- Search and filter items by name, category, and price

### Comment System
- Users can comment on items
- Users can edit/delete their own comments
- Admins can remove any inappropriate comments

### Image Upload
- Upload multiple images per item (max 5)
- Images shown on item detail pages

### User Account
- User registration & login/logout
- Password change functionality
- View profile and own listings

### Category Management
- Users assign categories to items
- Admins can add/update/delete categories

### Browsing & Discovery
- Explore items from all users
- Search and filtering for ease of discovery

### Security & Access
- Only item/comment owners can edit/delete
- Admins can moderate items, comments, and users

### Admin Dashboard
- View stats: total users, items, comments
- Monitor recent activities
- Ban or deactivate problematic users

---

## Requirements (User Stories)

### Item Management:
- `user` As a user, I want to be able to add items to the marketplace  
- `user` As a user, I want to update items I have added  
- `user` As a user, I want to delete items I have added  
- `user` As a user, I want to get details of a certain item  
- `user` As a user, I want to search for items by name or category  
- `user` As a user, I want to filter items by price range  
- `user` As a user, I want to view all items I have listed  

### Comment System:
- `user` As a user, I want to be able to comment on items  
- `user` As a user, I want to edit or delete my own comments  
- `admin` As an admin, I want to delete inappropriate comments from any user  

### Image Upload:
- `user` As a user, I want to add multiple images for my items  
- `user` As a user, I want to see item images in the item details view  
- `user` As a user, I want validation to prevent uploading more than 5 images per item  

### User Account:
- `user` As a user, I want to register and log in securely  
- `user` As a user, I want to log out from my account  
- `user` As a user, I want to change my password  
- `user` As a user, I want to see my profile information  

### Category Management:
- `user` As a user, I want to assign categories to my items  
- `admin` As an admin, I want to create, update, and delete categories  
- `admin` As an admin, I want to prevent duplicate category names  

### Browsing & Discovery:
- `user` As a user, I want to browse items from other users  
- `user` As a user, I want to search and filter to find relevant items quickly  

### Security & Access:
- `user` As a user, I want to be the only one who can edit or delete my items/comments  
- `user` As a user, I want my data to be stored securely  
- `admin` As an admin, I want to be able to remove inappropriate content or items  
- `admin` As an admin, I want to have access to all user profiles and items  

### Admin Dashboard:
- `admin` As an admin, I want to view platform statistics (e.g., total users, items, comments)  
- `admin` As an admin, I want to monitor recent activity (e.g., new items, flagged comments)  
- `admin` As an admin, I want to deactivate or ban problematic users  

---

## Tech Stack
- **Rust** — Main backend language
- **Rocket** — Web framework
- **SeaORM** — ORM for database interactions
- **SQLite / PostgreSQL** — Database
- **Tailwind CSS** — Styling (if frontend is included)

---

## Project Structure
```
├── src
│   ├── controllers
│   ├── models
│   ├── services
│   ├── views
│   └── main.rs
├── static
├── templates
└── Cargo.toml
```

---

## Getting Started
```bash
git clone https://github.com/H2Orizon/rust_web_project.git
cd rust_web_project
cargo run
```
