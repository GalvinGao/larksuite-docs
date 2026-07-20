---
document_id: '6967331173082005509'
directory_id: '6907567269107515394'
title: sheet支持写入数据类型
full_path: /ukTMukTMukTM/ugjN1UjL4YTN14CO2UTN
breadcrumb:
- Server API
- Docs
- Appendix
- Data Types Supported By Sheets
document_type: GuideDocumentType
updated_at: 2022-03-11T12:23:59Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugjN1UjL4YTN14CO2UTN
---

# sheet支持写入的数据类型

## 字符串

```
"string"
```

## 数字

```
123
```

## 链接

### 无文本链接

```
"http://www.dd.com"
```

### 带文本的链接

```json
{
    "text": "文本",
    "link": "http://www.dd.com",
    "type": "url"
}
```

## 邮箱

```
"aaa@aa.com"
```

## @人

只支持@同租户的用户;单次请求最多支持同时@50人；

notify:是否发送Lark消息，没有阅读权限的用户不会收到Lark消息；

grantReadPermission:是否赋予该用户阅读权限（仅在独立表格中支持该字段）；

textType:指定text字段的传入的内容，可选email，openId，unionId；

text:需要@的人的信息，由textType指定

```json
{
    "type": "mention",
    "text": "aaa@aa.com",
    "textType": "email",
    "notify": true,
    "grantReadPermission": true
}
```

## 公式

text字段为对应的公式，暂不支持跨表引用公式（IMPORTRANGE）

```json
{
    "type": "formula",
    "text": "=A1"
}
```

## @文档

textType:固定为fileToken

text:文档token

objType:文档类型，可选sheet,doc,slide,bitable,mindnote

```json
{
    "type": "mention",
    "textType": "fileToken",
    "text": "shtxxxx",
    "objType": "sheet"
}
```

## 下拉列表

values为数组，可填bool,string,number类型。string类型数据不能包含","。使用前需要先使用[设置下拉列表](/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/set-dropdown)接口设置下拉列表。

```json
{
    "type": "multipleValue",
    "values": [
        1,
        "test"
    ]
}
```

## 局部样式

### segmentStyle

bold：是否加粗

Italic：是否斜体

strikeThrough：是否含有删除线

underline：是否含有下划线

foreColor：字体颜色，16进制rgb颜色

fontSize：字体大小，最小字号9，最大字号36

```json
{
    "bold": true,
    "italic": true,
    "strikeThrough": true,
    "underline": true,
    "foreColor": "#ff00ff",
    "fontSize": 30
}
```

### 写入带局部样式的数据

#### 字符串

```json
{
    "text": "string",
    "type": "text",
    "segmentStyle": {
        "bold": true,
        "italic": true,
        "strikeThrough": true,
        "underline": true,
        "foreColor": "#ff00ff",
        "fontSize": 20
    }
}
```

#### 数字

不支持局部样式

#### 链接

支持对链接中不同字符串写入不同样式。foreColor字段不生效，固定为蓝色；underline字段不生效，固定含有下划线；

```json
{
    "text": "文本",
    "link": "http://www.dd.com",
    "type": "url",
    "texts": [
        {
            "text": "文",
            "segmentStyle": {
                "bold": true,
                "italic": true,
                "strikeThrough": true,
                "underline": true,
                "foreColor": "#ffffff",
                "fontSize": 20
            }
        },
        {
            "text": "本",
            "segmentStyle": {
                "bold": true,
                "italic": false,
                "strikeThrough": true,
                "underline": true,
                "foreColor": "#ffffff",
                "fontSize": 10
            }
        }
    ]
}
```

#### 邮箱

支持对邮箱中不同字符串写入不同样式。foreColor字段不生效，固定为蓝色；underline字段不生效，固定含有下划线；

```json
{
    "type": "url",
    "text": "aa@bytedance.com",
    "texts": [
        {
            "text": "aa",
            "segmentStyle": {
                "bold": true,
                "italic": true,
                "strikeThrough": true,
                "underline": true,
                "foreColor": "#ffffff",
                "fontSize": 20
            }
        },
        {
            "text": "@bytedance.com",
            "segmentStyle": {
                "bold": true,
                "italic": false,
                "strikeThrough": true,
                "underline": true,
                "foreColor": "#ffffff",
                "fontSize": 10
            }
        }
    ]
}
```

#### @人

只支持@同租户的用户;单次请求最多支持同时@50人

notify:是否发送Lark消息，没有阅读权限的用户不会收到Lark消息；

grantReadPermission:是否赋予该用户阅读权限（仅在独立表格中支持该字段）；

textType:指定text字段的传入的内容，可选email，openId，unionId；

text:需要@的人的信息，由textType指定；

仅支持对整体设置局部样式；foreColor字段不生效，固定为蓝色；

```json
{
    "type": "mention",
    "text": "aaa@aa.com",
    "textType": "email",
    "notify": true,
    "grantReadPermission": true
    "segmentStyle": {
        "bold": true,
        "italic": true,
        "strikeThrough": true,
        "underline": true,
        "foreColor": "#ff00ff",
        "fontSize": 30
    }
}
```

#### @文档

textType:固定为fileToken

text:文档token

objType:文档类型，可选sheet,doc,slide,bitable,mindnote

foreColor字段不生效，固定为蓝色；

```json
{
    "type": "mention",
    "textType": "fileToken",
    "text": "shtxxxx",
    "objType": "sheet",
    "segmentStyle": {
        "bold": true,
        "italic": true,
        "strikeThrough": true,
        "underline": true,
        "foreColor": "#ff00ff",
        "fontSize": 30
    }
}
```

#### 下拉列表

不支持局部样式


