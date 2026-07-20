---
document_id: '7180270043523121158'
directory_id: '7180165099251073029'
title: request
full_path: /uAjLw4CM/uYjL24iN/block/api/network/request
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Network
- request
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/network/request
---

# request

发起一个 HTTP 请求。

## 输入

param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)，扩展属性描述：

| **名称**       | **数据类型**                          | **是否必填**                                | **默认值** | **描述**    |
| ------------ | ------------------------------- | -------------------------------------- | ------ | --------- | ---------- |
| url          | string                          | 是                                     | -      | 请求地址      | PC 端 1.0.0 |
| header       | object                          | 否 | { 'content-type': 'application/json' }      | 请求 Header | PC 端 1.0.0 |
| method       | 'GET' \| 'POST' \| 'PUT'        |   否                                | 'GET'      | 请求方法      | PC 端 1.0.0 |
| data         | object \| string \| arraybuffer |  否                                   | ''       | 请求数据      | PC 端 1.0.0 |
| dataType     | string                          | 否                                 | 'json'      | 请求数据类型    | PC 端 1.0.0 |
| responseType | 'text' \| 'arraybuffer'         |   否                               | 'text'      | 响应数据类型    | PC 端 1.0.0 |

## 输出

success 函数返回对象参数扩展属性：

| **名称**     | **数据类型**                          | **描述**      |
| ---------- | ------------------------------- | ----------- | ---------- |
| statusCode | number                          | HTTP 状态码    | PC 端 1.0.0 |
| header     | object                          | HTTP Header | PC 端 1.0.0 |
| data       | object | string | arraybuffer | 数据          | PC 端 1.0.0 |

## 返回值

RequestTask 为 tt.request 返回的一个对象，可以通过调用该对象的 abort 方法中断任意请求任务。

| **名称** | **数据类型**     | **描述**   |
| ------ | ---------- | -------- | ---------- |
| abort  | () => void | HTTP 状态码 | PC 端 1.0.0 |

## 示例代码

### 请求示例
:::html
<md-block-api>
{
  "sourceData":{
  	"tab": "api",
  	"item": "request"
  },
    "openDetail": 1, 
    "title": "request", 
    "list_page_url": "https://applink.larksuite.com/client/block/workplace/open?appId=cli_a00834ec56f8d01b%26blockTypeId=blk_610a40455f800004c32b6bb6%26sourceData=%7B%22tab%22%3A%22api%22%2C%22item%22%3A%22login%22%7D", 
    "min_lk_ver": {
        "pc": "5.10.0", 
        "mobile": "5.11.0"
    },
    "blockEntity": {
        "sourceData": {
            "type": "api",
            "item": "request",
            "isNew": true
        },
  		"blockID": "mock-block"
    }
}
</md-block-api>

:::
```js
const task = tt.request({
  url: 'someURL',
  header: {
    'content-type': 'application/json'
  },
  method: 'GET',
  data: {},
  dataType: 'json',
  responseType: 'text',
  success (res) {
    console.log('request 调用成功', res);
  },
  fail (res) {
    console.log('request 调用失败', res.errMsg);
  },
  complete (res) {
    console.log('request 调用结束', res.errMsg);
  }
});

if (someReason) {
  task.abort();
}
```

### 返回示例

```json
{
  "data": {},
  "header": {
    "access-control-allow-headers": "Content-Type,User-Agent, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization, accept, origin, Cache-Control, X-Requested-With,X-Nt-Engine,Psm,Token,locale,x-web-id",
    "access-control-allow-origin": "http://localhost:9000",
    "content-type": "application/json; charset=utf-8"
  },
  "statusCode": 200,
  "errMsg": "request:ok"
}
```

## 已知问题

1. header 的部分关键字应当遵守标准，如 Cookie，具体请参考 [HTTP Headers](https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Headers)，同时 header 字段不支持设置 referer。
2. 单个 Block 并发请求的最高数量为 5 个。
3. 开发者服务端需要开启 CORS 以允许 <https://*.feishu.cn> 跨域访问。其中，Access-Control-Allow-Origin 的值不能包含星号，需要设置为确定的域名，可根据请求头部中的 Origin 字段确定。例如：

	```
	Access-Control-Allow-Credentials: true
	Access-Control-Allow-Headers: Content-Type
	Access-Control-Allow-Methods: GET,POST,PUT,DELETE,OPTIONS
	Access-Control-Allow-Origin: https://bytedance.feishu.cn
	```
  
	如果需要获取到响应头中某个字段（常用于登录场景），还需要服务端配置：
  
	```
	Access-Control-Expose-Headers: 字段名
	```

4. 请求不会携带 Cookie，因此开发者不能通过 Cookie 鉴权。请选择通过 Authorization 等在请求头携带 Token 的方式鉴权。
