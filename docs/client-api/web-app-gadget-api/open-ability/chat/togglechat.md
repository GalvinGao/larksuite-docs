---
document_id: '6971043590354026501'
directory_id: '6907567269107597314'
title: toggleChat
full_path: /uYjL24iN/ugDM04COwQjL4ADN/toggleChat
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- toggleChat
document_type: GuideDocumentType
updated_at: 2022-03-11T04:12:44Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDM04COwQjL4ADN/toggleChat
---

# toggleChat(Object object)

侧边栏形式打开或关闭会话，重复调用可以控制侧边栏的打开和关闭。

:::html
<md-alert type="tip">
如果需要兼容 [V4.1.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 之前的版本，切换聊天对象时，可以通过调用两次`tt.toggleChat({ openChatId: newOpenChatId, isKeep: true })`保证功能不受影响
  - [V4.1.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 之前会是关闭侧边栏，再打开 `newOpenChatId` 对应的新聊天对象窗口
  - [V4.1.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及之后是不关闭侧边栏，直接切换聊天对象，相同 `openChatId` 连调两次不会有闪烁
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
<md-td>**X**</md-td>
<md-td>**X**</md-td>
<md-td>**✓**</md-td>
<md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td>**X**</md-td>
<md-td>**X**</md-td>
<md-td><md-version>V4.1.0+</md-version></md-td>
<md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app></md-td>
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
openChatId
</md-td>
<md-td>
string
</md-td>
<md-td>
是
</md-td>
<md-td></md-td>
<md-td>
会话 [open_chat_id](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)

**示例值**：oc_7dab8a3d3cdcc9da365777c7ad535d64
</md-td>
</md-tr>
<md-tr>
<md-td>
width
</md-td>
<md-td>
number
</md-td>
<md-td>
否
</md-td>
<md-td>
400
</md-td>
<md-td>
侧边栏宽度
</md-td>
</md-tr>
<md-tr>
<md-td>
needSidebar
</md-td>
<md-td>
boolean
</md-td>
<md-td>
否
</md-td>
<md-td>
true
</md-td>
<md-td>
是否需要侧边菜单栏
</md-td>
</md-tr>
<md-tr>
<md-td>
isKeep
</md-td>
<md-td>
boolean
</md-td>
<md-td>
否
</md-td>
<md-td>
false
</md-td>
<md-td>
保持侧边栏不关闭，切换聊天对象
- 当侧边栏关闭时：isKeep 无论是 true、false 还是没有传参，都是打开侧边栏
- 当侧边栏打开时：isKeep 只有为 true 才是保持侧边栏不关闭，其他情况都是关闭侧边栏
<md-alert type="tip" icon="none">
- Android/iOS 端：暂不支持
- PC 端：Lark[V4.1.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
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
tt.login({
  success() {
    tt.chooseChat({
      success(res) {
        const openChatId = res.data[0].id;
        tt.toggleChat({
          openChatId,
          width: 400,
          needSidebar: true,
          isKeep: false,
          success(res) {
            console.log(JSON.stringify(res));
          },
          fail(res) {
            console.log(`toggleChat fail: ${JSON.stringify(res)}`);
          },
        });
      },
    });
  },
});
```

`success`返回对象示例：

```json
{
  "errMsg": "toggleChat:ok"
}
```
