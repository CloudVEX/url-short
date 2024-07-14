# url-short
[![License](https://img.shields.io/badge/license-Apache-orange.svg)](https://github.com/CloudVEX/url-short/blob/main/LICENSE)
[![Created](https://img.shields.io/github/created-at/CloudVEX/url-short?color=orange
)](https://github.com/CloudVEX/url-short)
[![Activity](https://img.shields.io/github/commit-activity/m/CloudVEX/url-short?color=orange
)](https://github.com/CloudVEX/url-short/graphs/contributors)
### A simple url shortener built to get a basic feel of mongodb

### Usage
 - GET -> `http://127.0.0.1:8000/<short-code>` (Redirects you to the destination of the shortcode)
 - POST -> `http://127.0.0.1:8000/shorten/<url>` (Shortens the URL and returns a shortcode)
 - DELETE -> `http://127.0.0.1:8000/<short-code>` [JSON Body: { "username": "", "password": ""} ] (Deletes the shortcode)
