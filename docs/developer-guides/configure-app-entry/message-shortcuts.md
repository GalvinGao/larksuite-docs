---
document_id: '6970625975390486534'
directory_id: '7021842990277787653'
title: 消息快捷操作
full_path: /ukzMukzMukzM/ugDN1YjL4QTN24CO0UjN
breadcrumb:
- Developer Guides
- Configure App Entry
- Message Shortcuts
document_type: GuideDocumentType
updated_at: 2023-07-27T02:27:56Z
source_url: https://open.larksuite.com/document/ukzMukzMukzM/ugDN1YjL4QTN24CO0UjN
---

# 如何接入“消息快捷操作”？


# 消息快捷操作是什么
“消息快捷操作”是Lark的一项开放能力，可连接会话消息与应用。通过这项能力，开发者可以让自己的应用出现在消息操作菜单中，用户则可以通过消息操作菜单快速打开应用、并将消息内容传递给应用处理。

通过“消息快捷操作”，可以极大的丰富消息的功能、提高效率，例如：
- 将对话内容快速创建为待办任务；
- 将他人在对话中反馈的问题快速沉淀至工具内；
- 将在群中发布的图片快速同步至企业文化相册中；
- 更可一次性选中多条对话消息，将其作为上下文，同步至任务的评论中，便于各方了解上下文。



::: note
  * 目前，消息快捷操作已支持**文本、富文本、图片、文件、视频、卡片消息**：
     * 当用户触发上述消息类型时，开发者可获取完整消息内容；
     * 当用户触发其他消息类型时，开发者可知悉用户触发了一条暂不支持的消息，进行清晰区分。

  * 当选择多条消息进行快捷操作时，消息条数不能超过**20**条
::: 



# 如何接入消息快捷操作
## 创建应用并设计承接页
如上文所述，用户可通过消息快捷操作，在会话中快速打开一个应用页面并将消息传递给应用。因此，创建应用并设计一个承接页是接入的重要一步：

1.首先，开发者需先**开启[小程序](/document/uQjL04CN/ukjL54SO)或[网页](/document/uQjL04CN/uAzM3QjLwMzN04CMzcDN)能力**。

2.其次，开发者需**基于小程序或网页应用设计一个专属承接页**。不同于从工作台进入应用，通过消息快捷操作进入应用往往具备明确的场景性。因此，需要结合用户使用场景设计承接页展示内容，例如：
> 工单工具可将工单创建页作为承接页，支持用户基于收到的消息快速创建工单；
> 提醒工具可将提醒创建页作为承接页，支持用户对消息设置稍后提醒。

3.除设计承接页的展示，开发者也需**在承接页中对用户传递的消息进行有效处理**。针对于如何获取与处理消息详情，将在下方“获取消息”模块中展开阐述。



## 启用能力
完成承接页设计后，可前往"开发者后台-应用功能-扩展"一栏启用消息快捷操作能力，并填写操作名称及对应承载页的 [applink](/document/uYjL24iN/ucjN1UjL3YTN14yN2UTN)。

> - 由于桌面端需要在会话内打开应用，**桌面端小程序应用 applink 的打开模式必须定义为 sidebar-semi 模式；网页应用则为 mode=sidebar**。
> -  例如：https://applink.larksuite.com/client/mini_program/open?appId=cli_9e2c876453ee900e&**mode=sidebar**&path=pages/plus/index

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/006919ba790bfe07f41666b6cd6c714b_UL1qAJDZ1K.png?lazyload=true&width=1640&height=801)
![开发文档msg1.gif](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8796b4b66c50fb7bb880163a9cbec1c5_wYTOlMU8Ds.gif?lazyload=true&width=1813&height=1231)

启用能力并发布新版本后，具备该应用可用性的用户即可在会话中看到应用的操作。

## 场景判断
开发者需要根据不同的场景来调用相应的Api来完成功能，下文将阐述如何判断是通过快捷操作打开的应用

### 小程序应用场景判断
通过[getLaunchOptionsSync](/document/uYjL24iN/uAzM1YjLwMTN24CMzUjN) Api 获取LaunchOptions，然后通过[场景值](/document/uYjL24iN/uQzMzUjL0MzM14CNzMTN)进行判断

示例代码：
```javascript
	 var options = tt.getLaunchOptionsSync() 
     var isFromMessageAction = options.scene == 1516
```
### 网页应用场景判断

:::note
注意：如果你为不同的场景配置了不同的承载页，此步骤可以跳过
:::

1. 获取当前页面的Url
2. 判断是通过快捷操作打开的应用
- pc: 从url 获取 `from` 参数值，判断`from`是否为`message_action`
- 移动端：从url获取 `required_launch_ability`, 判断 `required_launch_ability` 是否为 `message_action`

示例代码
```javascript
const searchParams = new URLSearchParams(location.search)
const launchAbility = searchParams.get('required_launch_ability')
const from = searchParams.get('from')
var isFromAction = false
console.log(`launchAbility${launchAbility}`)
console.log(`from${from}`)
boolean fromPc = ...// check ua
boolean fromMobile = ...// check ua
if (fromMobile && launchAbility) {
  isFromAction = launchAbility == 'message_action'
}
if (fromPc && from) {
  isFromAction = from == 'message_action'
}
```
## 获取消息
> 当用户在Lark中选中消息并触发应用的操作后，平台会通过如下接口向应用传递信息，请开发者提前做好处理，便于消息的完整获取。

