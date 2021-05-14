#!/bin/bash

wasm-pack build
cd www
npm run build