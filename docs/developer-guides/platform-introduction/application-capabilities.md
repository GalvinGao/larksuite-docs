---
document_id: '7074246291802832901'
directory_id: '7260031291679473669'
title: 应用能力简介
full_path: /home/app-types-introduction/robots-web-applications-and-mini-programs
breadcrumb:
- Developer Guides
- Platform Introduction
- Application capabilities
document_type: GuideDocumentType
updated_at: 2024-12-05T10:45:58Z
source_url: https://open.larksuite.com/document/home/app-types-introduction/robots-web-applications-and-mini-programs
---

# 应用能力简介

根据应用的能力形态，Lark应用主要可分为 **机器人、网页、小程序、小组件**四类。我们常看到的 **Lark工作台** 中一系列应用均为小程序、网页应用、小组件。

:::html

<table>
  <thead>
    <tr>
      <th style="width: 12%;">应用形态</th>
      <th style="width: 22%;">小程序（不推荐）</th>
      <th style="width: 22%;">网页</th>
      <th style="width: 22%;">机器人</th>
      <th style="width: 22%;">工作台小组件</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="font-weight: bold;">功能示意</td>
      <td style="text-align: center;">
        <br>
        <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/bcd4368797ddb108a10ebb56bedb8654.png?height=1402&amp;lazyload=true&amp;width=1640" alt="图片" />
        <br>
      </td>
      <td style="text-align: center;">
        <br>
        <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/35f78ccc64f575665b9ce4a6e7e81cf7.png?height=1402&amp;lazyload=true&amp;width=1640" alt="图片" />
        <br>
      </td>
      <td style="text-align: center;">
        <br>
        <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/e58f31f14b2c3dd42040aa54a35bf8da.png?height=864&amp;lazyload=true&amp;width=1052" alt="图片" />
        <br>
      </td>
      <td style="text-align: center;">
        <br>
        <img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/38b2e03f33e69626dd4a783b6cb0b104.png?height=1402&amp;lazyload=true&amp;width=1640" alt="图片" />
        <br>
      </td>
    </tr>
    <tr>
      <td style="font-weight: bold;">适用场景</td>
      <td>可以在Lark客户端运行的应用，可调用Lark组件，使<strong>体验达到原生水平</strong>。</td>
      <td><strong>快速接入已有的网页应用</strong>，用户可以通过<strong>Lark客户端免登录</strong>快速进入。</td>
      <td>与用户<strong>在聊天中交互</strong>的应用，它可以向用户或群组自动发送消息，响应用户的消息，并能进行群组管理。</td>
      <td>将<strong>数据图表、图文资讯</strong>等信息添加到工作台。</td>
    </tr>
    <tr>
      <td style="font-weight: bold;">开发工具</td>
      <td><a href="/document/uYjL24iN/ucDOzYjL3gzM24yN4MjN">Lark开发者工具</a></td>
      <td><a href="/document/uYjL24iN/ucDOzYjL3gzM24yN4MjN">Lark开发者工具</a></td>
      <td>纯服务端开发，提供<a href="https://open.larksuite.com/tool/cardbuilder?lang=zh-CN">消息卡片搭建工具</a></td>
      <td><a href="/document/uYjL24iN/ucDOzYjL3gzM24yN4MjN">Lark开发者工具</a></td>
    </tr>
    <tr>
      <td style="font-weight: bold;">是否支持服务端 API</td>
      <td>支持</td>
      <td>支持</td>
      <td>支持</td>
     <td>支持</td>
    </tr>
    <tr>
      <td style="font-weight: bold;">接入方式</td>
      <td>支持<a href="/document/uYjL24iN/uEzMuEzMuEzM">组件库</a>和 <a href="/document/uYjL24iN/ucjL34yN/gadget-api-list">小程序 API </a></td>
      <td><a href="/document/uYjL24iN/uMTMuMTMuMTM/introduction#3dde05bd">H5-JS-SDK</a></td>
      <td><a href="/document/home/develop-a-bot-in-5-minutes/create-an-app">开发机器人应用</a></td>
      <td><a href="/document/uAjLw4CM/uYjL24iN/block/api/api-introduction">小组件 API</a></td>
    </tr>
    <tr>
      <td style="font-weight: bold;">是否支持应用商店</td>
      <td>支持</td>
      <td>支持</td>
      <td>支持</td>
      <td>不支持</td>
    </tr>
  </tbody>
</table>
:::

# 机器人

机器人 ( Bot ) 是一种可以基于会话与用户进行交互的应用，是触达用户的最常用渠道。Lark机器人可以与Lark日历、审批、云文档等应用、第三方主流业务系统、企业自建系统打通，通过向用户发送消息卡片的形式，在Lark中实现一站式聚合各类应用的通知，例如发送监控告警、待办事项提醒、公司活动通知、数据日报推送、库存预警等。

