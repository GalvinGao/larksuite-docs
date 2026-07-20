---
document_id: '6970625975390470150'
directory_id: '7021842990277787653'
title: 加号（“+”）菜单
full_path: /ukTMukTMukTM/ukDN1YjL5QTN24SO0UjN
breadcrumb:
- Developer Guides
- Configure App Entry
- Message Field Shortcuts (“+”)
document_type: GuideDocumentType
updated_at: 2021-12-20T07:56:27Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukDN1YjL5QTN24SO0UjN
---

# 如何将应用配置到加号（“+”）菜单

# 什么是“加号菜单”
“加号菜单”是Lark提供的一种开放能力，**允许应用在Lark会话输入框的“+”中配置入口**，从而提供一种便捷的交互路径，支持Lark用户在会话中便捷打开应用，并分享应用内容至会话。

目前， **小程序应用、网页应用** 均可开启“加号菜单”能力（网页应用从3.39版本开始支持）。



# 如何配置“加号菜单”
## 创建应用并设计承接页
加号菜单能力的亮点在于可支持用户通过会话内加号菜单入口快速打开应用的指定页面；因此，创建应用并设计一个承接页是接入加号的重要步骤：

1. 首先，开发者需先**开启[小程序](/document/uQjL04CN/ukjL54SO)或[网页](/document/uQjL04CN/uAzM3QjLwMzN04CMzcDN)能力**；

2. 其次，开发者需基于小程序或网页应用**设计一个专属承接页**，满足用户的场景化诉求；例如：
> 问卷工具可将**问卷创建页**作为承接页，支持用户快速创建问卷；
> 
> 提醒工具可将**提醒任务设置页**作为承接页，支持用户在会话中快速创建提醒项；
> 
> 天气查询工具可将**搜索查询页**或**城市列表页**作为承接页，支持用户快速查询天气。



## 启用能力
完成承接页设计后，可前往"开发者后台-应用功能-扩展"一栏启用加号菜单能力，并填写对应承载页的 [applink](/document/uYjL24iN/ucjN1UjL3YTN14yN2UTN)。

> - 由于桌面端需要在会话内打开应用，**桌面端小程序应用 applink 的打开模式必须定义为 sidebar-semi 模式；网页应用则为 mode=sidebar**。
> -  例如：https://applink.larksuite.com/client/mini_program/open?appId=cli_9e2c876453ee900e&**mode=sidebar**&path=pages/plus/index


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1856ee2faf88ca95a4f30662356f7d7b_dOdw5yhSch.png)

启用能力并发布新版本后，具备该应用可用性的用户即可在会话的加号菜单内看到应用入口。

## 场景判断
开发者需要根据不同的场景来调用相应的Api来完成功能，下文将阐述如何判断是通过加号(“+”)菜单打开的应用

### 小程序应用场景判断
通过[getLaunchOptionsSync](/document/uYjL24iN/uAzM1YjLwMTN24CMzUjN) Api 获取LaunchOptions，然后通过[场景值](/document/uYjL24iN/uQzMzUjL0MzM14CNzMTN)进行判断

示例代码：
```javascript
var options = tt.getLaunchOptionsSync() 
const isFromPlusMenu = (options.scene == 1509 || options.scene == 1510)
```
### 网页应用场景判断

:::note
注意：如果你为不同的场景配置了不同的承载页，此步骤可以跳过
:::

1. 获取当前页面的Url
2. 判断是通过加号("+")打开的应用
- pc: 从url 获取 `from` 参数值，判断`from`是否为`plus_menu_p2p` 或者 `plus_menu_group`
- 移动端：从url获取 `required_launch_ability`, 判断 `required_launch_ability` 是否为 `chat_action`

示例代码
```javascript
const searchParams = new URLSearchParams(location.search)
const launchAbility = searchParams.get('required_launch_ability')
const from = searchParams.get('from')
var isFromPlus = false
console.log(`launchAbility${launchAbility}`)
console.log(`from${from}`)
boolean fromPc = ...// check ua
boolean fromMobile = ...// check ua
if (fromMobile && launchAbility) {
  isFromPlus = launchAbility == 'chat_action'
}
if (fromPc && from) {
  isFromPlus = (from == 'plus_menu_p2p' || from == 'plus_menu_group')
}
```

## 发送卡片
除了支持用户便捷打开应用页面查询或创建信息，开发者也需为用户提供信息发送的能力，支持用户便捷发送应用内容至当前会话。

### 小程序应用
目前，小程序应用可以通过以下3个API，获取相关上下文，完成消息卡片的发送。
#### 1. 通过 [tt.getHostLaunchQuery](/document/uYjL24iN/ugzM4UjL4MDO14COzgTN) 获取 triggerCode

