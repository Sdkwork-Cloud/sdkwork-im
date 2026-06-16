# Sdkwork IM API 文档

## 概述

本文档描述 Sdkwork IM 即时通信系统的 REST API 设计，遵循行业标准（Discord、Slack、Matrix）。

## API 版本

- 当前版本：`v1`
- 基础路径：`/api/v1/`

## 服务划分

| 服务 | 路径前缀 | 职责 |
|------|----------|------|
| **contact-service** | `/api/v1/contacts/` | 好友、屏蔽、单聊、用户资料 |
| **space-service** | `/api/v1/spaces/` | 空间、群组、频道、成员、邀请、封禁 |
| **interaction-service** | `/api/v1/interactions/` | Reaction、Pin、Thread |

---

## Contact Service API（联系人服务）

### 好友请求

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/contacts/friend-requests` | 发送好友请求 |
| GET | `/api/v1/contacts/friend-requests` | 获取好友请求列表 |
| GET | `/api/v1/contacts/friend-requests/{request_id}` | 获取好友请求详情 |
| POST | `/api/v1/contacts/friend-requests/{request_id}/accept` | 接受好友请求 |
| POST | `/api/v1/contacts/friend-requests/{request_id}/decline` | 拒绝好友请求 |
| POST | `/api/v1/contacts/friend-requests/{request_id}/cancel` | 取消好友请求 |

#### POST /api/v1/contacts/friend-requests

**Request:**
```json
{
  "target_user_id": "user_123",
  "request_message": "Hi, let's be friends!"
}
```

**Response (201):**
```json
{
  "request_id": "123456789012345678",
  "requester_user_id": "user_456",
  "target_user_id": "user_123",
  "status": "pending",
  "request_message": "Hi, let's be friends!",
  "created_at": "2026-06-16T10:00:00Z"
}
```

### 好友关系

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/contacts/friends` | 获取好友列表 |
| GET | `/api/v1/contacts/friends/{friendship_id}` | 获取好友详情 |
| DELETE | `/api/v1/contacts/friends/{friendship_id}` | 删除好友 |

### 用户屏蔽

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/contacts/blocks` | 屏蔽用户 |
| GET | `/api/v1/contacts/blocks` | 获取屏蔽列表 |
| GET | `/api/v1/contacts/blocks/{block_id}` | 获取屏蔽详情 |
| DELETE | `/api/v1/contacts/blocks/{block_id}` | 取消屏蔽 |

#### POST /api/v1/contacts/blocks

**Request:**
```json
{
  "blocked_user_id": "user_123",
  "scope": "all",
  "reason": "Spam"
}
```

### 单聊会话

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/contacts/direct-chats` | 创建单聊 |
| GET | `/api/v1/contacts/direct-chats` | 获取单聊列表 |
| GET | `/api/v1/contacts/direct-chats/{direct_chat_id}` | 获取单聊详情 |
| PATCH | `/api/v1/contacts/direct-chats/{direct_chat_id}` | 更新单聊状态 |

### 用户资料

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/contacts/users/{user_id}/profile` | 获取用户资料 |
| PATCH | `/api/v1/contacts/users/{user_id}/profile` | 更新用户资料 |

#### PATCH /api/v1/contacts/users/{user_id}/profile

**Request:**
```json
{
  "im_nickname": "John",
  "im_avatar_url": "https://example.com/avatar.jpg",
  "im_status_message": "Available"
}
```

---

## Space Service API（空间服务）

### 空间管理

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/spaces` | 创建空间 |
| GET | `/api/v1/spaces` | 获取空间列表 |
| GET | `/api/v1/spaces/{space_id}` | 获取空间详情 |
| PATCH | `/api/v1/spaces/{space_id}` | 更新空间 |
| DELETE | `/api/v1/spaces/{space_id}` | 删除空间 |

#### POST /api/v1/spaces

**Request:**
```json
{
  "space_name": "My Workspace",
  "space_type": "team",
  "description": "A workspace for my team",
  "max_members": 1000
}
```

**Response (201):**
```json
{
  "space_id": "123456789012345678",
  "space_name": "My Workspace",
  "space_type": "team",
  "owner_user_id": "user_456",
  "max_members": 1000,
  "created_at": "2026-06-16T10:00:00Z"
}
```

### 空间成员

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/spaces/{space_id}/members` | 获取成员列表 |
| POST | `/api/v1/spaces/{space_id}/members` | 添加成员 |
| GET | `/api/v1/spaces/{space_id}/members/{user_id}` | 获取成员详情 |
| PATCH | `/api/v1/spaces/{space_id}/members/{user_id}` | 更新成员角色 |
| DELETE | `/api/v1/spaces/{space_id}/members/{user_id}` | 移除成员 |

### 群组管理

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/spaces/{space_id}/groups` | 创建群组 |
| GET | `/api/v1/spaces/{space_id}/groups` | 获取群组列表 |
| GET | `/api/v1/spaces/{space_id}/groups/{group_id}` | 获取群组详情 |
| PATCH | `/api/v1/spaces/{space_id}/groups/{group_id}` | 更新群组 |
| DELETE | `/api/v1/spaces/{space_id}/groups/{group_id}` | 删除群组 |

