#!/usr/bin/env bash

file_name="/Users/shaozengwei/projects/demo/data1.csv"
curl -XPUT 'http://root:@localhost:8000/v1/streaming_load' -H "insert_sql: insert into test.test(name,age,location) format csv" -H 'skip_header: 0' -F "upload=@"${file_name}""