# Ark
A full-stack monolithic application that heavily emphasizes its (IAM) solution. Initially, it was supposed to be closed source because it was going to be used for a side project, but I decided to release it to the public for personal reasons. Note: it's missing some fundamental things, so it's not ready for production use yet.
<br><br>
Technically, it is not a full stack just yet, I haven't wrote the frontend yet.
<br /><br />
![Status](https://img.shields.io/badge/status-not_production_ready-yellow)
![Version](https://img.shields.io/badge/ark-0.1.0-orange)
![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/w/notpointless/ark/main)

## Technical Breakdown
If you're interested in my thought process while I was building out the foundation of this project, you can view the post I made about this project on my substack [here](https://chomnr.substack.com/p/project-breakdown-ark)

## Installation

This section guides you through the process of installing the necessary tools and building the project.

### Prerequisites

Before you begin, ensure you have installed Rust, Cargo, PostgreSQL, and Redis on your system.

* Rust
* PostgreSQL
* Redis
* OAuth Provider (ex: Discord)

### From the source
```bash
git clone https://github.com/notpointless/ark.git
cd ark
cargo build
cargo run
```

### Setting environment variables
```bat
SET PG_HOST=
SET PG_USER=
SET PG_PASSWORD=
SET PG_DBNAME=
SET REDIS_HOST=
SET REDIS_USER=
SET REDIS_PASSWORD=
SET REDIS_DBNAME=
SET DISCORD_CLIENT_ID=
SET DISCORD_CLIENT_SECRET=
SET DISCORD_AUTH_URL=https://discord.com/oauth2/authorize
SET DISCORD_TOKEN_URL=https://discord.com/api/oauth2/token
SET DISCORD_REVOCATION_URL=https://discord.com/api/oauth2/token/revoke
SET OAUTH2_REDIRECT_URL=http://localhost:3000/auth/callback
SET COOKIE_ENCRYPTION_KEY=TESTKEY1324E31324123421244123TESTFEY1214E31324123421244123TESTKEY1224E31324123421244123
```