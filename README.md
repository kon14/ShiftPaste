
<div align="center">
<br>
<a href="https://github.com/kon14/ShiftPaste" target="_blank">
    <h1>ShiftPaste 📋</h1>
</a>
A pastebin service focused on editable snippets with persistent links.
</div>

<hr />

ShiftPaste is a flexible backend solution for creating, managing, and sharing editable snippets.<br />
With a primary emphasis on content updates, ShiftPaste provides users with persistent links that allow the associated data to be modified without breaking the original link.<br />
This dynamic approach ensures that snippets remain accessible even as their content evolves.

ShiftPaste also supports QR code generation for seamless sharing and retrieval, making it an ideal choice for anyone needing a robust, adaptable snippet storage service.

---

## Building 🔨 <a name="building"></a>

``` bash
# Build ShiftPaste
docker build -t shiftpaste .
```

## Running 💻 <a name="running"></a>

``` bash
# Configure PostgreSQL (user: postgres, db: postgres)
docker run --name shiftpaste-postgres -p 5432:5432 -e POSTGRES_DB="shiftpaste" -e POSTGRES_HOST_AUTH_METHOD="trust" -d postgres

# Apply DB Migrations (requires SQLX CLI)
DATABASE_URL="postgres://postgres@localhost:5432/shiftpaste" sqlx migrate run

# Run ShiftPaste
docker run --name=shiftpaste -p 4000:4000 \
-e DATABASE_URL="postgres://postgres@host.docker.internal:5432/shiftpaste" \
-e API_BASE_URL="http://localhost:4000" \
-e AUTH_JWT_SECRET="7h3 c4k3 15 4 l13" \
-d shiftpaste

# Navigate to Swagger UI (on Linux)
xdg-open "http://localhost:4000/swagger/"
```

---

## Environment Variables 📃 <a name="env-vars"></a>

|              Variable              | Description                                                                                                                                                                                                          | Required |         Default          |            Example             |
|:----------------------------------:|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:--------:|:------------------------:|:------------------------------:|
|           `DATABASE_URL`           | The connection string URL for your PostgreSQL database.                                                                                                                                                              |  `True`  |            —             | `postgres://localhost:5432/db` |
|             `API_PORT`             | The port to be used by the HTTP server.                                                                                                                                                                              | `False`  |          `4000`          |             `8080`             |
|           `API_BASE_URL`           | A public URL pointing to the backend API's root path.                                                                                                                                                                |  `True`  |            —             |   `https://foo.bar.baz/api`    |
|       `APP_SNIPPET_VIEW_URL`       | A public URL pointing to the frontend app's snippet preview page.<br />May be used to customize snippet redirections.<br />Frontend app is expected to handle `GET` requests at `$APP_SNIPPET_VIEW_URL/:snippet_id`. | `False`  | `$API_BASE_URL/snippets` | `https://foo.bar.baz/snippets` |
|             `RUST_LOG`             | Specifies the desired logging level.<br />Refer to the [env_logger](https://docs.rs/env_logger/latest/env_logger/) documentation for details.                                                                        | `False`  |         `error`          |             `info`             |
|         `AUTH_JWT_SECRET`          | The secret to be used for JWT authentication token encoding/decoding.                                                                                                                                                |  `True`  |            —             |      `7h3 c4k3 15 4 l13`       |
| `AUTH_ACCESS_TOKEN_DURATION_SECS`  | Duration for authentication access token validity (in seconds).                                                                                                                                                      | `False`  |   `5 * 60` (5 minutes)   |             `300`              |
| `AUTH_REFRESH_TOKEN_DURATION_SECS` | Duration for authentication refresh token validity (in seconds).                                                                                                                                                     | `False`  |  `24 * 60 * 60` (1 day)  |            `86400`             |

---

## Local Development 👨🏻‍🔬 <a name="local-dev"></a>

The following section assumes your environment contains an installation of the [Rust development toolchain](https://www.rust-lang.org/tools/install).

``` bash
# Prepare Git Hooks
lefthook install

# Install the SQLX CLI
cargo install sqlx-cli --no-default-features --features postgres
```

``` bash
# Apply DB Migrations
DATABASE_URL="postgres://postgres@localhost:5432/shiftpaste" sqlx migrate run

# Build ShiftPaste
cargo build

# Run ShiftPaste
DATABASE_URL="postgres://postgres@localhost:5432/shiftpaste" \
API_BASE_URL="http://localhost:4000" \
AUTH_JWT_SECRET="7h3 c4k3 15 4 l13" \
cargo run

# Navigate to Swagger UI (on Linux)
xdg-open "http://localhost:4000/swagger/"
```
