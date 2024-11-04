```rs
let std = import('std');
let fmt = import('fmt');
let http = import('http');

let server = http::server();

server::route('/hello/:name', fn(params) = {
  fn(request, response) = {
    params::method |> match _ {
      'GET' => fmt::format('Hello, {}!', params::name),
      _     => 'Method not allowed'
    } |> response::send;
  }
});

server::start(3003);
```
