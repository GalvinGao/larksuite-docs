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
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td><md-version>V3.47.0+</md-version></md-td>
      <md-td><md-version>V3.47.0+</md-version></md-td>
      <md-td>**x**</md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**x**</md-td>
      <md-td>**x**</md-td>
      <md-td>**x**</md-td>
      <md-td>/</md-td>
	</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 15%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                channelType
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
飞书：
              
["wx", "wx_timeline"]
              
Lark：
              
["system"]
            </md-td>
            <md-td>
                指定分享的渠道

**示例值**：["wx", "wx_timeline"]

**可选值**：
- `wx`：微信分享
- `wx_timeline`：微信朋友圈分享
- `system`：系统分享
  - Lark[V4.5.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持

wx、wx_timeline 仅飞书支持，Lark 请使用 system 渠道。              

<md-alert type="tip" icon="none">
**注意:** 如果分享渠道数等于 1，则直接发起分享到渠道，不需要经过分享面板。
</md-alert>   
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                contentType
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                指定内容的类型。目前支持文本、图片、URL 分享。

**示例值**：text

**可选值**：
- `text`：文本
- `image`：图片
- `url`：在线链接
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                title
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                分享标题。

**示例值**：share message title
<md-alert type="tip" icon="none">
**注意:** 仅在 contentType = "url" 下生效，且为必选参数。
</md-alert>  
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                content
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                分享的内容。

**示例值**：share this message to your friends.
<md-alert type="tip" icon="none">
**注意:** 如果 contentType = "text" 则该字段不能为空。
</md-alert>  
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                image
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                分享的图片的 Base64 编码。

<md-alert type="tip" icon="none">
  **注意**:
- 如果 contentType = "image" 则该字段不能为空。
- 如果使用前端转换工具将图片转成base64编码时，可能会在base64编码开头携带图片格式信息(**比如**:data:image/png;base64,)。对于这种情况，需要在为image参数赋值时去掉图片格式信息。
</md-alert>  
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                url
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                在线 URL。

**示例值**：https://www.larksuite.com/
<md-alert type="tip" icon="none">
**注意:** 如果 contentType = "url" 则该字段不能为空。
</md-alert>  
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


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

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">错误码</md-th>
      <md-th style="width: 40%;">描述</md-th>
      <md-th style="width: 40%;">排查建议</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>101</md-td>
      <md-td>应用未安装</md-td>
      <md-td>/</md-td>
   </md-tr>
    
    </md-tbody>
</md-table>
:::

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
