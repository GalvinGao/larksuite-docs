---
document_id: '7152073576123793413'
directory_id: '7148629329634656262'
title: 多维表格记录变更
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/event/list/bitable-record-changed
breadcrumb:
- Server API
- Docs
- Space
- Event
- Docs event list
- Bitable Record Changed
document_type: GuideDocumentType
updated_at: 2022-10-08T09:37:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/event/list/bitable-record-changed
---

# 多维表格记录变更
:::html
<md-alert type="tip">
了解事件订阅的使用场景和配置流程，请点击查看 [事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)
</md-alert>
:::
被订阅的多维表格记录发生变更将会触发此事件，**公式字段的值变化不会触发事件**。

## 概述

| 基本 |  |
| --- | --- |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br><md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm> |

如何订阅文档请点击查看 [订阅云文档事件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/subscribe)。

## 支持的记录变更类型

| 变更类型         | action           |
| --------- | --------------- |
|新增行记录 | `record_added` | 
|删除行记录 | `record_deleted` | 
|修改行记录 | `record_edited` | 


回调结构中的 `field_value` 字段为 JSON 序列化后的字符串，序列化前的结构请查看 [数据结构](/document/uAjLw4CM/ukTMukTMukTM/bitable/development-guide/bitable-structure)

## 回调示例
``` json
{
  "schema": "2.0", // 事件格式的版本。无此字段的即为1.0
  "header": {
    "event_id": "85ec04b2f6cff27a1bf249b26fbabcef", // 事件的唯一标识
    "token": "L3bzVf0sz7qb9XoTavpsHe0uJv7abcef", // 即 Verification Token 
    "create_time": "1652677457000", // 事件发送的时间
    "event_type": "drive.file.bitable_record_changed_v1", // 事件类型
    "tenant_key": "736588c9260abcef", // 企业标识 
    "app_id": "cli_a00c8400a7babcef" // 应用ID
  },
  "event": {
    "action_list": [
      {
        "action": "record_edited", // 变更类型：修改记录
        "after_value": [
          {
            "field_id": "fld9Eabcef", // 发生变更的字段 ID
            "field_value": "666" // 发生变更后的字段值
          },
          {
            "field_id": "fldqaabcef",// 发生变更的字段 ID
            "field_value": "[{\"type\":\"text\",\"text\":\"变更后的值\"}]" // 发生变更后的字段值
          }
        ],
        "before_value": [
          {
            "field_id": "fld9Eabcef", // 发生变更的字段 ID
            "field_value": "123" // 发生变更前的字段值
          },
          {
            "field_id": "fldqaabcef", // 发生变更的字段 ID
            "field_value": "[{\"type\":\"text\",\"text\":\"变更前的值\"}]" // 发生变更前的字段值
          }
        ],
        "record_id": "rec9sabcef" // 发生变更的记录 ID
      },
      {
        "action": "record_added", // 变更类型：新增记录
        "after_value": [
          {
            "field_id": "fld9Eabcef", // 发生变更的字段 ID
            "field_value": "666" // 发生变更后的字段值
          },
          {
            "field_id": "fldqaabcef",// 发生变更的字段 ID
            "field_value": "[{\"type\":\"text\",\"text\":\"新增记录字段值\"}]" // 发生变更后的字段值
          }
        ],
        "record_id": "rec9sabcef" // 新增的记录 ID
      },
      {
        "action": "record_deleted", // 变更类型：删除记录
        "before_value": [
          {
            "field_id": "fld9Eabcef", // 发生变更的字段 ID
            "field_value": "666" // 发生变更前的字段值
          },
          {
            "field_id": "fldqaabcef",// 发生变更的字段 ID
            "field_value": "[{\"type\":\"text\",\"text\":\"删除记录前字段值\"}]" // 发生变更前的字段值
          }
        ],
        "record_id": "rec9sabcef" // 删除的记录 ID
      }
    ],
    "file_token": "bascnItn6oHUSEL8RDUdF6abcef", // 多维表格 token
    "file_type": "bitable", // 文件类型，即 bitable
    "operator_id": { // 操作人
      "open_id": "ou_9bc587355789fc049904ae7c736abcef",
      "union_id": "on_8f71e0224c012a0365ad4e3c733abcef",
      "user_id": "638abcef"
    },
    "subscriber_id_list": [ // 订阅的用户列表
      {
        "open_id": "ou_9bc587355789fc049904ae7c736abcef",
        "union_id": "on_8f71e0224c012a0365ad4e3c733abcef",
        "user_id": "638abcef"
      }
    ],
    "table_id": "tblOaqBWfGeabcef" // 发生变更的数据表 ID
  }
}
```