### 群组成员

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/spaces/{space_id}/groups/{group_id}/members` | 获取成员列表 |
| POST | `/api/v1/spaces/{space_id}/groups/{group_id}/members` | 添加成员 |
| PATCH | `/api/v1/spaces/{space_id}/groups/{group_id}/members/{user_id}` | 更新成员 |
| DELETE | `/api/v1/spaces/{space_id}/groups/{group_id}/members/{user_id}` | 移除成员 |

### 频道管理

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/spaces/{space_id}/channels` | 创建频道 |
| GET | `/api/v1/spaces/{space_id}/channels` | 获取频道列表 |
| GET | `/api/v1/spaces/{space_id}/channels/{channel_id}` | 获取频道详情 |
| PATCH | `/api/v1/spaces/{space_id}/channels/{channel_id}` | 更新频道 |
| DELETE | `/api/v1/spaces/{space_id}/channels/{channel_id}` | 删除频道 |

### 邀请管理

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/spaces/{space_id}/invites` | 创建邀请 |
| GET | `/api/v1/spaces/{space_id}/invites` | 获取邀请列表 |
| GET | `/api/v1/spaces/{space_id}/invites/{invite_code}` | 获取邀请详情 |
| DELETE | `/api/v1/spaces/{space_id}/invites/{invite_code}` | 撤销邀请 |
| POST | `/api/v1/spaces/{space_id}/invites/{invite_code}/accept` | 接受邀请 |

### 封禁管理

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/spaces/{space_id}/bans` | 封禁用户 |
| GET | `/api/v1/spaces/{space_id}/bans` | 获取封禁列表 |
| GET | `/api/v1/spaces/{space_id}/bans/{user_id}` | 获取封禁详情 |
| DELETE | `/api/v1/spaces/{space_id}/bans/{user_id}` | 解封用户 |

---

## Interaction Service API（互动服务）

### 消息 Reaction

| 方法 | 路径 | 说明 |
|------|------|------|
| PUT | `/api/v1/interactions/conversations/{conversation_id}/messages/{message_id}/reactions/{emoji}` | 添加 Reaction |
| DELETE | `/api/v1/interactions/conversations/{conversation_id}/messages/{message_id}/reactions/{emoji}` | 删除 Reaction |
| GET | `/api/v1/interactions/conversations/{conversation_id}/messages/{message_id}/reactions` | 获取 Reaction 列表 |

#### PUT /api/v1/interactions/conversations/{conversation_id}/messages/{message_id}/reactions/{emoji}

**Response (204):** No Content

### 消息 Pin

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/interactions/conversations/{conversation_id}/pins` | Pin 消息 |
| GET | `/api/v1/interactions/conversations/{conversation_id}/pins` | 获取 Pin 列表 |
| DELETE | `/api/v1/interactions/conversations/{conversation_id}/pins/{message_id}` | Unpin 消息 |

#### POST /api/v1/interactions/conversations/{conversation_id}/pins

**Request:**
```json
{
  "message_id": "123456789012345678",
  "reason": "Important announcement"
}
```

### Thread

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/interactions/conversations/{conversation_id}/threads` | 创建 Thread |
| GET | `/api/v1/interactions/conversations/{conversation_id}/threads` | 获取 Thread 列表 |
| GET | `/api/v1/interactions/conversations/{conversation_id}/threads/{thread_id}` | 获取 Thread 详情 |
| POST | `/api/v1/interactions/conversations/{conversation_id}/threads/{thread_id}/messages` | 发送 Thread 消息 |
| GET | `/api/v1/interactions/conversations/{conversation_id}/threads/{thread_id}/messages` | 获取 Thread 消息列表 |

### 会话设置

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/interactions/conversations/{conversation_id}/settings` | 获取会话设置 |
| PATCH | `/api/v1/interactions/conversations/{conversation_id}/settings` | 更新会话设置 |

#### PATCH /api/v1/interactions/conversations/{conversation_id}/settings

**Request:**
```json
{
  "is_muted": true,
  "mute_until": "2026-06-17T10:00:00Z",
  "notification_level": "mentions"
}
```

---

## 通用约定

### ID 格式

所有 ID 使用 Snowflake BIGINT 格式，全局唯一且趋势递增。

### 时间格式

所有时间使用 ISO 8601 格式：`2026-06-16T10:00:00Z`

### 分页

使用 `limit` 和 `cursor` 参数进行分页：

```
GET /api/v1/contacts/friends?limit=20&cursor=123456789012345678
```

### 错误响应

```json
{
  "error": {
    "code": "not_found",
    "message": "Resource not found"
  }
}
```

### 认证

所有 API 需要在 Header 中携带 Bearer Token：

```
Authorization: Bearer <token>
```
