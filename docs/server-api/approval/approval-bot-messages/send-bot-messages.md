---
document_id: '6967331173082038277'
directory_id: '7122028361538813958'
title: 发送审批 Bot 消息
full_path: /ukTMukTMukTM/ugDNyYjL4QjM24CO0IjN
breadcrumb:
- Server API
- Approval
- Approval Bot messages
- Send Bot messages
document_type: GuideDocumentType
updated_at: 2023-01-31T12:17:08Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugDNyYjL4QjM24CO0IjN
---

# 发送审批 Bot 消息

此接口可以用来通过Lark审批的Bot推送消息给用户，当有新的审批待办，或者审批待办的状态有更新时，可以通过Lark审批的Bot告知用户。当然开发者也可以利用开放平台的能力自建一个全新的Bot，用来推送审批相关信息。如果出现推送成功，但是没有收到消息，可能是因为开通了审批机器人的聚合推送。

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
      <md-td>https://www.larksuite.com/approval/openapi/v1/message/send</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>

   <md-tr>
     <md-th>支持的应用类型</md-th>
      <md-td>
	  <md-app-support types="custom"></md-app-support>
      </md-td>
   </md-tr>
    
    
    
    
     <md-tr>
      <md-th>
 权限要求
 <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
</md-th>
      <md-td>
<md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm>
</md-td>
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
      <md-td> 是 </md-td> 
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
 
**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"
          
 [了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)
	</md-td>
</md-tr>
     <md-tr> 
      <md-td>Content-Type</md-td>  
      <md-td>string</md-td>  
      <md-td> 是 </md-td> 
     <md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
   
  </md-tbody> 
