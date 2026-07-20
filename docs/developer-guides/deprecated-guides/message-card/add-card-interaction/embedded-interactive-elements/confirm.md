---
document_id: '6967331158355034118'
directory_id: '7073444337254924293'
title: confirm
full_path: /ukTMukTMukTM/ukzNwUjL5cDM14SO3ATN
breadcrumb:
- Developer Guides
- Deprecated Guides
- Message Card
- Add card interaction
- Embedded interactive elements
- confirm
document_type: GuideDocumentType
updated_at: 2022-03-13T12:48:04Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukzNwUjL5cDM14SO3ATN
---

# confirm

**对象介绍：**<br>

​       **用于交互元素的二次确认**

​	   弹框默认提供确定和取消的按钮，无需开发者手动配置。



**字段定义：**<br>

| 字段    | 必须 | 类型   | 取值                          | 说明     |
| ------- | ---- | ------ | ----------------------------- | -------- |
| title   | 是   | Struct | [text](/document/ukTMukTMukTM/uUzNwUjL1cDM14SN3ATN)对象（仅支持"plain_text") | 弹框标题 |
| text | 是   | Struct | [text](/document/ukTMukTMukTM/uUzNwUjL1cDM14SN3ATN)对象（仅支持"plain_text") | 弹框内容 |



**使用介绍：**<br>

| 元素       | 字段    | 效果                     |
| ---------- | ------- | ------------------------ |
| button     | confirm | 点击button，弹出确认弹框 |
| selectMenu | Struct  | 点击选项，弹出确认弹框   |
| overFlow   | confirm | 点击选项，弹出确认弹框   |
| datePicker | confirm | 点击选项，弹出确认弹框   |



**样例结构**<br>

1. 基础结构

   ```json
	"confirm":{
		"title":{
			"tag":"plain_text",
			"content":"title"
		},
		"text":{
			"tag":"plain_text",
			"content":"content"
		}
	}
   ```

   

2. 交互元素配置confirm的样例

   ```json
    {
        "tag": "date_picker",
        "placeholder": {
            "tag": "plain_text",
            "content": "please select date(with confirm)"
        },
        "confirm": {
            "title": {
                "tag": "plain_text",
                "content": "title"
            },
            "text": {
                "tag": "plain_text",
                "content": "content"
            }
        }
    }
   ```



**样例效果**<br>


![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4893dcbb8874b05a49f5dbef84b55e4e.png)



**注意事项：**<br>

1. 弹窗的确认和取消文案，中文环境下默认为：”确认"/"取消"，英文环境下默认为 "confirm"/"cancel" .。