## 机器人的优势

* **嵌入式的体验**：可以在聊天中通过消息完成内容的触达、监听和响应等操作。借助机器人能力，你可以将企业系统集成进Lark，在Lark内获得一站式的系统使用体验。您可以用机器人能力实现自动推送消息，在聊天里进行简单的交互，以及自动化的群组管理。
* **开发成本相对较低**：只需进行服务端开发，就能实现内容呈现友好、可互动的机器人。并且一次开发后，可以被企业内的其他成员轻松使用。
* **支持丰富的消息类型**：你不仅可以用机器人发送文本、图片、文件、视频消息，还能进一步发送呈现样式更友好、支持互动的消息卡片，使推送内容更好地触达用户。
* **支持丰富的服务端能力**：机器人可以调用丰富的服务端能力，实现各种各样的工作流自动化，例如使用机器人在话题群中收集用户的反馈建议，一键同步到多维表格，高效完成Lark群组中零散信息的收集。

## 机器人的类型

Lark支持两种类型的机器人：**应用机器人**及**自定义机器人**，两者支持的能力以及使用方式有所差异，您可以根据业务场景选择合适的机器人类型。

:::html
<table style="width:100%;">
  <colgroup>
    <col style="width: 10%">
    <col style="width: 15%">
    <col style="width: 18%">
    <col style="width: 24%">
    <col style="width: 24%">
    <col style="width: 21%">
  </colgroup>
  <thead>
    <tr>
      <th>机器人类型</th>
      <th>支持的能力</th>
      <th>应用场景</th>
      <th>开发方式</th>
      <th>使用方式</th>
      <th>使用范围</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>应用机器人</td>
      <td>在应用管理员通过权限审核后，应用机器人可以调用Lark丰富的开放接口，获取、使用用户和租户资源。</td>
      <td>将外部系统集成进Lark，让机器人能进行互动、群管理。<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e39ec6adb0131034e9ba0f4a13135f14_TrjvijAMtP.png">
</td>
      <td>在 <a href="https://open.larksuite.com/app">开发者后台</a> 中创建应用机器人，完成服务端开发，申请发布并经过租户的应用管理员审核通过后，即可使用。 您可以参考教程 <a href="/document/home/message-development-tutorial/introduction">机器人自动拉群报警</a>，了解开发一个可直接在聊天里互动的机器人的开发过程。</td>
      <td>完成服务端开发并且发布上线后，在应用可用范围内的用户，可以在Lark的工作台、搜索栏及消息会话列表中找到该机器人，如下图所示：<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6a6cc1d91a5c78f1a3dd9d52dff5a7d1_XPChVLRqxK.png">
 用户可以直接与你创建的机器人单聊，或在 <b>群设置&gt;群机器人</b> 面板中将这个机器人添加进群聊使用。</td>
      <td>可用范围受本租户的应用管理员管控。 不支持将应用机器人添加进外部群。</td>
    </tr>
    <tr>
      <td>自定义机器人</td>
      <td>仅能对群聊进行单向的消息推送，不支持调用Lark丰富的开放接口，不具有 任何用户、租户数据访问权限。 对消息卡片类型的交互模块，仅支持 <a href="/document/hukTMukTMukTM/uYjNwUjL2YDM14iN2ATN#7f69ddbb">包含链接跳转的消息卡片</a>。</td>
      <td>仅需要临时性地在群聊中完成比较固定的消息推送。<img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2b49bddbb30a38c19c7ee69cf0fcc077_2b8O9aa8i4.png"></td>
      <td>在 群设置 &gt; 群机器人 面板中快捷创建一个自定义机器人，无需经过租户管理员审核，即可在当前群聊中通过调用 webhook 地址的方式完成消息推送，配置上更便捷。 详细的自定义机器人配置过程参看<a href="/document/client-docs/bot-v3/add-custom-bot">自定义机器人指南</a>。</td>
      <td>通过服务端调用 webhook 地址，即可将外部系统的通知消息即时推送到群聊中。我们也提供了自定义关键词、IP白名单和签名三种维度的安全配置，控制 webhook 的调用范围。 以 curl 指令为例，请求示例如下： <pre><code>HTTP curl -X POST -H "Content-Type: application/json" \
-d '{"msg_type":"text","content":{"text":"request example"}}' \
https://open.larksuite.com/open-apis/bot/v2/hook/xxxxxxxxxxxxxxxxx</code></pre>
</td>
      <td>只能在被添加的群聊内使用，不能与机器人单聊。 支持将自定义机器人添加进外部群。</td>
    </tr>
  </tbody>
