```rs
let std = import("std");
let fmt = import("fmt");
let http = import("http");

let server = http::server();

server::route("/hello/:name", fn(params) = {
  fn(request, response) = {
    params::method |> match {
      "GET" => fmt::format("Hello, {}!", params::name),
      _     => "Method not allowed"
    } |> response::send;
  }
});

server::start(3003);
```

```rs
let std = import("std");
let fmt = import("fmt");
let http = import("http");

let server = http::server();

server::route("/hello/:name", fn({ params, query }) => {
  let greeting = query::contains("greeting") ? query::get("greeting") : "Hello";
  fn(request, response) => {
    params::method |> match _ {
      "GET" => {
        let message = fmt::format("{} {}!", greeting, params::name);
        message |> response::send;
      },
      _ => response::send("Method not allowed"),
    }
  }
});

server::start(3003);
```