### 小程序应用
小程序应用本身提供了获取消息的能力，可通过如下方式获取信息：
#### 获取 triggerCode
- 通过 [tt.getHostLaunchQuery](/document/uYjL24iN/ugzM4UjL4MDO14COzgTN) 获取 triggerCode。
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

#### 获取内容详情
通过 [tt.getBlockActionSourceDetail](/document/getBlockActionSourceDetail) 获取消息的具体内容。

::: note
`tt.getTriggerContext` 在消息快捷操作场景不支持使用
:::

#### 处理建议
由于目前仅可支持开发者获取6类消息详情；因此，**在处理 messages 时建议优先判断 support 状态**：
- support 是 true 时 content 返回的是 json 串；
- support 是 false 时 content 返回的是字符串；

同时，建议当返回 false 时，也在应用承载页中以 toast、文案提醒等方式，给予用户明确反馈，告知其触发的是暂不支持的消息类型，避免产生疑惑。

#### 代码示例
```js
// 代码示例
tt.getHostLaunchQuery({
    success(res) {
        if (res.launchQuery) {
            let json = JSON.parse(res.launchQuery)
            if (json && json.__trigger_id__) {
                // getHostLaunchQuery获取triggerCode的方法
                tt.getBlockActionSourceDetail({
                    triggerCode: json.__trigger_id__,
                    success(res) {
                        console.log('success', res)
                    },
                    fail(res) {
                        console.log('fail', res)
                    }
                })
            } else {
                tt.showToast({title: 'get code fail'})
            }
        } else {
            tt.showToast({title: 'get code fail'})
        }
    },
});
```


### 网页应用

#### 接口鉴权
**在调用 H5 接口前，请先完成鉴权**。即基于页面 URL，使用固定的算法生成一个字符串——"签名"，并将签名和其他参数传给 **h5sdk.config** 接口，完成鉴权。

> 生成签名及鉴权方法请见[H5JSSDK - 鉴权接口](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)；可参考文档里的方式，实现一个生成签名的后端接口，用来在 H5 页面上获取签名。
> 
> 注意：在生成签名时，需要携带**页面 URL 的 query**，即 ? 及其后面的全部内容。


#### 获取 triggerCode
网页应用获取消息的方式与小程序基本相同，主要区别是 H5 JS SDK 没有 tt.getHostLaunchQuery 方法，需要手动从 URL 中取出 triggerCode。可参考：

```javascript 
let launchQuery = new URLSearchParams(location.search).get("bdp_launch_query");
if (!launchQuery) {
    console.log("bdp_launch_query not found in URL");
    return;
}
launchQuery = JSON.parse(launchQuery);
const triggerCode = launchQuery.__trigger_id__; 
``` 

#### 内容详情
获取消息内容的接口为 [tt.getBlockActionSourceDetail](/document/getBlockActionSourceDetail)，用法与小程序一致，亦可参考下方的完整实例。

::: note
`tt.getTriggerContext` 在消息快捷操作场景不支持使用
:::

#### 代码示例
**引入 H5 JS SDK**

页面引入 H5 JS SDK 之后，在全局作用域下会有 window.h5sdk 和 window.tt 这两个对象。**注意：只有在Lark内嵌的网页里，才会存在这两个对象**。

```javascript 
<!-- 在html文档中 -->
<script type="text/javascript" src="https://s3.bytecdn.cn/ee/lark/js_sdk/h5-js-sdk-1.4.13.js"></script> 
``` 

**JS 逻辑**

```javascript 
const urlObj = new URL(location.href);
// 注意：带 query
const pageUrl = encodeURIComponent(urlObj.origin + urlObj.pathname + urlObj.search);
// 获取签名。注意：这里的请求地址，请换成自己的后端接口地址
const getSignPromise = window.fetch(`https://xxxx?url=${url}`);

getSignPromise
.then(res => res.json())
.then(res => {
  // 换成自己的 appId
  return Object.assign({ appId: 'cli_xxxxx' }, res.data);
})
.then(res => {
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
        'getBlockActionSourceDetail'
      ],
      onSuccess: (res) => console.log(`config: success ${JSON.stringify(res)}`)
    });
    window.h5sdk.error(err => {
      console.error('config error', JSON.stringify(err));
    });
    window.h5sdk.ready(() => {
      // 从页面 URL 中获取 triggerCode
      let launchQuery = new URLSearchParams(location.search).get("bdp_launch_query");
      if (!launchQuery) {
        console.log("bdp_launch_query not found in URL");
        return;
      }
      launchQuery = JSON.parse(launchQuery);
      const triggerCode = launchQuery.__trigger_id__;
      
      // 调用方法，传入 triggerCode，获取消息内容
      tt.getBlockActionSourceDetail({
         triggerCode: triggerCode,
         success(res) {
           console.log('success', res)
         },
         fail(res) {
           console.log('fail', res)
         }
      });
    });
  }
});
 
``` 

注意：PC 端网页应用的侧边栏模式，在 3.37 及更高的版本中，才能正常使用。


