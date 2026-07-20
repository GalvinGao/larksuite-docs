---
document_id: '6963854764275236869'
directory_id: '6907567269107777538'
title: 调用 API
full_path: /ukTMukTMukTM/ukDNz4SO0MjL5QzM/get-
breadcrumb:
- Server API
- API Call Guide
- Calling Process
- Calling APIs
document_type: GuideDocumentType
updated_at: 2023-11-10T07:54:01Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/get-
---

# 调用 API

为了让开发者可以便捷地调用 API，Lark开放平台提供了 Java SDK、Golang SDK 和 NodeJS SDK。有关 SDK 的详细介绍，可以参考[服务端 SDK](/document/ukTMukTMukTM/uETO1YjLxkTN24SM5UjN)。本文介绍基本的 API 调用方法。

## 前提条件

- 完成创建应用、[申请权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)、[获取访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)、[设置 IP 白名单](/document/ukTMukTMukTM/ucTMxYjL3ETM24yNxEjN)之后，才能调用服务端 API。
- 调用 API 时，需要将访问凭证放入请求 Header 中（`Authorization:Bearer <access token>`）。
- 调用服务端 API 时，需要使用 HTTPS 协议、UTF-8 编码。

## 调用示例

### 向企业内员工发消息

你可以调用[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)接口完成向企业内员工发消息的操作，从接口文档中可以确定，调用该接口前，需要获取 `tenant_access_token`。

1.  参考 [获取访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) 获取 `tenant_access_token`。
    
    获取凭证的请求示例如下，你需要将 *app_id* 和 *app_secret* 替换为实际值。

    ```
    curl -X POST 'https://open.larksuite.com/open-apis/auth/v3/tenant_access_token/internal/'
    -H 'Content-Type: application/json; charset=utf-8' 
    -d '{
    "app_id": "<app_id>",
    "app_secret": "<app_secret>"
	}'
    ```
    
2. 根据文档内的请求参数描述，调用[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)接口。

	- 方式一：在调试台发起 API 调用
	
		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/aa007bc867dc38d935cdcd335a037c06_smkGlGqsxN.png?height=332&lazyload=true&maxWidth=600&width=943)
	- 方式二：本地发送 curl 请求
	
		该 API 需要使用 POST 方式发起。
			
		![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/85be6f7b4a4611d41665b1120c1b1445_0TGCvluM2T.png?height=366&lazyload=true&maxWidth=600&width=730)
		
		示例如下，请将参数示例值替换为实际值。
    
		```HTTP 
		curl -X POST 'https://open.larksuite.com/open-apis/im/v1/messages?receive_id_type=user_id'
		-H 'Authorization:Bearer <tenant_access_token>'
		-H 'content-type:application/json; charset=utf-8'
		-d '{
	        	"content": {
	                	"text": "Hello World"
	        	},
	        	"msg_type": "text",
 	       	"receive_id": "<user_id>"
		}'
		``` 
        
        - *receive_id_type* 作为查询参数。
		- *content* *、msg_type* 和 *receive_id* 作为请求的 Body 内容。
		- 请求所需的 `tenant_access_token` 和 Content-Type 放在 Header 中。
	

### 查询用户信息

你可以调用[获取单个用户信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/get)接口查询用户信息。从接口文档中可以确定，调用该接口前，需要获取 `tenant_access_token` 或 `user_access_token`，请根据需要获取的用户信息范围，选择合适的访问凭证。

1. 参考 [获取访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) 获取 `tenant_access_token` 或 `user_access_token`。

2. 根据文档内的请求参数描述，调用[获取单个用户信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/get)接口。

	- 方式一：在调试台发起 API 调用
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/60edbe9c112de55e65427818eab3fa06_DE7dVr6hYb.png?height=151&lazyload=true&maxWidth=600&width=437)
    
	- 方式二：本地发送 curl 请求

     从接口文档中可以看出，`user_id` 是该接口的路径参数，例如我们查询一个 `user_id` 为 `ou_c99c5f35d542efc7ee492afe11af19ef` 的用户信息，示例如下：
   
     ```
        curl -X GET 'https://open.larksuite.com/open-apis/contact/v3/users/ou_c99c5f35d542efc7ee492afe11af19ef?user_id_type=user_id' \
        -H 'Authorization: Bearer <tenant_access_token>' \
        -H 'Content-Type: application/json; charset=utf-8'
     ```
     
## 响应结果

绝大多数 API 的响应体结构包括 `code`、`msg`、`data` 三个部分：
- `code`：错误码。如果是成功响应，`code` 取值为 0。
- `msg` ：错误信息。
- `data`：API 的调用结果。`data` 在一些操作类 API 的返回中可能不存在。
:::note
- 请不要依据 `msg` 来判定一个请求是否失败。
- 接收到失败响应后，你可以先查看 [通用错误码](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)说明，排查问题。如果依然不能解决问题，可以向Lark开放平台反馈响应头中的 `x-tt-log``id` 值，以便我们协助定位问题。
:::

成功响应示例：
```
{
  "code": 0,
  "msg": "success",
  "data": {
    // 响应的具体数据内容
  }
}
```
失败响应示例：
```
{
  "code": 40004,
  "msg": "no dept authority error"
}
```
