---
document_id: '6967331173081235461'
directory_id: '7073444337254924293'
title: url
full_path: /ukTMukTMukTM/uczNwUjL3cDM14yN3ATN
breadcrumb:
- Developer Guides
- Deprecated Guides
- Message Card
- Add card interaction
- Embedded interactive elements
- url
document_type: GuideDocumentType
updated_at: 2022-03-13T12:48:07Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uczNwUjL3cDM14yN3ATN
---

# url

**对象介绍：**<br>

​       **url对象用作多端差异跳转链接**

- 可用于button的multi_url字段，支持按键点击的多端跳转。
- 可用于lark_md类型text对象的href字段，支持超链接点击的多端跳转。



**字段定义：**<br>

| 字段        | 必须 | 类型   | 取值   | 说明           |
| ----------- | ---- | ------ | ------ | -------------- |
| url         | 是   | String | 超链接 | 默认跳转链接，参考注意事项-2  |
| android_url | 是   | String | 超链接 | 安卓端跳转链接 |
| ios_url     | 是   | String | 超链接 | ios端跳转链接  |
| pc_url      | 是   | String | 超链接 | pc端跳转链接   |



**样例结构**<br>

1. 结构样例

   ```json
   {
        "url": "https://www.baidu.com",
        "android_url": "https://developer.android.com/",
        "ios_url": "https://developer.apple.com/",
        "pc_url": "https://www.windows.com"
   }
   ```

   

2. 配置button元素multi字段样例

   ```json
    {
        "tag": "button",
        "text": {
            "tag": "lark_md",
            "content": "multi url"
        },
        "multi_url": {
            "url": "https://www.baidu.com",
            "android_url": "https://developer.android.com/",
            "ios_url": "https://developer.apple.com/",
            "pc_url": "https://www.windows.com"
        },
        "type": "primary"
    }
   ```



3. 配置text对象href字段样例

   ```json
    "text": {
        "tag": "lark_md",
        "content": "Button-[multi_url]($urlVal)",
        "href": {
            "urlVal": {
                "url": "https://www.baidu.com",
                "android_url": "https://developer.android.com/",
                "ios_url": "https://developer.apple.com/",
                "pc_url": "https://www.windows.com"
            }
        }
    }
   ```

   

**注意事项：**<br>

1. 在lark_md使用多端跳转时，在content中使用[链接名]绑定变量($urlVal)，在href字段中指定变量对应的url对象。支持放置多个跳转链接变量，注意变量名不能相同。<br>
2. url和各端url的关系：仅有url时才跳转url；各端url的优先级高于url，当二者同时存在时，优先跳转各端url；各端3个url必须同时配置才能生效。
