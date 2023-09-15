# Server-Side Rendering with Rust

This repository demonstrates server-side rendering (SSR) using Rust, showcasing the integration of Axum, static HTML/JS/CSS files, and the Askama template engine. SSR is a powerful technique for building web applications that improves performance and SEO by rendering content on the server before sending it to the client.

## Tech Stack Details

### Axum Function

The core of this application utilizes the Axum web framework to handle HTTP requests and generate HTTP responses.

### Static HTML

This application serves static HTML, JavaScript, and CSS files directly to the client. These files are pre-rendered and optimized for faster load times.

### Askama Template

Askama is used as a template engine to generate HTML on the server based on provided templates. This allows for dynamic content rendering and customization.

## Quick Start

To run the application locally, follow these steps:

1. Clone the repository:

   ```shell
   git clone https://github.com/mikezupper/ssr-template-rs.git
   cd ssr-template-rs
   ```

2. Build and Run the Rust Application

```shell
cargo run
```

3. To test, visit the following urls

### Axum Function

http://localhost:4000/

### Static Html

http://localhost:4000/index_static.html

### Askama Template

http://localhost:4000/index_tempalate.html

## Devops

### Build with Docker

To containerize the application, you can use Docker. Build a Docker image with the following command:

```shell
docker build . -t ssr-template-rs:latest
```

### Run with Docker

After building the Docker image, you can run the application in a Docker container:

```shell
docker run -p 4000:4000 ssr-template-rs:latest
```

### Build & Run with Docker Compose

For a more comprehensive deployment setup, you can use Docker Compose. It simplifies the process of managing containers and their dependencies:

```shell
docker compose up
```