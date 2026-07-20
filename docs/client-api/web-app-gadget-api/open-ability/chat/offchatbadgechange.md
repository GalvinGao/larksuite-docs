---
document_id: '7073693024736067589'
directory_id: '6907567269107597314'
title: offChatBadgeChange
full_path: /uYjL24iN/ugDM04COwQjL4ADN/offchatbadgechange
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- offChatBadgeChange
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:34Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDM04COwQjL4ADN/offchatbadgechange
---

# offChatBadgeChange(Object object)

取消监听某个群未读消息数变化

:::html
<md-alert type="tip">
注意事项：
- 小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )
- 需要读取群信息权限，接口才能调用 <md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm>
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
      <md-td><md-version>V3.10.0+</md-version></md-td>
      <md-td><md-version>V3.10.0+</md-version></md-td>
      <md-td><md-version>V3.10.0+</md-version></md-td>
      <md-td>
        <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app>
      </md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V7.10.0+</md-version></md-td>
      <md-td><md-version>V7.10.0+</md-version></md-td>
      <md-td><md-version>V7.10.0+</md-version></md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入
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
                [openChatId](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                获取会话信息的会话Id

            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                onChange
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                回调函数
<md-alert type="tip" icon="none">
如果不传 `onChange` 回调函数，则会取消[openChatId](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)对应的所有监听，传onChange则会取消指定监听
</md-alert>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出
无


## 示例代码

```js
tt.offChatBadgeChange(
  {
    openChatId: 'oc_7dab8a3d3cdcc9da365777c7ad535d64',
    onChange: (res) => {
      console.log(JSON.stringify(res))
    }
  }
)
```


