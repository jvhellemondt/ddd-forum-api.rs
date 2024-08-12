#!/bin/sh

# Generated with ChatGPT. Of course

BASE_URL='http://localhost:8080'

print_response() {
    response=$1
    code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d' | jq .)

    case $code in
        2*)
            tput setaf 2
            echo "Response Code: $code"
            ;;
        4*|5*)
            tput setaf 1
            echo "Response Code: $code"
            ;;
        *)
            tput setaf 3
            echo "Response Code: $code"
            ;;
    esac
    tput sgr0
    echo "$body"
    echo ""
}

print_input() {
    echo "Input Data:"
    echo "$1" | jq .
}

print_request() {
    method=$1
    url=$2
    echo "Method: $method"
    echo "URL: $url"
}

echo "ERROR | it should return user not found / 404"
print_request "GET" "${BASE_URL}/users?email=john%40EXAMPLE.com"
response=$(curl --silent --location --write-out "\n%{http_code}" "${BASE_URL}/users?email=john%40EXAMPLE.com")
print_response "$response"

echo "ERROR | it should return user not found / 404"
data='{
    "last_name": "Snow"
}'
print_request "PUT" "${BASE_URL}/users/update/101"
print_input "$data"
response=$(curl --silent --location --request PUT --write-out "\n%{http_code}" "${BASE_URL}/users/update/101" \
--header 'Content-Type: application/json' \
--data "$data")
print_response "$response"

echo "ERROR | it should throw a validation error / 400"
data='{
    "username": "john",
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "notValidField": "error"
}'
print_request "POST" "${BASE_URL}/users/create"
print_input "$data"
response=$(curl --silent --location --write-out "\n%{http_code}" "${BASE_URL}/users/create" \
--header 'Content-Type: application/json' \
--data-raw "$data")
print_response "$response"

echo "SUCCESS | it should create the user / 201"
data='{
    "username": "john",
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe"
}'
print_request "POST" "${BASE_URL}/users/create"
print_input "$data"
response=$(curl --silent --location --write-out "\n%{http_code}" "${BASE_URL}/users/create" \
--header 'Content-Type: application/json' \
--data-raw "$data")
print_response "$response"

echo "ERROR | it should return username already in use / 409"
data='{
    "username": "john",
    "email": "john-snow@example.com",
    "first_name": "John",
    "last_name": "Doe"
}'
print_request "POST" "${BASE_URL}/users/create"
print_input "$data"
response=$(curl --silent --location --write-out "\n%{http_code}" "${BASE_URL}/users/create" \
--header 'Content-Type: application/json' \
--data-raw "$data")
print_response "$response"

echo "ERROR | it should return email already in use / 409"
data='{
    "username": "john-snow",
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe"
}'
print_request "POST" "${BASE_URL}/users/create"
print_input "$data"
response=$(curl --silent --location --write-out "\n%{http_code}" "${BASE_URL}/users/create" \
--header 'Content-Type: application/json' \
--data-raw "$data")
print_response "$response"

echo "SUCCESS | it should return the user / 200"
print_request "GET" "${BASE_URL}/users?email=john%40EXAMPLE.com"
response=$(curl --silent --location --write-out "\n%{http_code}" "${BASE_URL}/users?email=john%40EXAMPLE.com")
print_response "$response"

echo "ERROR | it should return validation error / 400"
data='{
    "admin": true
}'
print_request "PUT" "${BASE_URL}/users/update/1"
print_input "$data"
response=$(curl --silent --location --request PUT --write-out "\n%{http_code}" "${BASE_URL}/users/update/1" \
--header 'Content-Type: application/json' \
--data "$data")
print_response "$response"

echo "SUCCESS | it should update the user / 200"
data='{
    "username": "winter-is-coming",
    "last_name": "Snow"
}'
print_request "PUT" "${BASE_URL}/users/update/1"
print_input "$data"
response=$(curl --silent --location --request PUT --write-out "\n%{http_code}" "${BASE_URL}/users/update/1" \
--header 'Content-Type: application/json' \
--data "$data")
print_response "$response"
