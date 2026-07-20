---
document_id: '6967261316286808069'
directory_id: '6907567266537472001'
title: 上传图片
full_path: /ukTMukTMukTM/uEDO04SM4QjLxgDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Message
- Images
- Upload Image
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:14Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uEDO04SM4QjLxgDN
---

# 上传图片

上传图片，获取图片的 image_key。


:::html
<md-alert type="warn">
需要启用机器人能力;图片大小不能超过10MB
</md-alert>
:::


## 请求
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/image/v4/put/</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::

### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 18%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
       <md-th style="width: 15%;">必填</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant-desc">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)
</md-td>
</md-tr>
     <md-tr>
      <md-td>Content-Type</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      <md-td>**示例值**："multipart/form-data; boundary=---7MA4YWxkTrZu0gW"</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

### 请求体
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
image | binary| 必填 | 图片文件 ||图片二进制数据|
image_type | string| 必填 | 图片类型，"message"表示消息图片，"avatar"表示头像。 ||message|


### 请求体示例

```HTTP
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="image";
Content-Type: application/octet-stream

image binary data
---7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="image_type";

message
---7MA4YWxkTrZu0gW
```


## 响应
### 响应体
参数 |类型| 说明 
-- |--| -- 
code |int| 返回码，非 0 表示失败
msg	|string| 返回码描述
data | - | -
&emsp;image_key|string|图片 key


### 响应体示例
```json
{
	"code": 0,
	"data": {
		"image_key": "fd3a475a-0c27-4071-a9a6-8712b84b0cb6"
	},
	"msg": "ok"
}
```


### 图片上传 Demo - Python

```Python
import requests

def upload_image(image_path):
    """上传图片
    Args:
        image_path: 文件上传路径
        image_type: 图片类型
    Return
        {
            "ok": true,
            "image_key": "xxx",
            "url": "https://xxx"
        }
    Raise:
        Exception
            * file not found
            * request error
    """

    with open(image_path, 'rb') as f:
        image = f.read()
    resp = requests.post(
        url='https://open.larksuite.com/open-apis/image/v4/put/',
        headers={'Authorization': "Bearer XXXX"},
        files={
            "image": image
        },
        data={
            "image_type": "message"
        },
        stream=True)


    resp.raise_for_status()
    content = resp.json()
    print(content)
    if content.get("code") == 0:
        return content
    else:
        raise Exception("Call Api Error, errorCode is %s" % content["code"])

```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
