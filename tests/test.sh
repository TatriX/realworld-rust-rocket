#!/bin/sh
newman run Conduit.postman_collection.json -e Conduit.postman_integration_test_environment.json --global-var "EMAIL=tester@test.test" --global-var "PASSWORD=qwerty"
