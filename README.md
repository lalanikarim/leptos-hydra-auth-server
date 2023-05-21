# Overview
A proof-of-concept implementation of Ory/Hydra's Login and Client UIs in Leptos, a Rust web framework, and server logic for those UIs using Leptos server functions.

## Ory/Hydra
[Hydra](https://www.ory.sh/docs/oauth2-oidc/) is an OAuth 2.0 and OpenID Connect provider. It provides you with a certified OAuth 2.0 and OpenID Connect implementation that you can use to build your own OAuth 2.0 and OpenID Connect authentication server.  
To build your authentication server, you first create a web application that can:
1. Render UI to collect login credentials and any necessary consent from the users
2. Verify the provided credentials against your user database
3. Communicate with Hydra's admin API on the success or failure of verification  

Once your web application is built and deployed, you update Hydra's configuration and provide the login and consent UI endpoints for your web application. Hydra takes care of the rest.  
You now have an OAuth 2.0 and OpenID-Connect compliant identity provider that can authenticate users and issue access tokens, refresh tokens, and ID tokens to your users, not only for your internal applications but also for external third-party applications, should they choose to, or with those applications that allow you to bring your own OAuth 2.0 authentication providers.

## Leptos Web Framework
In their own words, [Leptos](https://github.com/leptos-rs/leptos) is a full-stack, isomorphic Rust web framework leveraging fine-grained reactivity to build declarative user interfaces.  
Leptos combines the power of reactive lightweight Javascript frontend frameworks like SolidJS with the correctness imposed by Rust's borrow checker.  
One of Leptos' many powerful features is that it allows you to write colocated frontend and backend code, which Lepto's "server" macro separates into backend and frontend applications along with any additional boilerplate code necessary to communicate and exchange data between the frontend and backend. You can now focus on implementing your business logic while the "server" macro handles the tedious yet necessary boilerplate code.

## Setup
In order to test this integration, you'll need to:
1. Copy the provided `.env.example` to `.env` file.
2. create and run `hydra` container using the provided sample `docker-compose.yml`
```
docker-compose up -d
```
3. create an `oauth2-client` using the following command against the `hydra` container
```
docker-compose exec hydra hydra create client --endpoint=http://localhost:4445 --token-endpoint-auth-method client_secret_post --redirect-uri http://127.0.0.1:3000
```
4. Copy the generated `CLIENT ID` and `CLIENT SECRET` values and paste them in `.env` for `OAUTH2_CLIENT_ID` and `OAUTH2_CLIENT_SECRET`. Don't forget to uncomment those variables.
5. compile and run the project
```
cargo leptos watch
```
6. Visit `http://127.0.0.1:3000` in your browser to test the sample integration.

## Demo OAuth2 Client
The project includes a demo OAuth2 client for testing purposes only. You may use it to verify that the server logic is working as expected. The client does the bare minimum to help verify the server and was included so you wouldn't need to create a separate application for it.
