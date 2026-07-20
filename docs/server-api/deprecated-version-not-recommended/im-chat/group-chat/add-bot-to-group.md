---
document_id: '6967261389551435781'
directory_id: '6907567266536652801'
title: 机器人进群
full_path: /ukTMukTMukTM/uYDO04iN4QjL2gDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Group Chat
- Add Bot to Group
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:27Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYDO04iN4QjL2gDN
---

 # 机器人进群
拉机器人进群<br>

**权限说明** ：需要启用机器人能力；机器人的owner需要已经在群里

**请求方式** ：POST<br>
**请求地址** ：https://open.larksuite.com/open-apis/bot/v4/add<br>

**请求参数说明**<br>
**请求 Header** ：<br>
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
Authorization|string|必填|tenant_access_token 通过接口 [获取 tenant_access_token（应用商店应用）](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token) 或者 [获取 tenant_access_token（企业自建应用）](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal) 获得<br>注意内容不要漏了 "Bearer"||Bearer t-394890fdlkfjdajfljajdkf 
Content-Type|string|必填|Content-Type||application/json
**请求参数** ：<br>
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
|chat_id|string|必填| 群的id||oc_4c24bbde8572c9daedd5e67f6a8ff5e4|

**请求 Body** ：
```json
{
   "chat_id":"oc_4c24bbde8572c9daedd5e67f6a8ff5e4"
}
```

**返回参数说明** : 
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码描述|

**返回 Body** ：  
```json
{
    "code": 0,
    "msg": "ok"
}
```






