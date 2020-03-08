# actix-graphql-react-apollo

**NOT** complete - Clean boilerplate for GraphQL app built with Rust & TypeScript.

## Features

- DB migration with Diesel
- Sign up
- Sign in
- Change password
- Profile Update
- JSON web token authentication

## Stacks

### Backend

- [Rust](https://www.rust-lang.org/zh-CN/)
- [actix-web](https://github.com/actix/actix-web) - Web server
- [juniper](https://github.com/graphql-rust/juniper) - GraphQL server
- [diesel](https://github.com/diesel-rs/diesel) - ORM
- DB: Postgres
- JSON Web Token : Authentication

### Frontend

- TypeScript
- react - user interfaces
- apollo-client - GraphQL client

## How to run?

### Backend

``` Bash
 $ git clone https://github.com/zzy/actix-graphql-react-apollo.git
 $ cd actix-graphql-react-apollo/backend/
```

#### Put the `DATABASE_URL` & `port` in a `.env` file.

``` Shell
$ cargo install diesel_cli --no-default-features --features postgres
$ echo DATABASE_URL=postgres://username:password@localhost/actix_graphql > .env
$ echo GRAPHQL_PORT=5000 >> .env
$ diesel setup
$ diesel migration run // option
$ cargo run
```

GraphiQL: connect to http://localhost:5000/ with browser.

### Frontend

``` Bash
 $ git clone https://github.com/zzy/actix-graphql-react-apollo.git
 $ cd actix-graphql-react-apollo/frontend/
```

#### yarn

``` Bash
 $ yarn install # yarn upgrade; yarn install
 $ yarn start
```

#### npm

``` Bash
 $ npm install # npm up; npm install
 $ npm start
```

Then go to http://localhost:3000/.
