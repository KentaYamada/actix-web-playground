# Auth API
## [POST] /api/auths/signin
Signin user.

### Request
| field | data type | Nullable |
| :-- | :--- | :--- |
| email | string | x |
| password | string | x |

### [WIP] Response payload
| field | data type | Nullable |
| :-- | :--- | :--- |
| id | integer | x |
| first_name | string | x |
| family_name | string | x |
| email | string | x |
| password | string | x |

### Request example
``` sh
curl -v -s -X POST http://localhost:8080/api/auths/signin \
     -H "content-type: application/json" \
     -d '{ "email": "dev@email.com", "password": "dev" }' | jq
```
