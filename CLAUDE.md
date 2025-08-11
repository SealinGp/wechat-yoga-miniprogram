# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a WeChat Mini Program for yoga class booking, built with a Rust backend server and WebAssembly integration. The application allows users to book yoga classes, view schedules, and manage their profiles through a WeChat Mini Program interface.

## Architecture

### Backend Server (`/server`)
- **Technology**: Rust with Rocket framework
- **Database**: PostgreSQL with direct SQL queries (no stored procedures)
- **ORM**: Using tokio-postgres for database operations
- **Port**: 8002 (configured in Rocket.toml)
- **Main Components**:
  - `handlers/`: API endpoints for different functionalities (booking, admin, user management, membership, location, etc.)
  - `models/`: Data models and structured types for all entities
  - `utils/`: Utility functions (CORS, content disposition, etc.)
  - `errors/`: Custom error handling

### WeChat Mini Program (`/miniprogram`)
- **Framework**: WeChat Mini Program native framework
- **Entry Point**: `app.js` - handles authentication and global state
- **Key Features**:
  - User authentication via WeChat OpenID
  - Class booking system with custom tab bar
  - Admin panel for lesson and user management
  - Sudoku game integration
- **Components**: Reusable UI components in `/components`
- **Pages**: Different screens in `/pages` including admin, booking, user profile

