use log::debug;

const HTTP_MESSAGE_INDEX: &str =
    "
HTTP/1.1 200 OK
Date: Mon, 23 May 2005 22:38:34 GMT
Content-Type: text/html; charset=UTF-8
Last-Modified: Wed, 08 Jan 2003 23:11:55 GMT
Server: Apache/1.3.3.7 (Unix) (Red-Hat/Linux)
ETag: \"3f80f-1b6-3e1cb03b\"
Accept-Ranges: bytes
Connection: close

<!DOCTYPE html>
<html>
<head>
<link rel=\"stylesheet\" href=\"style.css\">
<link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/css?family=FiraCode\">
</head>
<body>

<div class=\"topnavholder\">

<div class=\"title\">
<h1>
ebains.dev
</h1>
</div>

<div class=\"topnav\">
<a class=\"active\" href=\"index.html\">Home</a>
<a href=\"https://github.com/ewan15/\">Github</a>
</div>

</div>

<p>I am a programmer beep boop. I enjoy programming low level systems in languages such as Rust & C++. In my free time I develop VR Games, build autotraders and try to break my kernel. This website was built using my own http server which uses io_uring (<a href=\"https://github.com/ewan15/eipher\">eipher</a>)</p>

Contact: ewan.bains@gmail.com

</body>
</html>
";

const HTTP_MESSAGE_STYLE: &str =
"
HTTP/1.1 200 OK
Date: Mon, 23 May 2005 22:38:34 GMT
Content-Type: text/css; charset=UTF-8
Last-Modified: Wed, 08 Jan 2003 23:11:55 GMT
Server: Apache/1.3.3.7 (Unix) (Red-Hat/Linux)
ETag: \"3f80f-1b6-3e1cb03b\"
Accept-Ranges: bytes
Connection: close

body {
  background-color: #5c5c5c;
  font-family: \"Fira Code\", sans-serif;
}

h1 {
  color: white;
  text-align: center;
}

p {
  font-size: 20px;
}

.topnavholder {
    width: 100%;
    text-align: left;
    float: left;
  vertical-align: middle;
}

.title {
    text-align: left;
    width: 10%;
    float: left;
}

.topnav {
  vertical-align: middle;
  background-color: #333;
  overflow: hidden;
  float: right;
  margin-top: 1%;
}

.topnav a {
  float: left;
  color: #f2f2f2;
  text-align: center;
  padding: 14px 16px;
  text-decoration: none;
  font-size: 17px;
}

.topnav a:hover {
  background-color: #ddd;
  color: black;
}

.topnav a.active {
  background-color: #04AA6D;
  color: white;
}

.pgp {
  font-family: verdana;
  background-color: #333;

  margin: auto;
  width: 50%;
  border: 3px solid green;
  padding: 10px;
}

";


const HTTP_MESSAGE2: &str =
    "
HTTP/1.1 200 OK
Date: Mon, 23 May 2005 22:38:34 GMT
Content-Type: text/html; charset=UTF-8
Content-Length: 155
Last-Modified: Wed, 08 Jan 2003 23:11:55 GMT
Server: Apache/1.3.3.7 (Unix) (Red-Hat/Linux)
ETag: \"3f80f-1b6-3e1cb03b\"
Accept-Ranges: bytes
Connection: close

<html>
  <head>
    <title>Ewan Website</title>
  </head>
  <body>
    <p>wowwwwwww</p>
  </body>
</html>
";


pub enum HttpError {
    UnableToParseMessage()
}

pub struct HttpServer {

}

impl HttpServer {
    pub fn new() -> Self {
        Self {

        }
    }

    pub async fn run(&self) {

    }

    pub fn process_message(&self, http_message: &str) -> Result<String, HttpError> {
        let mut message_iterator = http_message.split("\n");
        let request_query = message_iterator.next().ok_or(HttpError::UnableToParseMessage())?;

        let mut request_query_iterator = request_query.split(" ");
        let request_type = request_query_iterator.next().ok_or(HttpError::UnableToParseMessage())?;
        let path = request_query_iterator.next().ok_or(HttpError::UnableToParseMessage())?;

        debug!("requesting path: {}", path);
        if path == "/" {
            Ok(HTTP_MESSAGE_INDEX.to_string())
        } else if path == "/style.css" {
            Ok(HTTP_MESSAGE_STYLE.to_string())
        } else {
            Ok(HTTP_MESSAGE2.to_string())
        }
    }
}