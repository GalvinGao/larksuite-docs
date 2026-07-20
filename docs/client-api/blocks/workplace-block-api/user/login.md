---
document_id: '7180270043523022854'
directory_id: '7180165099250991109'
title: login
full_path: /uAjLw4CM/uYjL24iN/block/api/user/login
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- User
- login
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/user/login
---

# login

获取临时登录凭证。

## 输入

param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)，无额外扩展属性。

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
                code
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                临时登录凭证，有效期 3 分钟，只能使用一次
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
  	"item": "login"
  },
    "openDetail": 1, 
    "title": "login", 
    "list_page_url": "https://applink.larksuite.com/client/block/workplace/open?appId=cli_a00834ec56f8d01b%26blockTypeId=blk_610a40455f800004c32b6bb6%26sourceData=%7B%22tab%22%3A%22api%22%2C%22item%22%3A%22login%22%7D", 
    "min_lk_ver": {
        "pc": "5.10.0", 
        "mobile": "5.11.0"
    },
    "blockEntity": {
        "sourceData": {
            "type": "api",
            "item": "login",
            "isNew": true
        },
  		"blockID": "mock-block"
    }
}
</md-block-api>

:::
```js
tt.login({
  success (res) {
    console.log('login 调用成功', res.code);
  },
  fail (res) {
    console.log('login 调用失败', res.errMsg);
  },
  complete (res) {
    console.log('login 调用结束', res.errMsg);
  } 
});
```

### 返回示例

```json
{
  "errMsg": "login:ok",
  "code": "46c725e3d6fa545b"
}
```
