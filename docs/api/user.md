# User API
## [GET] /api/users
Get user list.

### Request
WIP
todo: implement query parameters

### Response
| field | data type | Nullable |
| :-- | :--- | :--- |
| first_name | string | x |
| family_name | string | x |
| email | string | x |
| password | string | x |

Response payload
``` json
{
    "message": "",
    "data": {
       "users": [
            {
                "first_name": "Foo",
                "family_name": "Bar",
                "email": "foo.bar@email.com",
                "password": "foobar"
            }
        ]
    }
}
```

Request
``` sh
# format json if you need.
curl -X GET http://localhost:8080/api/users -H "content-type: application/json" | jq

# dump header if you need.
curl -i -X GET http://localhost:8080/api/users -H "content-type: application/json"
```

## [GET] /api/users/{id}
Get specified user data.

### Request
| field | data type | Nullable |
| :-- | :--- | :--- |
| id | integer | x |

### Response
| field | data type | Nullable |
| :-- | :--- | :--- |
| first_name | string | x |
| family_name | string | x |
| email | string | x |
| password | string | x |

Response payload
``` json
{
    "message": "",
    "data": {
        "user": {
            "first_name": "Foo",
            "family_name": "Bar",
            "email": "foo.bar@email.com",
            "password": "foobar"
        }
    }
}
```

Request example
``` sh
# format json if you need.
curl -X GET http://localhost:8080/api/users/1 -H "content-type: application/json" | jq

# dump header if you need.
curl -i -X GET http://localhost:8080/api/users/1 -H "content-type: application/json"
```

## [POST] /api/users
Create user data.

### Request
| field | data type | Nullable |
| :-- | :--- | :--- |
| first_name | string | x |
| family_name | string | x |
| email | string | x |
| password | string | x |

### Response
| field | data type | Nullable |
| :-- | :--- | :--- |
| message | string | x |
| id | integer | x |

Response payload
``` json
{
    "message": "Created successfully",
    "data": {
        "id": 1
    }
}
```

Request example
``` sh
# format json if you need.
curl -X POST http://localhost:8080/api/users \
     -H "content-type: application/json" \
     -d '{
        "first_name": "hoge",
        "family_name": "fuga",
        "email": "hoge@email.com",
        "password": "dev"
    }' \
    | jq

# dump header if you need.
curl -i -X POST http://localhost:8080/api/users \
     -H "content-type: application/json" \
     -d '{
        "first_name": "hoge",
        "family_name": "fuga",
        "email": "hoge@email.com",
        "password": "dev"
    }' 
```

## [PATCH] /api/users/{id}
Modify user data.

### Request
| field | data type | Nullable |
| :-- | :--- | :--- |
| first_name | string | x |
| family_name | string | x |
| email | string | x |
| password | string | x |

### Response
| field | data type | Nullable |
| :-- | :--- | :--- |
| message | string | x |
| id | integer | x |

Response payload
``` json
{
    "message": "Updated successfully",
    "data": {
        "id": 1
    }
}
```

Request example
``` sh
# format json if you need.
curl -i -X PATCH http://localhost:8080/api/users/1 \
     -H "content-type: application/json" \
     -d '{
        "first_name": "hoge",
        "family_name": "fuga",
        "email": "hoge@email.com",
        "password": "dev"
    }' \
    | jq

# dump header if you need.
curl -X PATCH http://localhost:8080/api/users/1 \
     -H "content-type: application/json" \
     -d '{
        "first_name": "hoge",
        "family_name": "fuga",
        "email": "hoge@email.com",
        "password": "dev"
    }' 
```

## [DELETE] /api/users/{id}
Delete specified user data.

### Request
| field | data type | Nullable |
| :-- | :--- | :--- |
| id | integer | x |

### Response
| field | data type | Nullable |
| :-- | :--- | :--- |
| message | string | x |
| data | object | o |

Response payload
``` json
{
    "message": "Deleted successfully",
    "data": null
}
```

Request example
``` sh
# format json if you need.
curl -X DELETE http://localhost:8080/api/users/1 -H "content-type: application/json" | jq

# dump header if you need.
curl -i -X DELETE http://localhost:8080/api/users/1 -H "content-type: application/json"
```
