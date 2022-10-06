# Newsletter API
This is a full-fledged Newsletter API containing all the main features for building a sizable and scalable service. It is fully test-driven, every feature implemented is solid, stable and reliable. It has been written in pair programming with the book zero2prod

# How to run it

Add 
`configuration/base.yaml`
```
application:
  port: 8000
database:
  host: "localhost"
  port: 5432
  username: "username"
  password: "pass"
  database_name: "newsletter"
email_client:
  base_url: "localhost"
  sender_email: "test@gmail.com"
  authorization_token: "Postmark auth"
  timeout_milliseconds: 10000
```

`configuration/local.yaml`

```
application:
  host: 127.0.0.1
  base_url: "http://127.0.0.1"
database:
  require_ssl: false
```

`production.yaml`

```
application:
  host: 0.0.0.0
database:
  require_ssl: true
email_client:
  base_url: "https://api.postmarkapp.com"
  sender_email: "realemail@gmail.com"
```

Launch a (migrated) Postgres database via Docker:
```
./scripts/init_db.sh
```

`>cargo run` 