### WebAssembly (`/WebAssembly`)
- **weixin/**: WebAssembly module for the mini program using wasm-bindgen
- **admin/**: Admin-specific WebAssembly functionality
- Built with Rust and compiled to WASM for performance-critical operations

### Deployment (`/Deploy`)
- Rust-based deployment tool for server setup

## Development Commands

### Local Database Setup (Docker)
```bash
# Start PostgreSQL database
docker-compose up -d

# Stop database
docker-compose down

# View database logs
docker-compose logs postgres

# Connect to database
docker exec -it yoga-postgres psql -U psycho -d yoga
```

### Backend Server
```powershell
# Windows development setup
$ENV:DB_HOST="localhost";$ENV:DB_PORT="5432";$ENV:DB_PASSWORD="dev_password";$ENV:APPID="your_appid";$ENV:SECRET="your_secret";$ENV:IMAGE_DIR="./images";cargo run
```

```bash
# Unix/Linux/macOS development setup
export DB_HOST=localhost
export DB_PORT=5432
export DB_PASSWORD=dev_password
export APPID=your_appid
export SECRET=your_secret
export IMAGE_DIR=./images
cargo run
```

Required environment variables:
- `DB_HOST`: Database host (localhost for Docker setup)
- `DB_PORT`: Database port (5432 for Docker setup)
- `DB_PASSWORD`: Database password (dev_password for Docker setup)
- `APPID`: WeChat Mini Program App ID
- `SECRET`: WeChat Mini Program Secret
- `IMAGE_DIR`: Directory for image storage

Copy `.env.example` to `.env` and update with your WeChat Mini Program credentials.

### WebAssembly Build
```powershell
# Build WebAssembly module for mini program
$working="C:\Users\Administrator\Desktop\file\yg\WebAssembly\weixin";$dir="C:\Users\Administrator\Desktop\file\yg\miniprogram\pkg";Set-Location $working;wasm-pack build --target web --out-dir $dir
```

### Mini Program Development
Use WeChat DevTools (Stable Build) to open the project. The mini program configuration is in `project.config.json`.

## Key Configuration Files

- `server/Cargo.toml`: Backend dependencies and configuration
- `server/Rocket.toml`: Web server configuration (address: 127.0.0.1, port: 8002)
- `miniprogram/app.json`: Mini program page routing and tab bar configuration
- `project.config.json`: WeChat DevTools project configuration

## Database Schema

The application uses PostgreSQL with database name "yoga" and user "psycho". The server connects to the database using connection pooling via deadpool-postgres.

**IMPORTANT**: All stored procedures have been removed. The application now uses direct SQL queries with structured models.

### Database Tables

#### Core Tables
- `users`: User profiles with WeChat OpenID authentication
- `admin_users`: Backend admin accounts with username/password authentication  
- `teachers`: Yoga instructor information with ratings and experience
- `locations`: Teaching rooms/venues with capacity, equipment, and facilities
- `lessons`: Class schedules with types, difficulty levels, pricing, and location references
- `bookings`: User class reservations with status tracking

#### Content Management
- `notices`: Announcements and notifications with priority levels
- `posters`: Homepage carousel images with sorting and scheduling
- `action_buttons`: Homepage action buttons configuration
- `market_info`: Marketing/promotional content

#### Membership System
- `membership_plans`: Membership card templates (year cards, count-based cards, etc.)
- `user_membership_cards`: User-owned membership cards with status and expiration
- `membership_card_usage`: Usage tracking for count-based cards with automatic deduction

#### Ratings and Analytics
- `teacher_ratings`: User ratings for teachers with detailed feedback
- `rating_criteria`: Rating standards and weighting
- `debug_logs`: Device information for analytics with IP tracking

### Database Structure Details

#### Location Management (`locations`)
```sql
- id: Primary key
- name: Room name (e.g., "A教室", "空中瑜伽室", "普拉提室")  
- description: Room description
- capacity: Maximum occupancy
- equipment: Array of available equipment
- facilities: Array of facilities (lockers, showers, etc.)
- floor_number, room_number: Physical location
- is_accessible: Wheelchair accessibility
- booking_enabled: Whether room can be booked
- hourly_rate: Rental rate for room booking
- images: Array of room photos
```

#### Enhanced Lessons (`lessons`)
```sql
- id: Primary key
- title, description: Lesson information
- teacher_id: Foreign key to teachers table
- location_id: Foreign key to locations table (replaces venue)
- lesson_type: ENUM ('team', 'small_class', 'private', 'equipment_small_class', 'workshop')
- difficulty_level: ENUM ('beginner', 'intermediate', 'advanced', 'all_levels')
- start_time, end_time: Lesson schedule
- max_students, current_students: Capacity management
- price: Lesson fee
- equipment_required: Array of required equipment
- prerequisites, cancellation_policy, notes: Additional information
```

#### Membership System (`membership_plans`, `user_membership_cards`)
```sql
-- Plans (templates)
- card_type: 'unlimited' or 'count_based'
- validity_days: Card validity period
- total_classes: Number of classes (for count-based cards)
- applicable_lesson_types: Restricted to specific lesson types
- benefits, restrictions: Card terms

-- User Cards (instances)
- card_number: Auto-generated unique identifier
- status: 'active', 'expired', 'suspended', 'used_up'
- remaining_classes: Current balance (for count-based)
- expires_at: Expiration timestamp
- purchase_price, actual_paid: Financial tracking
```

### Key Features
- **Location Management**: Comprehensive room/venue system with capacity, equipment, facilities, and availability checking
- **Membership Cards**: Support for unlimited cards (yearly, half-yearly, quarterly) and count-based cards with automatic usage tracking
- **Teacher Rating System**: 0-5.0 scale ratings with category breakdown
- **Enhanced Course Structure**: Course types, difficulty levels, pricing, and location references
- **Admin Management**: Separate admin user system for backend management
- **Conflict Detection**: Automatic checking of room availability and booking conflicts

The database is automatically initialized with sample data when using the Docker setup via `init.sql`.

## API Endpoints

### Core APIs
The main server routes are mounted at `/` and include:

#### Authentication & Users
- `POST /yoga/auth`: WeChat authentication
- `GET /yoga/user/query?<openid>`: Get user profile
- `POST /yoga/user`: Register/update user
- `GET /yoga/user/book/statistics?<id>`: User booking statistics

#### Booking System  
- `GET /yoga/lessons?<start>&<openid>&<class_type>`: List available lessons
- `GET /yoga/book?<id>&<openid>`: Book a lesson
- `GET /yoga/unbook?<id>&<openid>`: Cancel booking

#### Location Management
- `GET /yoga/locations`: List all locations
- `GET /yoga/locations/available`: List bookable locations
- `GET /yoga/locations/availability?<location_id>&<start_time>&<end_time>`: Check availability
- `GET /yoga/locations/<id>/stats`: Location usage statistics

#### Membership System
- `GET /yoga/membership/plans`: List membership card plans
- `GET /yoga/membership/cards?<openid>`: User's membership cards
- `POST /yoga/membership/purchase`: Purchase membership card
- `GET /yoga/membership/usage?<openid>&<card_id>`: Card usage history

#### Admin Operations
- `GET /yoga/admin/lessons`: Admin lesson management
- `GET /yoga/admin/users/all`: User management
- `POST /yoga/admin/lesson/update`: Update lesson details
- Various other admin endpoints for content management

#### Content Management
- `GET /yoga/index?<openid>`: Homepage data (posters, actions, teachers, notices)
- `GET /yoga/teacher/lessons?<id>`: Teacher's lesson schedule
- `POST /yoga/debug`: Device information logging

## Model Structure

### Handler-Model Mapping
Each handler has corresponding model structures in `/server/src/models/`:

- `admin_lesson.rs`: Admin lesson management models
- `admin_user.rs`: Admin and user management models
- `booking.rs`: Booking system models with location references
- `debug.rs`: Debug logging models
- `index.rs`: Homepage data models (posters, actions, teachers, notices)
- `location.rs`: Location/venue management models with availability checking
- `membership.rs`: Membership card system models
- `teacher.rs`: Teacher and rating models
- `user.rs`: User profile and statistics models

### Key Model Features
- **Type Safety**: All models use Rust type system with SQLx integration
- **Serialization**: Serde support for JSON API responses
- **Database Mapping**: FromRow derives for direct SQL result mapping
- **Validation**: Built-in constraints and type checking
- **Request/Response**: Separate models for API requests and database entities

## Mini Program Global Data

Located in `app.js`:
- `openid`: User's WeChat OpenID for authentication
- `host`: Backend server domain (http://localhost:8002 for development)
- `staticHost`: CDN domain for static assets
- `title`: App display name ("LC PILATES 空中普拉提")

## Development Notes

### Code Organization
- **No Stored Procedures**: All business logic implemented in Rust handlers using direct SQL
- **Structured Queries**: Complex queries broken down into readable, maintainable SQL
- **Error Handling**: Comprehensive error handling with graceful degradation
- **Type Safety**: Leveraging Rust's type system for database operations

### Development Practices  
- **Chinese Documentation**: Project uses Chinese comments and documentation
- **Environment Variables**: All configuration through environment variables
- **Docker Development**: PostgreSQL runs in Docker for consistent development environment
- **Image Processing**: Server-side image handling using Rust libraries
- **Performance**: WebAssembly integration for performance-critical operations

### Testing & Deployment
- **Local Development**: Docker Compose for database, Rust server, WeChat DevTools
- **Environment Setup**: Comprehensive environment variable configuration
- **Build Process**: Separate WebAssembly build step for mini program integration
- **Database Migration**: Automatic initialization through init.sql

## Recent Updates

### Location Management System
- Added comprehensive room/venue management with equipment and facilities tracking
- Integrated location references throughout the lesson and booking system
- Implemented availability checking to prevent booking conflicts
- Created location usage statistics and analytics

### Enhanced Membership System  
- Complete membership card system with unlimited and count-based cards
- Automatic usage tracking and balance management
- Support for multiple cards per user with priority handling
- Integration with booking system for seamless card usage

### Model Restructuring
- Removed all stored procedures in favor of structured Rust models
- Created comprehensive type-safe data models for all entities  
- Implemented proper error handling and validation
- Enhanced API responses with detailed structured data

This architecture provides a solid foundation for a comprehensive yoga studio management system with modern development practices and scalable design patterns.