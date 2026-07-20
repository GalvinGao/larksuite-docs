---
document_id: '7035816914552340486'
directory_id: '7021842990278164485'
title: 延时更新消息卡片
full_path: /ukTMukTMukTM/uMDO1YjLzgTN24yM4UjN
breadcrumb:
- Server API
- Messaging
- Message card
- Delay update message card
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:42Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMDO1YjLzgTN24yM4UjN
---

# 延时更新消息卡片
用户对消息卡片完成交互操作后，对指定用户更新卡片内容，给与及时反馈。

### 使用场景
1. 用户点击卡片后业务方需要处理较长时间，无法在3s内及时返回需要展示的卡片内容。
2. 只更新一部分收到这张卡片成员（同一个`message_id`）看到的卡片内容。延迟更新使用交互回传事件中的`token`来指定目标更新的消息，无需额外关注原消息的`message_id`。

:::html
<md-alert type="warn"><b>注意事项</b>：
- 需要用户主动交互触发，不支持无条件更新
- 延迟更新使用的token有效期为30分钟，超时则无法更新卡片
- 调用延迟更新接口需要晚于同步返回，否则会出现不可预测行为<br>
  服务端处理时，可先立即 return 空串，再在30分钟内调用延迟更新接口更新卡片
 
- 只能更新用户交互对应卡片，不允许更新其他卡片
- 卡片内容经转换后不能超过100KB
- 同一token仅能使用3次
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
      <md-td>https://open.larksuite.com/open-apis/interactive/v1/card/update</md-td>
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

### 请求体
| 参数| 类型   | 必须 | 说明  | 实例 |                                                   
| - | - | - | - | - | - |
| token|string | 是   | 用于更新卡片的token，不是tenant_access_token（可通过[卡片交互返回内容](/document/ukTMukTMukTM/uEzNwUjLxcDM14SM3ATN)获取）| c-xxxxxxx |
| card|object | 是   | 消息卡片的描述内容，具体参考[卡片结构](/document/ukTMukTMukTM/uEjNwUjLxYDM14SM2ATN) | - |
|&emsp;∟open_ids | array | 否 | 指定需要更新的用户，共享卡片默认更新所有人卡片，无需填写该字段。推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)|["ou_5ad573a6411d72b8305fda3a9c15c70e"] |

### 请求体示例

```json
{
    "token":"c-515fbxxxx", // POST请求中返回的卡片更新凭证
     "card": {
        "open_ids":["ou_515fbe9d04838174e2035f8xxxx53d07f"], // 选填字段，指定需要更新消息的用户列表
        //更新后的卡片JSON内容，详细结构体在 通用能力 - 消息卡片 中查看
        "elements": [
            {
                "tag": "div",
                "text": {
                    "tag": "plain_text",
                    "content": "overflow & datePicker 功能测试"
                },
                "fields": [
                    {
                        "is_short": true,
                        "text": {
                            "tag": "lark_md",
                            "content": "**已同意**"
                        }
                    }
                ]
            }
        ]
    }
 }
```

## 响应

### 响应体

|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码描述|

### 响应体示例

```json
{
    "code": 0,
    "msg": "ok"
}
```

### 错误码
:::html
<md-table> 
  <md-thead> 
    <md-tr> 
      <md-th style="width: 18%;">错误码</md-th>  
      <md-th style="width: 15%;">说明</md-th>  
       <md-th style="width: 15%;">排查建议</md-th>   
    </md-tr> 
  </md-thead>  
  <md-tbody> 
    <md-tr> 
      <md-td>11311</md-td>  
      <md-td>卡片格式不符合要求。</md-td>  
      <md-td>参照错误message具体内容，卡片构造格式可以参考[卡片结构](/document/ukTMukTMukTM/uEjNwUjLxYDM14SM2ATN)。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>10002</md-td>  
      <md-td>card参数校验错误，请根据msg信息进行确认。</md-td>  
      <md-td>补齐card字段，具体格式可以参考[卡片结构](/document/ukTMukTMukTM/uEjNwUjLxYDM14SM2ATN)。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>100000</md-td>  
      <md-td>卡片内容转换后超过100KB。</md-td>  
      <md-td>减小卡片体积。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>100030</md-td>  
      <md-td>传入body参数不符合json规范。</md-td>  
      <md-td>检查传入参数。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>200000</md-td>  
      <md-td>该卡片消息已被撤回。</md-td>  
      <md-td>该卡片消息已撤回，不支持更新。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>200310</md-td>  
      <md-td>更新其他应用发送的卡片。</md-td>  
      <md-td>不允许更新其他应用发送的卡片。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>200320</md-td>  
      <md-td>非共享卡片的open_ids内容是否正确。</md-td>  
      <md-td>检查open_ids是否正确。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>300020</md-td>  
      <md-td>更新卡片token格式错误。</md-td>  
      <md-td>检查token格式，格式为c-xxxx，可通过[卡片交互返回内容](/document/ukTMukTMukTM/uEzNwUjLxcDM14SM3ATN)获取。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>300030</md-td>  
      <md-td>更新卡片token失效。</md-td>  
      <md-td>token有效期为30分钟，请检查token是否在有效期。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>300040</md-td>  
      <md-td>更新卡片token使用超过限制次数。</md-td>  
      <md-td>token仅能使用3次，请检查token是否超过使用次数。</md-td> 
	</md-tr>
    <md-tr> 
      <md-td>300090</md-td>  
      <md-td>非共享卡片需填写open_ids字段。</md-td>  
      <md-td>检查open_ids是否填写且是否正确。</md-td> 
	</md-tr>
  </md-tbody> 
</md-table>
:::

其他通用错误码可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)。