</table>
:::

# 网页应用

Lark客户端网页（Web）应用指的是用 H5 方式开发，可以运行在Lark客户端内的应用。网页应用可以调用丰富的Lark客户端开放接口（客户端 API，也称为 JSAPI），包含手机系统功能和通讯录、云文档等Lark客户端功能，也可以享受到客户端侧的性能优化，使你的网页应用能够接近原生体验。

## 网页应用的优劣势

:::html
<table>
  <thead>
    <tr>
      <th style="width: 55%;">网页应用的优势</th>
      <th style="width: 45%;">网页应用的缺点</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>
        <ul>
          <li>开发速度快，一端开发多端运行。</li>
          <li>无需发版本，可动态更新，针对运营活动等需要频繁变更内容的场景，开发成本更低。</li>
          <li>使用网页应用模式<strong>迁移到Lark的工作台内的成本是非常低的</strong>，只需要简单配置和少量开发就可以正常使用。</li>
        </ul>
      </td>
      <td>
        <ul>
          <li>由于渲染层和数据层都需要在服务器端维护，打开应用时才加载到客户端，交互响应的速度比原生更慢。</li>
          <li>对网络质量的要求比较高，没有网络或弱网情况下严重影响可用性。</li>
        </ul>
      </td>
    </tr>
  </tbody>
</table>
:::

## 网页应用的能力