```js 
tt.getHostLaunchQuery({ 
    success (res) {
        let triggerCode = JSON.parse(decodeURIComponent(res.launchQuery)). __trigger_id__
        console.log(`triggerCode`); 
    }, 
    fail (res) { 
        console.log(`获取参数失败`); 
    } 
}); 
```

#### 2. 通过 [tt.sendMessageCard](/document/uYjL24iN/uUjN5UjL1YTO14SN2kTN) 发送卡片
- triggerCode和openChatIDs参数传其一即可，两个都传的时候openChatIDs优先使用；
- 使用triggerCode时，会将卡片发送到经“加号菜单”打开小程序时的会话中。


```js 
tt.sendMessageCard({
    triggerCode: 'abc',
    openChatIDs: ['xxxx','xxxx'],
    cardContent: {
        update_multi:false, 
        card: { 
            // card content 
        }
    },
    success (res) {
        console.log(res);
    },
    fail (res) {
        console.log(res);
    }
})
 
```

卡片内容json示例，详细配置请参考[消息卡片](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)章节。

```js 
{
        "update_multi": false,
        "card": {
                "header": {
                        "title": {
                                "tag": "plain_text",
                                "content": "This is Header"
                        },
                        "template": "purple"
                },
                "elements": [{
                        "tag": "div",
                        "text": {
                                "tag": "plain_text",
                                "content": "测试用例：A:1-5"
                        }
                }]
        }
} 
```
#### **3. 通过 tt.getTriggerContext 确定目标会话**
:::note
注意：调用这个接口前请先确保已经[登录](/document/uYjL24iN/uYzMuYzMuYzM)。
:::

```js 
tt.getTriggerContext({
    triggerCode: 'abc',
    success (res) {
        let openChatId = res.openChatId
        console.log(openChatId);
    },
    fail (res) {
        console.log(res);
    }
}) 
```

#### **完整示例**
- 页面结构


```js 
<view>triggerCode:</view>
<view>{{ triggerCode }}</view>
<button bindtap="getTriggerCode">getTriggerCode</button>

<view>openChatId:</view>
<view>{{ openChatId }}</view>
<button bindtap="getOpenChatId">getOpenChatId</button>

<button bindtap="sandTestMessage">SendTestMessage</button> 
```

- 页面逻辑


```js 
/* eslint-disable @typescript-eslint/camelcase */
const cardSchema = `{
  "header": {
      "title": {
          "tag": "plain_text",
          "content": "This is Header"
      },
      "template": "purple"
  },
  "elements": [{
      "tag": "div",
      "text": {
          "tag": "plain_text",
          "content": "测试用例：A:1-5"
      }
  }]
}`;

Page({
  data: {
    triggerCode: '123',
    openChatId: '123',
  },

  getTriggerCode() {
    tt.getHostLaunchQuery({
      success: res => {
        tt.showModal({
          title: 'getHostLaunchQuery',
          content: res,
        });
        const obj = JSON.parse(decodeURIComponent(res.launchQuery));
        tt.showModal({
          title: 'getHostLaunchQuery',
          content: `got triggerCode: ${obj.__trigger_id__}`,
          success: res => {
            if (res.confirm) {
              console.log('confirm');
              this.setData({
                triggerCode: obj.__trigger_id__,
              });
            }
          },
        });
      },
      fail(err) {
        tt.showModal({
          title: 'getHostLaunchQuery',
          content: err,
        });
        console.log(err);
      },
    });
  },

  getOpenChatId() {
    console.log(this.data.triggerCode);
    tt.showModal({
      title: 'triggerCode',
      content: this.data.triggerCode,
      success: res => {
        if (res.confirm) {
          console.log('confirm');
        }
      },
    });
    tt.getTriggerContext({
      triggerCode: this.data.triggerCode,
      success: res => {
        console.log(res);
        tt.showModal({
          title: 'getTriggerContext',
          content: `got triggerContext: ${res}`,
          success(res) {
            if (res.confirm) {
              console.log('confirm');
            }
          },
        });
        this.setData({
          openChatId: res.openChatId,
        });
      },
      fail: err => {
        console.log(err);
      },
    });
  },

  sandTestMessage() {
    tt.sendMessageCard({
      triggerCode: this.data.triggerCode,
      cardContent: {
        msg_type: 'interactive',
        root_id: 'test',
        update_multi: false,
        card: JSON.parse(cardSchema),
      },
      success(res) {
        tt.showModal({
          title: 'sendMessageCard success',
          content: `res: ${res}`,
          success(res) {
            if (res.confirm) {
              console.log('confirm');
            }
          },
        });
        console.log(res);
      },
      fail(res) {
        console.log(res);
      },
    });
  },
}); 
```
### 网页应用
> 如果您之前没有开发过Lark内的网页应用，请先通过[开发文档](/document/uYjL24iN/uITO4IjLykDOy4iM5gjM)进行充分了解 。