</md-table>
:::
### 通用模板-请求体
|参数|类型|必须|说明|示例
|-|-|-|-|-
|template_id|String|是|模板 id，标识使用哪个卡片模板，枚举值|1001|
|user_id|string|是|接受bot提醒用户的user_id，是开放平台的 employee id（等同于user_id）|b85s39b
|uuid|string|否|幂等 id，最大长度为64，用于保证卡片只发送一次，uuid+user_id相同的卡片只会发送一次|1234567
|approval_name|string|否|卡片标题中的{审批定义名称}， i18n key|@i18n@1
|title_user_id|string|否|卡片标题中的{申请人}、{审批人}、{评论人}、{抄送人}等，user id|b85s39b
| title_user_id_type             | string | 是   | 可选"open_id"或者"user_id", 值为"open_id"则title_user_id传open_id(ou_xxxx), 不传或者值为"user_id"则title_user_id传employee id                       |
|comment|string|否|评论区内容，支持[消息卡片-markdown](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN)，i18n key|@i18n@2
|content|map|否|卡片内容|
|&emsp;∟user_id|string|否|申请人 user id<br>user_id为空时，申请人不显示|b85s39b
|&emsp;∟user_id_type|string|否|申请人user id类型。<br>user_id_type为空或取值"user_id"时, user_id为employee id, 取值"open_id"时user_id为open_id(例: ou_xxxx)|b85s39b
|&emsp;∟department_id|string|否|申请人部门 department_id<br>如果传入user_id，则department_id要必填，该字段不支持传open_department_id|--
|&emsp;∟user_name|string|否|申请人名，如果传了 user_id 则优先使用 user_id<br>用于申请人不是真实用户的场景|@i18n@3
|&emsp;∟department_name|string|否|申请人部门名，如果传了 department_id 则优先使用 department_id|@i18n@4
|&emsp;∟summaries|list|是|审批事由，最多5个|
|&emsp;&emsp;∟summary|string|否|审批事由，支持在i18中设置markdown元素[消息卡片-markdown](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN)，i18n key<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/69bada4fe9c3e017bcdaa202831be59a_x0YSohrDJt.png?lazyload=true&width=988&height=1280)|
|note|string|否|备注区，内容为审批来源，是否需要从内网访问<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6ffb657012336bab931d3f913ce5d9d4_Hrgxvqb4T9.png?lazyload=true&width=988&height=1280)|@i18n@6
|open_id|string|是|某个应用下用户的唯一标识|ou-8ec33278bc2
|sender_user_id|string|否|发送人的employee id，用于发送IM卡片|b85s39b
|text|string|否|转发留言，用于发送IM卡片发送一条留言<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8ef4c5af656f58f92c9cf7721945342f_wLmZNKKY1v.png?lazyload=true&width=622&height=1280)|请尽快完成审批
|actions|list|是|行动区，最多2个，第一个为查看详情，必传，第二个可自定义，可选传<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/23a9a6dbba28ae585796d62955610db5_6wtTxCMjwl.png?lazyload=true&width=1046&height=1280)|
|&emsp;∟action_name|string|是|操作类型，第一个action的action_name必须为DETAIL，自定义action的action_name为i18n key<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b08fe3bf4f7316d9bf60cf57ff119c94_JIHcNCWoOV.png?lazyload=true&width=989&height=1280)|@i18n@7
|&emsp;∟url|string|是|默认链接，不同的端配置不同的操作跳转 url，链接必须包含schema才能生效。例如：https、http|https://bytedance.com
|&emsp;∟android_url|string|是|Android链接|https://bytedance.com
|&emsp;∟ios_url|string|是|iOS链接|https://bytedance.com
|&emsp;∟pc_url|string|是|PC链接|https://bytedance.com
|action_callback|map|否|快捷审批<br>橘黄色的卡片支持快捷审批<br>其他卡片的不支持|
|&emsp;∟action_callback_url|string|否|三方系统的操作回调 url，【待审批】列表的任务审批人点同意或拒绝操作后，审批中心调用该地址通知三方系统|http://www.feish.cn/approval/openapi/instanceOperate
|&emsp;∟action_callback_token|string|否|回调时带的 token， 用于业务系统验证请求来自审批 具体参考[事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM) |sdjkljkx9lsadf110
|&emsp;∟action_callback_key|string|否|请求参数加密密钥，如果配置了该参数，则会对请求参数进行加密，业务需要对请求进行解密，加解密算法 [参考](/document/ukTMukTMukTM/uADM4QjLwADO04CMwgDN)|gfdqedvsadfgfsd
|&emsp;∟action_context|string|否|操作上下文，回调的时候会把该参数回传|asdasdasdasd
|action_configs|list|否|快捷审批操作配置 ，橘黄色的卡片支持快捷审批，其他卡片的不支持<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b08fe3bf4f7316d9bf60cf57ff119c94_949QCJilYA.png?lazyload=true&width=989&height=1280)|
|&emsp;∟action_type|string|是|操作类型：<br>APPROVE - 同意<br>REJECT - 拒绝<br>{KEY} - 任意英文字符串，如果使用任意字符串，则需要提供 action_name|APPROVE
|&emsp;∟action_name|string|否|操作名称，i18n key 用于前台展示，如果 action_type 不是 APPROVE和REJECT，则必须提供该字段，i18n key， 文案内容在 i18n_resource 提供|@i18n@8
|&emsp;∟is_need_reason|bool|否|是否需要意见，如果是，则用户点操作后会跳转到意见填写页面<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9c8098a3b221efb2662518e09b942f7e_dApPUrfHMh.png?lazyload=true&width=363&height=750)|true
|&emsp;∟is_reason_required|bool|否|意见是否必填|true
|&emsp;∟is_need_attachment|bool|否|意见支持附件|true
|&emsp;∟next_status|string|否|如果回调成功后，卡片的更新成什么状态。如果指定，则Lark审批会在用户操作后，把卡片状态更新成 {next_status}，如果不指定，则Lark侧不主动更新卡片，由业务自己更新卡片，状态值参见[更新审批Bot消息](/document/ukTMukTMukTM/uAjNyYjLwYjM24CM2IjN)|APPROVED<br>REJECTED
|i18n_resources|list|是|国际化文案|
|&emsp;∟locale|string|是|语言<br>zh-CN - 中文<br>en-US - 英文<br>ja-JP  - 日文|zh-CN
|&emsp;∟is_default|bool|是|是否默认语言，默认语言需要包含所有key，非默认语言如果key不存在会使用默认语言代替|true
|&emsp;∟texts|map|是|文案 key, value,  i18n key 以  @i18n@ 开头|{<br>"@i18n@1": "权限申请", <br>  "@i18n@2": "OA审批",<br>"@i18n@3": "Permission"<br>}

### 通用模板-请求体示例

