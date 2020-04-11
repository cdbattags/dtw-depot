#!/usr/bin/env bash

set -xe

schemats generate -c "postgres://ash:pokemon@pokeapi_db_1/pokeapi" -s public -o pokeapi.ts && cat pokeapi.ts