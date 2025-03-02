# actix-web-playground
Rust Web application playground.  
Implement simple todo application.

## Technology Stacks
- Backend
    - Docker
    - Rust
    - actix-web
    - Nginx
    - PostgreSQL
    - PgAdmin
- Frontend
    - TypeSctipt
    - React
    - Vite

## Architecture
``` mermaid
graph TD;
    subgraph Client
        A[User] -->|HTTP Request| B[Browser]
    end

    subgraph Server
        B -->|Reverse Proxy| C[Nginx]

        subgraph Applications
            C -->|Static Files| D[React App]  
            C -->|API Request /api| E[Actix-web]
        end

        E -->|DB Query| F[(PostgreSQL)]
    end

    subgraph Database Management
        G[pgAdmin] -->|DB Admin Access| F
    end
```

## Getting started
### Pre-requirements
- your favarite editor or IDE
- docker & docker compose
- jq (recommended)

### 1. Run docker 
``` sh
#build images
docker compose build --no-cache

# run containers
docker compose up
or
docker compose up -d
```
Note: Configure PgAdmin and Postgres access information in the .env file.

### Applications
| URL | Application |
| :-- | :--- |
| http://localhost:8080 | main application |
| http://localhost:8080/api | main app api |
| http://localhost:5050 | pgAdmin |