```json
{
    "template_id":"1001",
    "user_id":"b85s39b",
    "uuid":"uuid",
    "approval_name":"@i18n@1",
    "title_user_id":"od-8ec33278bc2",
    "title_user_id_type": "user_id",
    "comment":"@i18n@2",
    "content":{
        "user_id":"b85s39b",
      	 "user_id_type": "user_id",
        "department_id":"od-8ec33278bc2",
        "summaries":[
            {
                "summary":"@i18n@3"
            }
        ]
    },
    "note":"@i18n@4",
    "actions":[
        {
            "action_name":"DETAIL",
            "url":" https://bytedance.com",
            "android_url":"https://bytedance.com",
            "ios_url":"https://bytedance.com",
            "pc_url":"https://bytedance.com"
        }
    ],
    "action_configs": [
        {
            "action_type": "APPROVE",
            "is_need_reason": true,
            "is_reason_required": true,
            "is_need_attachment": true,
            "next_status": "APPROVED"
        },
        {
            "action_type": "REJECT",
            "action_name": "@i18n@5",
            "next_status": "REJECTED"
        }
    ],
    "action_callback": {
        "action_callback_url":"http://feish.cn/approval/openapi/operate",
        "action_callback_token":"sdjkljkx9lsadf110",
        "action_callback_key":"gfdqedvsadfgfsd",
        "action_context":"acasdasd"
    },
    "i18n_resources":[
        {
            "locale":"en-US",
            "is_default":true,
            "texts":{
                "@i18n@1":"Temporary release",
                "@i18n@2":"Disapproval",
                "@i18n@3":"Need to modify",
                "@i18n@4":"From OA,please access through the internal network.",
                "@i18n@5":"Cancel"
            }
        }
    ]
}
```

### 自定义模板-请求体 

| 参数                     | 类型   | 必须 | 说明                                                         |
| ------------------------ | ------ | ---- | ------------------------------------------------------------ |
| template_id              | string | 是   | 模板 id，标识使用哪个卡片模板，枚举值                        |
| user_id                  | string | 是   | 推送给哪个用户，开放平台的 employee id                       |       
| uuid                     | string | 否   | 幂等 id，最大长度为64，用于保证卡片只发送一次，uuid+user_id相同的卡片只会发送一次 |
| custom_title             | string | 是   | 标题，i18n key                                               |
| custom_content           | string | 是   | 卡片内容区域，支持[消息卡片-markdown](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN)，i18n key |
| note                     | string | 否   | 备注区，内容为审批来源，是否需要从内网访问，i18n key         |
| actions                  | list   | 否  | 行动区，最多2个                                              |
| &emsp;∟action_name       | string | 是   | 操作内容，i18n key                                           |
| &emsp;&emsp;∟url         | string | 是   | 默认链接不同的端配置不同的操作跳转 url，链接必须包含schema才能生效。例如：https、http |
| &emsp;&emsp;∟android_url | string | 是   | Android链接                                                  |
| &emsp;&emsp;∟ios_url     | string | 是   | iOS链接                                                      |
| &emsp;&emsp;∟pc_url      | string | 是   | PC链接                                                       |
| i18n_resources           | list   | 是   | 国际化文案                                                   |
| &emsp;∟locale            | string | 是   | 语言<br>zh-CN - 中文<br>en-US - 英文<br>ja-JP  - 日文       |
| &emsp;∟is_default        | bool   | 是   | 是否默认语言，默认语言需要包含所有key，非默认语言如果key不存在会使用默认语言代替 |
| &emsp;∟texts             | map    | 是   | 文案 key, value,  i18n key 以  @i18n@ 开头                   |

### 自定义模板-请求体示例
```json
{
    "template_id":"1021",
    "user_id":"employeeId1",
    "uuid":"uuid",
    "custom_title":"@i18n@1",
    "custom_content":"@i18n@2",
    "note":"@i18n@3",
    "actions":[
        {
            "action_name":"@i18n@4",
            "url":" https://bytedance.com",
            "android_url":"https://bytedance.com",
            "ios_url":"https://bytedance.com",
            "pc_url":"https://bytedance.com"
        }
    ],
    "i18n_resources":[
        {
            "locale":"en-US",
            "is_default":true,
            "texts":{
                "@i18n@1":"Custom template",
                "@i18n@2":"Please help process my approval as soon as possible.",
                "@i18n@3":"From OA,please access through the internal network.",
                "@i18n@4":"DETAIL"
            }
        }
    ]
}

```

## 响应

### 响应体
|参数|类型|必须|说明|
|-|-|-|-|
|code|int|是|错误码，非 0 表示失败|
|msg|string|是|返回码的描述|
|data|map|是|返回业务信息|
|&emsp;∟message_id|string|是|消息 id ，用于卡片更新|
### 响应体示例 
```json
{
    "code":0,
    "msg":"success",
    "data":{
        "message_id": "6968359519504171036"
    }
}
```

### 错误码
具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

