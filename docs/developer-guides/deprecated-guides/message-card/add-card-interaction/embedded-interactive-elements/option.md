---
document_id: '6967331173081612293'
directory_id: '7073444337254924293'
title: option
full_path: /ukTMukTMukTM/ugzNwUjL4cDM14CO3ATN
breadcrumb:
- Developer Guides
- Deprecated Guides
- Message Card
- Add card interaction
- Embedded interactive elements
- option
document_type: GuideDocumentType
updated_at: 2022-03-13T12:48:07Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugzNwUjL4cDM14CO3ATN
---

# option

**对象介绍：**<br>

- 作为selectMenu的选项对象
- 作为overflow的选项对象



**字段定义：**<br>

| 字段      | 必须 | 类型   | 取值     | 说明                                               |
| --------- | ---- | ------ | -------- | -------------------------------------------------- |
| text      | 否   | Struct | [text](/document/ukTMukTMukTM/uUzNwUjL1cDM14SN3ATN)对象 | 选项显示内容，非待选人员时必填                     |
| value     | 是   | String |          | 选项选中后返回业务方的数据                         |
| url       | 否   | String |          | *仅支持overflow，跳转指定链接，和multi_url字段互斥 |
| multi_url | 否   | Struct | [url](/document/ukTMukTMukTM/uczNwUjL3cDM14yN3ATN)对象  | *仅支持overflow，跳转对应链接，和url字段互斥       |



**样例结构**<br>

1. 基础使用结构（用于selectMenu的选项模式，overflow选项）

   ```json
    {
        "text": {
            "tag": "plain_text",
            "content": "Option"
        },
        "value": "option"
    }
   ```

   

2. selectMenu的选人模式，指定待选人员，只需指定待选人的openId

   ```json
    {
        "tag": "select_person",
        "placeholder": {
            "tag": "plain_text",
            "content": "person-target group"
        },
        "options": [
            {
                "value": "ou_2*******************ede3"
            },
            {
                "value": "ou_f********************3d1"
            }
        ]
    }
   ```



3.overflow指定选项跳转链接的option对象结构

```json
{
    "text": {
        "tag": "plain_text",
        "content": "Option-1"
    },
    "value": "option-1",
    "multi_url": {
        "url": "https://www.baidu.com",
        "android_url": "https://developer.android.com/",
        "ios_url": "https://developer.apple.com/",
        "pc_url": "https://www.windows.com"
    }
}
```





**注意事项：**<br>

1. overflow的选项存在跳转链接时，点击选项优先跳转链接，否则向业务方回传选项的value。
