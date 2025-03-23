# ER diagram
``` mermaid
erDiagram
    users ||--o{ todos : "user has one or more todos."
    users ||--o{ login_histories : "user has an active token."
```

## users
| Physical name | Logical name | Data type | Nullable | Note |
| :--- | :--- | :--- | :---: | :--- |
| id | ユーザーID | intger | x | |
| first_name | ユーザー名 (姓) | string | x | |
| first_name | ユーザー名 (名) | string | x | |
| email | メールアドレス | string | x | |
| password | パスワード | string | x | |
| created_at | 作成日時 | datetime | x | |
| modified_at | 更新日時 | datetime | x | |

### constraints
| fields | Key | Note |
| :--- | :--- | :--- |
| id | PK | |

## login_histories
| Physical name | Logical name | Data type | Nullable | Key | Note |
| :--- | :--- | :--- | :---: | :--- | :--- |
| user_id | ユーザーID | integer | x | | |
| last_logged_in_at | 最終ログイン日時 | datetime | x | | |
| token | ログイントークン | string | x | | |
| expired_at | トークン有効期限 | string | x | | |
| created_at | 作成日時 | datetime | x | |

### constraints
| fields | Key | Note |
| :--- | :--- | :--- |
| user_id | FK | |
| user_id, last_logged_in_at | UK | |
| token | UK | |

## todos
| Physical name | Logical name | Data type | Nullable | Key | Note |
| :--- | :--- | :--- | :---: | :--- | :--- |
| id | ユーザーID | intger | x | PK | |
| user_id | ユーザーID | integer | x | | |
| status | todoステータス | integer | x | | |
| title | タイトル | string | x | |
| detail | 詳細内容 | string | x | |
| created_at | 作成日時 | datetime | x | |
| modified_at | 更新日時 | datetime | x | |

### constraints
| fields | Key | Note |
| :--- | :--- | :--- |
| user_id | FK | |

### index
| fields | Note |
| :--- | :--- |
| status, modified_at | (todo)更新日時の値が変わる度にインデックスできそう... |