## 模板列表
|模版编号|名称|参数|备注|
|-|-|-|-
|1001|收到评论|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8fb9760d9942485a563ca1a6ae4d7a80_DC69JK4Cb3.png?lazyload=true&width=612&height=744)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>comment|
|1002|审批暂存待办|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6965ac41d8b28b37e3a20f37795d14a1_jCZDbsfssB.png?lazyload=true&width=610&height=600)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions
|1003|审批已拒绝|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e09421ab669877a077df2dedbe7886d1_ZD7GaPZh05.png?lazyload=true&width=610&height=740)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions|
|1004|审批已通过|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33961fa40872dfcf7c920cdc54824f65_zFfabDOlyJ.png?lazyload=true&width=612&height=698)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>comment|
|1005|成功发起了审批|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/71e6428e1762b1489803669d308ed9a6_7ey4z5EJ10.png?lazyload=true&width=614&height=558)|必填字段：<br>approval_name<br>summaries<br>actions|
|1006|审批将被关闭|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/50aac7f26cedc703061207c19070e6e8_nVaYgR6XQ9.png?lazyload=true&width=614&height=556)|必填字段：<br>approval_name<br>summaries<br>actions|
|1007|审批已被关闭|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ced4e31bcea1371dc7dd265d831fb928_CKG7WDzkXy.png?lazyload=true&width=610&height=558)|必填字段：<br>approval_name<br>summaries<br>actions|
|1008|收到审批待办|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dafea7bff2c4f9882adf1504726880ec_87Cb86lNiP.png?lazyload=true&width=610&height=676)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>快捷审批<br>action_configs<br>action_callback<br>注意<br>title_user_id为空时，标题展示为：“{approval_name}”待你审批|
|1028|收到审批待办(无审批发起人)|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ffe9be0aff6d1f2f4ac0f72ef3984703_kSi0G6Wxrg.png?lazyload=true&width=608&height=622)|必填字段：<br>approval_name<br>summaries<br>actions<br>快捷审批<br>action_configs<br>action_callback|
|1009|被加签|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b2e556c8537a354d96e01891bcc9036f_t76Ku80hI9.png?lazyload=true&width=612&height=818)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>快捷审批<br>action_configs<br>action_callback|
|1010|被转交|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d20ead5685436adfdb264704af6b1b87_oyxuciPjYF.png?lazyload=true&width=612&height=814)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>快捷审批<br>action_configs<br>action_callback|
|1011|被委托|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a36faf13a0ce10489a20928ad97c0f1a_A2jsLDHvsp.png?lazyload=true&width=612&height=674)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>快捷审批<br>action_configs<br>action_callback|
|1012|被回退|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2235a11f4c7be3d633f2473816d671e1_0SBP8ABvGr.png?lazyload=true&width=610&height=816)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>快捷审批<br>action_configs<br>action_callback|
|1013|人工催办(个人IM)|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/55485bd38f8c3ea2fab47e21d4d9a5fe_9majcl930D.png?lazyload=true&width=610&height=814)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>快捷审批<br>action_configs<br>action_callback|
|1015|被撤回|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/73cb77eb1c541a84cff03f20cd111037_kSdabPPy27.png?lazyload=true&width=610&height=848)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions|
|1016|被抄送|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/57b4790def282828b8a6045b1f54b470_NYBfNguOVd.png?lazyload=true&width=612&height=844)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions|
|1017|评论被回复|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f353ef1941cf1678912e0c01b4294b2d_Mc6ZKsI2E9.png?lazyload=true&width=612&height=848)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>comment|
|1018|评论被提及|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/61216b9689fbeabe57cda30f64741838_7kh1DlwWvd.png?lazyload=true&width=616&height=852)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>comment|
|1019|申请人离职转交主管|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/930476e2ea2226418e78f73a44bdfa5f_MXWaavp7XW.png?lazyload=true&width=612&height=706)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions|
|1020|审批人离职抄送主管|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8334dad4a247eaee980908c32e846a8e_3RnJGRDMc7.png?lazyload=true&width=612&height=676)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>快捷审批<br>action_configs<br>action_callback|
|1021|自定义模板|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0b9bad0f497b1cfa2bb869ea2ccd22e7_0D5KuuHDfl.png?lazyload=true&width=612&height=472)|必填字段：<br>custom_title<br>custom_content<br>|
|1024|审批分享|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0414cfe2ad23056f7194591387b05f9e_fP2g2pOtoe.png?lazyload=true&width=614&height=660)|必填字段：<br>approval_name<br>summaries<br>actions|
|1026|系统抄送|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b819176c4f20f0d1cd8b0de15fe8924a_2PypN4k7Wi.png?lazyload=true&width=614&height=712)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions|
|1027|添加评论|<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/481c26811420a62d9f69556935578acf_OjJglPSBgn.png?lazyload=true&width=614&height=846)|必填字段：<br>approval_name<br>title_user_id<br>summaries<br>actions<br>comment|