#### **1. 接口鉴权**

**在调用 H5 接口前，请先完成鉴权**。即基于页面 URL，使用固定的算法生成一个字符串——"签名"，并将签名和其他参数传给 **h5sdk.config** 接口，完成鉴权。

> 生成签名及鉴权方法请见[H5JSSDK - 鉴权接口](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)；可参考文档里的方式，实现一个生成签名的后端接口，用来在 H5 页面上获取签名。
> 
> 注意：在生成签名时，需要携带**页面 URL 的 query**，即` ? `及其后面的全部内容。



#### **2. 获取 triggerCode**

网页应用获取消息的方式与小程序基本相同，主要区别是 H5 JS SDK 没有 tt.getHostLaunchQuery 方法，需要手动从 URL 中取出 triggerCode，参考代码：

```javascript 
let launchQuery = new URLSearchParams(location.search).get("bdp_launch_query");
if (!launchQuery) {
    console.log("bdp_launch_query not found in URL");
    return;
}
launchQuery = JSON.parse(launchQuery);
const triggerCode = launchQuery.__trigger_id__; 
``` 


#### **3. 确定目标会话与发送卡片**
确定目标会话（tt.getTriggerContext）和发送消息卡片（tt.sendMessageCard）的用法与小程序一致。可参考下方的完整实例。

#### **完整示例**
**引入 H5 JS SDK**：页面引入 H5 JS SDK 之后，在全局作用域下会有 window.h5sdk 和 window.tt 这两个对象。**注意：只有在Lark内嵌的网页里，才会存在这两个对象。**

```javascript 
<!-- 在html文档中 -->
<script type="text/javascript" src="https://s3.bytecdn.cn/ee/lark/js_sdk/h5-js-sdk-X.X.X.js"></script> 
``` 
:::note
==具体JSSDK版本号请前往[JSSDK 开发指南](/document/uYjL24iN/uITO4IjLykDOy4iM5gjM)获取==
:::


**JS 逻辑**：

```javascript 
const urlObj = new URL(location.href);
// 注意：带 query
const pageUrl = encodeURIComponent(
  urlObj.origin + urlObj.pathname + urlObj.search
);
// 获取签名。注意：这里的请求地址，请换成自己的后端接口地址
const getSignPromise = window.fetch(https://xxxx?url=${url});
getSignPromise
  .then((res) => res.json())
  .then((res) => {
    // 换成自己的 appId
    return Object.assign(
      {
        appId: "cli_xxxxx",
      },
      res.data
    );
  })
  .then((res) => {
    if (window.h5sdk) {
      // 接口鉴权
      window.h5sdk.config({
        // 下面这 4 个字段，都需要从生成签名的后端接口返回
        appId: res.appId,
        timestamp: +res.timestamp,
        nonceStr: res.noncestr,
        signature: res.signature,
        jsApiList: [
          // 声明需要使用的方法名
          "getTriggerContext",
          "sendMessageCard",
        ],
        onSuccess: (res) =>
          console.log(config: success ${JSON.stringify(res)}),
      });
      window.h5sdk.error((err) => {
        console.error("config error", JSON.stringify(err));
      });
      window.h5sdk.ready(() => {
        // 从页面 URL 中获取 triggerCode
        let launchQuery = new URLSearchParams(location.search).get(
          "bdp_launch_query"
        );
        if (!launchQuery) {
          console.log("bdp_launch_query not found in URL");
          return;
        }
        launchQuery = JSON.parse(launchQuery);
        const triggerCode = launchQuery.trigger_id;
        /**
         * 使用方式 一
         * 通过 triggerCode 字段指定会话
         */
        tt.sendMessageCard({
          triggerCode: triggerCode,
          cardContent: {
            update_multi: false,
            card: {
              // card content
            },
          },
          success(res) {
            console.log(res);
          },
          fail(res) {
            console.log(res);
          },
        });
        /*
         * 使用方式 二
         * 通过 openChadIDs 字段指定会话
         */
        // 获取当前会话的 openChatId
        tt.getTriggerContext({
          triggerCode: triggerCode,
          success(res) {
            const openChatId = res.openChatId;
            tt.sendMessageCard({
              openChatIDs: [openChatId],
              cardContent: {
                update_multi: false,
                card: {
                  // card content
                },
              },
              success(res) {
                console.log(res);
              },
              fail(res) {
                console.log(res);
              },
            });
          },
          fail(res) {
            console.log(res);
          },
        });
      });
    }
  }); 
``` 

:::note
注意：PC 端网页应用的侧边栏模式，在 3.37 及更高的版本中，才能正常使用。
:::





