---
document_id: '6965379543683760134'
directory_id: '6907567269107597314'
title: enterChat
full_path: /uYjL24iN/ukDM04SOwQjL5ADN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- enterChat
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:12Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDM04SOwQjL5ADN
---

# enterChat(Object object)

打开指定会话

:::html
<md-alert type="tip">
小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )

[V3.8](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)版本后支持回调 
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
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td> <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> 
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
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
            <md-th style="width: 10%;">
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
                openid
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                用户 [open_id](/document/home/user-identity-introduction/open-id)

**示例值**：ou_f096d8391d54fab99895d34519eb9e79
              
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                openChatId
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                会话[open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)

**示例值**：oc_1965ed81fc91d3b73d68c4ca4cfc110a
<md-alert type="tip" icon="none">
- Lark[V3.10](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description) 和[open_id](/document/home/user-identity-introduction/open-id)都传的时候，[open_id](/document/home/user-identity-introduction/open-id)优先
- [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)和[open_id](/document/home/user-identity-introduction/open-id) 必须传入一个
</md-alert>              
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                needBadge
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>true</md-td>
            <md-td>
                是否需要展示会话页面左上角badge数

        
<md-alert type="tip" icon="none">
- Android/iOS 端：Lark[V3.10.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC 端：暂不支持
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
tt.enterChat({
    openChatId: 'oc_1965ed81fc91d3b73d68c4ca4cfc110a',
    success (res) {
        console.log(JSON.stringify(res));
    },
    fail (res) {
        console.log(`enterChat fail:${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{"errMsg":"enterChat:ok"}
``` 








