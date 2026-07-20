---
document_id: '7074952334766014470'
directory_id: '7073442394955628549'
title: 搜索并查看部门信息
full_path: /home/mass-messaging-to-designated-departments/search-and-view-departmental-information
breadcrumb:
- Home
- Send Messages to Specified Dept. Members
- Search and view departmental information
document_type: GuideDocumentType
updated_at: 2023-05-16T03:11:38Z
source_url: https://open.larksuite.com/document/home/mass-messaging-to-designated-departments/search-and-view-departmental-information
---

# 搜索并查看部门信息

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dcf84a883241663ec8fdff4f617a2383_2IzAQGJMz4.png)

**根据部门名称```department_name ``` ，以用户身份搜索部门**
```bash
  curl -X POST -H '{"Authorization":"Bearer {{{user_access_token}}}"}' -H 'Content-Type: application/json' 
  https://open.larksuite.com/open-apis/contact/v3/departments/search -d '
  {"query": "{{{department_name}}}"}'
``` 
返回结果如下：

```json 
{
  "code": 0,
  "data": {
    "has_more": false,
    "items": [
      {
        "chat_id": "oc_xxx",
        "department_id": "xxx",
        "i18n_name": {
          "en_us": "Lark Department",
          "ja_jp": "",
          "zh_cn": ""
        },
        "leader_user_id": "ou_xxx",
        "member_count": 54,
        "name": "Lark Department",
        "open_department_id": "od-xxx",
        "order": "1",
        "parent_department_id": "od-xxx",
        "status": {
          "is_deleted": false
        }
      }
    ]
  },
  "msg": "Success"
} 
``` 


找到其中的```name ```，查看是否是我们想要的部门，如果是的话，找到```department_id```字段，即我们需要的部门id；否则，找到```parent_department_id ```字段，即该```name```部门对应的父部门的```open_department_id```。这是我们需要查看部门信息

**查看部门信息**

根据部门```open_department_id```，以租户应用身份访问

```bash 
 curl -H '{"Authorization":"Bearer {{{tenant_access_token}}}"}' https://open.larksuite.com/open-apis/contact/v3/departments/{{{open_department_id}}}
``` 
会返回类似如下的内容： 

```json 
{
  "code": 0,
  "data": {
    "department": {
      "chat_id": "oc_xxx",
      "department_id": "xxx",
      "i18n_name": {
        "en_us": "XXX",
        "ja_jp": "",
        "zh_cn": ""
      },
      "leader_user_id": "ou_xxx",
      "member_count": 197,
      "name": "XXX",
      "open_department_id": "od-xxx",
      "order": "40000",
      "parent_department_id": "xxx",
      "status": {
        "is_deleted": false
      }
    }
  },
  "msg": "success"
} 
``` 
找到其中的```name ```，查看是否是我们想要的部门，如果是的话，找到```department_id```字段，即我们需要的部门id；否则，找到```parent_department_id```字段，即该name部门对应的父部门的```open_department_id```。这时我们继续查看部门信息，直到找到我们需要的部门id。