网页应用提供了的开发能力支持主要包括：
* **免登支持**：接入Lark免登打通Lark和应用的用户体系，参考 [步骤三：免登流程（可选）](/document/uYjL24iN/uMTMuMTMuMTM/development-guide/step-3)。
* **SDK 工具包**：引入 H5 SDK 即可调用Lark的原生能力，参考 [网页应用开发指南](/document/uYjL24iN/uMTMuMTMuMTM/introduction#3dde05bd)
* **开放能力**：提供 web-meta、[导航栏控制](/document/uYjL24iN/uYjMy4iNyIjL2IjM/setnavigationbar) 等容器开放能力，也提供了网页应用可以调用的 [JSAPI 总览](/document/uYjL24iN/uMTMuMTMuMTM/)。
* **低成本迁移**：在 [开发者后台](https://open.larksuite.com/app) 你可以为 **移动端** 和 **桌面端** 分别配置主页地址。如果企业中已经存在大量的 H5 办公应用，使用网页应用模式迁移到Lark的工作台内的成本是非常低的，只需要简单配置和少量开发就可以正常使用。

![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/37469e1438f9639577140086a9c6a748_w4whdRYL8s.png?height=900&lazyload=true&maxWidth=600&width=2878)

# 小程序（不推荐）
:::warning
小程序将不再升级迭代，推荐选择网页应用能力。
:::

小程序是一种无需下载、用完即走的新应用，它具备丰富的框架能力和优秀的产品体验，开发者以此为载体提供服务，并能获得便捷的传播和分享。

Lark小程序运行在Lark客户端上(包括 PC 端和移动端)，客户端为小程序提供运行时所需的环境和各种能力，支撑起小程序应用，我们将这套环境称为 **宿主环境**。通过宿主环境提供的各种能力，小程序可以实现很多 Web 网页无法完成的功能，获得更流畅的体验和交互。

小程序从框架上分出了逻辑层和视图层。逻辑层加载小程序应用的 Javascript 脚本，负责处理业务逻辑，而视图层负责渲染 TTML 模板和 TTSS 样式，进而显示页面，更多详情参考 小程序介绍。
小程序的逻辑层和视图层是 2 个独立的线程来管理，一个小程序应用只有一个 JSCore 线程，多个 Webview 线程。

* 逻辑层的线程启动 JSCore 引擎去执行 JavaScript 脚本，处理业务逻辑。
* 视图层的每个页面使用一个 Webview 进行渲染，多个页面则对应多个 Webview。 

![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/598e24f442b0fdbf3e5307d0ee619700_UVY1lIx4pu.png?height=921&lazyload=true&maxWidth=600&width=1280)

## 小程序的优势

相对于网页应用，小程序在技术上有更多创新：

* **多平台适配**：一套代码，Android、iOS、PC 多平台运行。区别于业界的小程序方案，Lark小程序统一了不同客户端的小程序引擎，从而使得为移动端开发的小程序可以不经适配，直接在桌面客户端上通过多模式运行（Window 模式、Sidebar 模式、 AppCenter 模式），更多详情参考 [PC 小程序介绍](/document/uYjL24iN/uIjNzUjLyYzM14iM2MTN#37991f21)。

	![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/98aa0343b50d2a4b87600a41cfb46845_78FR5rW4iG.png?height=1010&lazyload=true&maxWidth=650&width=1640)
    
* **体验流畅**：本质是客户端 Native 开发，应用使用体验可类比 Native app 原生，更为流畅。
* **易上手**：框架、语法与微信小程序类似，上手门槛低。
* **易管理**：小程序代码包是一个不可分割的整体，不用担心 CDN 上出现资源碰撞，而必须在资源上加上 hash。
* **可本地运行**：Client-Server 模式，非 Browser-Server 模式。


## 小程序的能力

相比网页应用，小程序应用在开放能力的支持上比网页应用更多，交互的整体体验上也比网页应用更优秀。
小程序也是Lark客户端原始能力不断对外开放的一个出口。Lark主要通过开放组件和小程序接口的形式，在不断开放各种移动端本地的能力和Lark生态相关的能力。

* 小程序支持的组件内容，可参考 [组件](/document/uYjL24iN/uEzMuEzMuEzM)。
* 小程序的 API 开放能力，可参考 [API 概述](/document/uYjL24iN/uADOy4CM4IjLwgjM)。


# 工作台小组件

**Lark小组件（Block）** 是Lark独创的轻量级功能块，它被定义为一个集渲染、交互、数据为一体的信息单元载体，实现信息在Lark套件内的顺畅流转与消费。
作为一个信息单元载体，小组件不依附于其它主体，和小程序、机器人、网页应用一样，可以作为独立的应用发布。Lark套件内开放出来承载小组件的场景，称之为宿主，小组件可以被添加在宿主的指定位置。

**工作台小组件**是Lark工作台中的一种应用展示形态，通过功能模块向用户实时呈现应用或企业模块的核心内容。比起应用，小组件提供了更高效、便捷的内容呈现方式。用户无需打开应用，只需在工作台扫一眼小组件，即可了解重要信息。更多详情参考[工作台小组件概述](/document/uAjLw4CM/uYjL24iN/block/guide/hosting-scenario-introduction/workplace)。

![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7b04044596baebcce392222c041fab43_rhh700do8R.png?height=1002&lazyload=true&maxWidth=700&width=2342)

## 工作台小组件的优势

你可以按需设计并开发小组件，将其添加至工作台，灵活定制企业个性化工作台。
* **更直观的信息整合**：无需打开第三方应用，小组件可以作为视图块在工作台中进行展示。
* **更便捷的数据流转**：无需切换多个应用，小组件作为功能块在工作台中进行交互。
* **一次学习，多宿主开发：** 无需持续高门槛学习，当出现新的宿主时，开发者仅需了解新的业务场景和相关接口。

## **工作台**小组件的能力
工作台小组件（Block）在设计上，沿用了Lark小程序的 DSL 语言，降低开发者的学习门槛：
* 小组件（Block）框架内置了很多基础组件，参考 [基础组件](/document/uAjLw4CM/uYjL24iN/block/component/basic-components)。
* 小组件（Block）框架提供了一系列的 API 接口，参考 [API 简介](/document/uAjLw4CM/uYjL24iN/block/api/api-introduction)。


# 如何选择合适的应用能力？
选择开启不同的应用能力时，可参考以下原则：

**选择网页应用**：
* 如果你的应用已经有基于 H5 技术框架实现的版本，希望迁移到Lark，开发周期紧张，可以考虑使用网页应用接入免登功能快速实现。
* 如果你的应用会频繁更新内容，并且是偏向于运营类的内容，且需要在Lark之外的环境中传播，可以考虑使用网页应用。

**选择小程序**：
* 如果你的应用被用户高频使用，并且需要深度使用到手机或Lark的能力，希望获得接近原生应用的极致体验，我们建议使用小程序框架。
* 如果你开发的应用是商店应用，希望在Lark上架，被更多的外部租户使用，那么用户体验是第一位的，直接使用小程序框架是不二选择。

**选择小组件**：
* 需要将**数据图表、图文资讯**等信息添加到工作台，通过功能模块向用户实时呈现应用或企业模块的核心内容。

**选择机器人**：
* 需要与用户在聊天中进行交互，向用户或群组自动发送消息，响应用户的消息，并进行群组管理。

**搭配组合**：Lark的应用能力可以组合使用，你可以为**小程序/网页应用开启机器人能力**，来更好地与用户交互，例如：
  * 配合事件订阅，把原本小程序/ H5 应用中触发的业务通知发送到Lark的会话或者群聊中。
  * 使用机器人结合消息卡片的能力，进行通知下发或让用户与应用进行轻量交互。


