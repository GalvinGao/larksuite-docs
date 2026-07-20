---
document_id: '7531947220879556614'
directory_id: '7072190414392360966'
title: 多维表格中添加子记录
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/add-a-sub-record-in-a-base-table
breadcrumb:
- Server API
- Docs
- Base
- Record
- Add a sub-record in a Base table
document_type: GuideDocumentType
updated_at: 2025-07-28T02:01:30Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/add-a-sub-record-in-a-base-table
---

# 多维表格中添加子记录

在多维表格中添加子记录，本质上是通过在子记录和父记录之间设置单向或双向关联字段来实现的。子记录会通过关联字段，映射至父记录，从而建立联系。本文档介绍如何通过 OpenAPI，在多维表格数据表中为一个记录添加子记录。
## 前提条件

你已创建了一个多维表格，且在多维表格中，已有一条记录作为父记录。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ed1bf13eaffaae51a96a153f4faa80be_JMUAqSNi8R.png?height=866&lazyload=true&maxWidth=500&width=1050)

## 流程说明

添加子记录的整体流程如下所示：
1. 调用[新增字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/create)接口，在多维表格中创建一个单向或双向关联字段，用于建立记录之间的关联关系。
1. 调用[查询记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search)接口，获取已有的父记录的 ID。
1. 调用[新增记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/create)接口，新增一条记录作为子记录，并确保该记录关联了已有的父记录。
1. 调用[列出字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list)接口，获取“单向关联字段”的字段 ID。
1. 调用[更新视图](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/patch)接口，将 `hierarchy_config` 参数中的 `field_id` 设为“单向关联字段”的字段 ID，以更新表格视图的层级结构样式。

## 操作步骤

本小节以如下的一条记录为例，介绍如何为该记录添加子记录。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ed1bf13eaffaae51a96a153f4faa80be_7JxlJ32vkA.png?height=866&lazyload=true&maxWidth=500&width=1050)

1. 调用[新增字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/create)接口，在多维表格数据表中创建一个单向关联字段，用于建立记录之间的关联关系。请求体如下所示：
    
    
    ```json
    {
      "field_name": "单向关联字段",
      "property": {
        "multiple": true,
        "table_id": "tblY2ha8xGSabcef"
      },
      "type": 18
    }
    ```
   
   若调用成功，多维表格将呈现如下效果：
   
   ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5e8c72d4b11abd8d1f32d87651f33286_R5Tgu9HvDr.png?height=865&lazyload=true&maxWidth=500&width=1163)

2. 调用[查询记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search)接口，获取已有的父记录的 ID。请求体为空即可：

    ```json
    {}
    ```
   
   若调用成功，接口将返回如下结构数据，其中 `rec9k8PAbR` 即为父记录的 ID。
   
    ```json
    {
      "code": 0,
      "data": {
        "has_more": false,
        "items": [
          {
            "fields": {
              "单向关联字段": {},
              "文本": [
                {
                  "text": "父记录",
                  "type": "text"
                }
              ]
            },
            "record_id": "rec9k8PAbR"   // 父记录 ID
          },
          {
            "fields": {
              "单向关联字段": {}
            },
            "record_id": "recb9nHBYR"
          },
          {
            "fields": {
              "单向关联字段": {}
            },
            "record_id": "recwG1hh0g"
          },
          {
            "fields": {
              "单向关联字段": {}
            },
            "record_id": "recBlfgGRO"
          },
          {
            "fields": {
              "单向关联字段": {}
            },
            "record_id": "recKZHTepH"
          },
          {
            "fields": {
              "单向关联字段": {}
            },
            "record_id": "recDZXc9fs"
          },
          {
            "fields": {
              "单向关联字段": {}
            },
            "record_id": "recX9dPV90"
          },
          {
            "fields": {
              "单向关联字段": {}
            },
            "record_id": "rec6cq2RIk"
          },
          {
            "fields": {
              "单向关联字段": {}
            },
            "record_id": "recuK6kUA1"
          },
          {
            "fields": {
              "单向关联字段": {}
            },
            "record_id": "recLjQD5Eo"
          }
        ],
        "total": 10
      },
      "msg": "success"
    }
    ```
   
1. 调用[新增记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/create)接口，新增一条记录作为子记录，并确保该记录关联了已有的父记录。请求体示例如下所示：

    ```json
    {
      "fields": {
        "单向关联字段": [
          "rec9k8PAbR"
        ],
        "文本": "子记录"
      }
    }
    ```
        
    若调用成功，子记录将出现在数据表末尾，效果如下所示：
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1f27f151e95e8011daf5b50d5a6ce2fd_rdc40Je0KC.png?height=859&lazyload=true&maxWidth=500&width=1163)

2. 调用[列出字段](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list)接口，获取“单向关联字段”的字段 ID。若调用成功，接口将返回如下结构数据，其中 `fldfASqam8` 即为“单向关联字段”的字段 ID。
    
    
    ```json
    {
      "code": 0,
      "data": {
        "has_more": false,
        "items": [
          {
            "field_id": "fldplsqa97",
            "field_name": "文本",
            "is_hidden": false,
            "is_primary": true,
            "property": null,
            "type": 1,
            "ui_type": "Text"
          },
          {
            "field_id": "fldfASqam8",   // 单向关联字段的字段 ID
            "field_name": "单向关联字段",
            "is_hidden": false,
            "is_primary": false,
            "property": {
              "multiple": true,
              "table_id": "tblmyHKpQG3k1kSD",
              "table_name": "数据表"
            },
            "type": 18,
            "ui_type": "SingleLink"
          }
        ],
        "page_token": "fldfASqam8",
        "total": 2
      },
      "msg": "success"
    }
    ```
    
    
1. 调用[更新视图](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/patch)接口，参考如下请求示例，将 `hierarchy_config` 参数中的 `field_id` 设为“单向关联字段”的字段 ID，以更新表格视图的层级结构样式。


    ```json
    {
      "property": {
        "hierarchy_config": {
          "field_id": "fldfASqam8"
        }
      },
      "view_name": "表格"
    }
    ```
    
    若调用成功，子记录位置将位于父记录之下，最终效果如下所示：
    
     ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/80a089503898e472752511edf43a1c70_TfHQUYJqVr.png?height=866&lazyload=true&maxWidth=500&width=1161)
