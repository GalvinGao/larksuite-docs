---
document_id: '7180269945548308485'
directory_id: '7180165099250991109'
title: getUserInfo
full_path: /uAjLw4CM/uYjL24iN/block/api/user/getuserinfo
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- User
- getUserInfo
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/user/getuserinfo
---

# getUserInfo



获取已登录用户的基本信息或特殊信息。
:::html
<md-alert type="tip">
  调用前确保已经调用 tt.login 接口成功登录。
</md-alert>
:::

## 输入

param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)，扩展属性描述：

| **名称**          | **数据类型**  | **是否必填** | **默认值** | **描述**     |
| --------------- | ------- | ------- | ------ | ---------- | ---------- |
| withCredentials | boolean | 否   | false      | 是否需要返回敏感数据 | PC 端 1.0.0 |

## 输出

success 函数返回对象参数扩展属性：

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                userInfo
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                用户信息
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    nickName
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户昵称
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    avatarUrl
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户头像
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    gender
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户性别

**可选值**：
- `''`：未知
- `male`：男性
- `female`：女性
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    country
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户所在国家或地区
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    province
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户所在省份
<md-alert type="tip" >
该字段已下线
</md-alert> 
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    city
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户所在城市
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    language
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                所在地区所用的语言

**可选值**：
- `en_US`：英文
- `zh_CN`：中文
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                rawData
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                userInfo 的 JSON 字符串形式
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                signature
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用于校验用户信息是否被篡改（withCredentials 需为 true）
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                encryptedData
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                包括敏感信息（如 openId）在内的已加密用户数据（withCredentials 需为 true）
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                iv
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                加密算法参数（withCredentials 需为 true）
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码

### 调用示例
:::html
<md-block-api>
{
  "sourceData":{
  	"tab": "api",
  	"item": "getUserInfo"
  },
    "openDetail": 1, 
    "title": "getUserInfo", 
    "list_page_url": "https://applink.larksuite.com/client/block/workplace/open?appId=cli_a00834ec56f8d01b%26blockTypeId=blk_610a40455f800004c32b6bb6%26sourceData=%7B%22tab%22%3A%22api%22%2C%22item%22%3A%22getUserInfo%22%7D", 
    "min_lk_ver": {
        "pc": "5.10.0", 
        "mobile": "5.11.0"
    },
    "blockEntity": {
        "sourceData": {
            "type": "api",
            "item": "getUserInfo",
            "isNew": true
        },
  		"blockID": "mock-block"
    }
}
</md-block-api>
:::
```js
tt.getUserInfo({
  withCredentials: true,
  success (res) {
    console.log('getUserInfo 调用成功', res.userInfo);
  },
  fail (res) {
    console.log('getUserInfo 调用失败');
  },
  complete (res) {
    console.log('getUserInfo 调用结束', res.errMsg);
  } 
});
```

### 返回示例

```json
{
  "errMsg": "getUserInfo:ok",
  "encryptedData": "Otonx45hSc9CnIAOK8jFKfpnvM+oEIti8h9oKIRyeMOmAnmjT5vMEQtK8itF/leoTKefOo6I0gKUj4fkRkGpLSAB5CwAT5FpcX93O7WTcfatZ0G+wecXGIrpNI8tm4KpElNNUSRsXw2fK+ll/wP87S4KsM6HB+uLObEo+3JW7D0WwzyIBZ0Y2OgeQ0lVrZayn66HkEx3+ZQIfR6lqcHYmZHBhMJ4mKXJPgwervA2pTc=",
  "iv": "5ef0f7127289c52cf09f58a078f1f709",
  "rawData": "{"nickName":"潘二铭","avatarUrl":"https://p6-lark-file.byteimg.com/img/lark.avatar/437b7334-de49-40f8-b3e2-69ee357e40bg~72x72.png","gender":"male","city":"","province":"","country":"","language":"","i18nName":{"en_us":"Erming Pan","ja_jp":"","zh_cn":""}}",
  "signature": "afee7903abad25f71d9248467e2202a853ef6a14",
  "userInfo": {
    "avatarUrl": "https://p6-lark-file.byteimg.com/img/lark.avatar/437b7334-de49-40f8-b3e2-69ee357e40bg~72x72.png",
    "city": "",
    "country": "",
    "gender": "male",
    "i18nName": {
      "en_us": "Erming Pan",
      "ja_jp": "",
      "zh_cn": ""
    },
    "language": "",
    "nickName": "潘二铭",
    "province": ""
  }
}
```
