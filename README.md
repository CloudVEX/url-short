# Hey :D

### Usage
 - GET -> http://127.0.0.1:8000/&lt;short-code&gt; (Redirects you to the destination of the shortcode)
 - POST -> http://127.0.0.1:8000/shorten/&lt;url&gt; (Shortens the URL and returns a shortcode)
 - DELETE -> http://127.0.0.1:8000/&lt;short-code&gt; [JSON Body: { "username": "", "password": ""} ] (Deletes the shortcode)