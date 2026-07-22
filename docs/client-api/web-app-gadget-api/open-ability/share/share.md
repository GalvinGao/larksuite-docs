---
document_id: '7073692582769377286'
directory_id: '7073448001569914886'
title: share
full_path: /uYjL24iN/ugDM04COwQjL4ADN/thirdShare
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Share
- share
document_type: GuideDocumentType
updated_at: 2024-01-05T03:46:55Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDM04COwQjL4ADN/thirdShare
---

# share(Object object)

分享内容到三方应用。

:::html
<md-alert type="tip">
注意事项：
- 微信朋友圈仅支持图片分享。
</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.47.0+</md-version> | <md-version>V3.47.0+</md-version> | **x** | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **x** | **x** | **x** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| channelType | string[] | 否 | 飞书：<br>["wx", "wx_timeline"]<br>Lark：<br>["system"] | 指定分享的渠道<br>**示例值**：["wx", "wx_timeline"]<br>**可选值**：<br>- `wx`：微信分享<br>- `wx_timeline`：微信朋友圈分享<br>- `system`：系统分享<br>- Lark[V4.5.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>wx、wx_timeline 仅飞书支持，Lark 请使用 system 渠道。<br><md-alert type="tip" icon="none"><br>**注意:** 如果分享渠道数等于 1，则直接发起分享到渠道，不需要经过分享面板。<br></md-alert> |
| contentType | string | 是 |  | 指定内容的类型。目前支持文本、图片、URL 分享。<br>**示例值**：text<br>**可选值**：<br>- `text`：文本<br>- `image`：图片<br>- `url`：在线链接 |
| title | string | 否 |  | 分享标题。<br>**示例值**：share message title<br><md-alert type="tip" icon="none"><br>**注意:** 仅在 contentType = "url" 下生效，且为必选参数。<br></md-alert> |
| content | string | 否 |  | 分享的内容。<br>**示例值**：share this message to your friends.<br><md-alert type="tip" icon="none"><br>**注意:** 如果 contentType = "text" 则该字段不能为空。<br></md-alert> |
| image | string | 否 |  | 分享的图片的 Base64 编码。<br><md-alert type="tip" icon="none"><br>**注意**:<br>- 如果 contentType = "image" 则该字段不能为空。<br>- 如果使用前端转换工具将图片转成base64编码时，可能会在base64编码开头携带图片格式信息(**比如**:data:image/png;base64,)。对于这种情况，需要在为image参数赋值时去掉图片格式信息。<br></md-alert> |
| url | string | 否 |  | 在线 URL。<br>**示例值**：https://www.larksuite.com/<br><md-alert type="tip" icon="none"><br>**注意:** 如果 contentType = "url" 则该字段不能为空。<br></md-alert> |



## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
tt.share({
    channelType: [
        "wx",
        "wx_timeline",
        "system"
    ],
    contentType: "url",
    title: "Lark官网",
    url: "https://www.larksuite.com/",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`share fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "share:ok"
}
```

## 错误码

`fail`返回对象中会包含errCode属性，代表错误码，具体错误码列表参见：

| 错误码 | 描述 | 排查建议 |
| --- | --- | --- |
| 101 | 应用未安装 | / |


## 注意
为image参数赋值时，通过转换工具将图片转换的base64编码开头如果带有图片格式信息(**例如**：data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGUAAABZCAY...)。需要**去掉图片格式信息**，image参数赋值为：iVBORw0KGgoAAAANSUhEUgAAAGUAAABZCAY... ，例如：
```js

tt.share({
    channelType: [
        "wx",
        "wx_timeline",
        "system"
    ],
    contentType: "image",
    image: "iVBORw0KGgoAAAANSUhEUgAAAGUAAABZCAY...",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`share fail: ${JSON.stringify(res)}`);
    }
});

```
