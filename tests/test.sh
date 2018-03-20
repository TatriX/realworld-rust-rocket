#!/bin/sh
newman run ./Conduit.postman_collection.json --global-var "apiUrl=http://localhost:8000/api"
