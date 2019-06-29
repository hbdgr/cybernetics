```sh
# get all objects:
curl -X GET http://localhost:8000/objects

# get by id:
curl -X GET http://localhost:8000/objects/1

# insert object:
curl -X POST -H 'Content-Type: application/json' http://localhost:8000/objects --data '{ "content": "curl post" }'

# update object:
curl -X PUT -H 'Content-Type: application/json' http://localhost:8000/objects/16 --data '{ "id":16, "content": "curl post4" }'

# delete object:
curl -X DELETE http://localhost:8000/objects/13
```

