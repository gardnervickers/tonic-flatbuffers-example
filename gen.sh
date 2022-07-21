#!/bin/bash
docker run -v $PWD:/src neomantra/flatbuffers flatc -o /src/proto --rust /src/proto/hello.fbs