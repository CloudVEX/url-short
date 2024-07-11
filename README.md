# Hey :D

### Usage
 - GET -> http://127.0.0.1:8000/<short-code> (Redirects you to the destination of the shortcode)
 - POST -> http://127.0.0.1:8000/shorten/<url> (Shortens the URL and returns a shortcode)
 - DELETE -> http://127.0.0.1:8000/<short-code> [JSON Body: { "username": "", "password": ""} ] (Deletes the shortcode)