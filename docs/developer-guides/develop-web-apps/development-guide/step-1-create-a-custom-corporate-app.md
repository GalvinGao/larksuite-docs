---
document_id: '7199617266244370438'
directory_id: '7130175417072861190'
title: 步骤一：创建企业自建应用
full_path: /uYjL24iN/uMTMuMTMuMTM/development-guide/step1
breadcrumb:
- Developer Guides
- Develop Web Apps
- Development Guide
- 'Step 1: Create a custom corporate app'
document_type: GuideDocumentType
updated_at: 2024-10-21T03:46:40Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMuMTMuMTM/development-guide/step1
---

# 步骤一：创建并配置自建应用

在Lark开放平台中开发网页应用时，你需要先创建一个企业自建应用，并添加网页应用能力。

## 操作步骤

1. 登录[Lark开发者后台](https://open.larksuite.com/app)。

2. 在开发者后台首页，单击 **创建企业自建应用**，填写应用名称、描述以及图标信息，然后单击 **创建**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d1fabf67d6ec45ae42ce78027bb6ff4f_YhwyD3pn6Q.png?height=1374&lazyload=true&maxWidth=400&width=1166)

3. 在应用详情页，进入 **应用能力** > **添加应用能力** 页面。

4. 添加 **网页应用** 能力。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/af98e8be39db7c2382d711c95cc99740_MNocJYcSvH.png?height=1206&lazyload=true&maxWidth=600&width=1952)

5. （可选）进入 **应用能力** > **网页应用** 页面，配置网页应用主页地址。
    
    - 如果你已开发完成了网页应用程序，且获取了程序的公网访问地址，则可以直接在此处配置。
    
    - 如果你还没有开发网页应用程序，则当前步骤无需填写地址，可参考指南提供的示例操作（[步骤二（可选）：鉴权调用 JSAPI](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)、[步骤三（可选）：应用免登流程](/document/uYjL24iN/uMTMuMTMuMTM/development-guide/step-3)）完成应用的开发后，再返回此处填写程序对应的访问地址。
   
:::note
- 实际生产环境中，此处配置的地址需要是公网地址。在开发调试阶段可以使用反向代理工具 ngrok 获得，使用方式参见[配置事件订阅](/document/home/develop-a-bot-in-5-minutes/step-5-configure-event-subscription)中的步骤 1~3。

- 若公司网络安全策略不允许使用反向代理工具，或想快速体验接入流程，可暂时先使用本地环境调试，填写 `http://127.0.0.1:3000`即可。
:::

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1a93257d3d91beb47e666f491f474f1b_dMS2xCB80e.png?height=1272&lazyload=true&maxWidth=600&width=1680)

6. （可选）选择桌面端主页的打开方式。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a6783bb6975a9c384cde7e25577fb868_ifHe9BJixz.png?height=1274&lazyload=true&maxWidth=600&width=1672)

	- 在Lark V7.4 以下的版本中，网页应用首页会在Lark内打开，通过 `window.open` 或者 `<a target="_blank>` 方式跳转新页面将会跳转至浏览器打开。
	- 在Lark V7.4 及以上版本中，网页应用首页仍然会在Lark内打开，你可以选择 `window.open` 或者 `<a target="_blank>` 方式跳转新页面时的打开方式为 **在Lark内新标签页打开** 或 **跳转至浏览器打开新页面**。
		- 如果选择 **在Lark内新标签页打开**，这种打开场景下，网页应用的表现有一些特性：
    		- 为了便于区分一个应用打开的不同标签页，这种场景下打开的应用，在标签页中显示的名称会展示网页本身定义的名称，而不再是应用名称。
    		- 如果用户将网页应用的某个标签页固定到主导航中，则下次重新打开时，会直接进入**标签页链接对应的页面**，而不再是从网页应用首页开始进入。你需要针对这种场景做好兼容（例如，任意子页面都需考虑是否需要进行用户登录和网页鉴权）。
    
		- 如果选择 **跳转至浏览器打开新页面**，那么网页应用打开的表现将与Lark版本 V7.4 以下保持一致，即：
    		- 标签页中显示的是网页应用的名称。
    		- 如果用户将该标签页固定到主导航，则下次重新打开时，会重新进入**网页应用的首页**。如果网页应用的首页变更了，则也会进入新的首页。

## 常见问题

本章节主要包含桌面端主页打开方式相关的常见问题。

### 配置桌面端主页打开方式的影响范围是什么？

1、网页应用通过中 `window.open` 或者 `<a target="_blank>` 方式跳转新页面受该打开方式的影响，通过 [openSchema API](/document/uYjL24iN/ukzN4IjL5cDOy4SO3gjM) 跳转新页面时不受到该打开方式的控制。

2、如果网页应用通过`window.open` 或者 `<a target="_blank>` 方式 [打开普通网页Applink](/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-the-web-view-in-feishu-to-access-the-specified-url)，则在Lark端内打开该普通网页，且该普通网页中跳转新页面不再受到网页应用打开方式的控制，与Lark会话中打开该链接的打开方式一致。


3、Lark中**独立窗口方式**打开的网页应用跳转 暂时**不受到**该打开方式的影响，通过 `window.open` 或者 `<a target="_blank>` 方式打开新页面页仍然跳转浏览器，后续将会逐步支持该配置。



### 当网页应用打开的新页面中配置的鉴权结果指向其他应用，页面将会如何打开？

如果网页应用 A 打开的新页面中做了[网页应用鉴权逻辑](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)，鉴权结果为应用 B，则该新页面中打开页面跳转浏览器还是Lark内打开（受到应用 B 的设置控制，不再受到应用 A 控制）。

### 配置桌面端主页打开方式后，应用首页仍然在Lark内打开，我如何配置可以让应用首页直接跳转到浏览器打开？

在Lark V7.7 及以上版本中，你可以进入[开发者后台](https://open.larksuite.com/app) > 应用详情页 > **网页应用** 配置页，在 **桌面端主页** 配置的 URL 后拼接参数 `lk_jump_to_browser=true`，后续该网页应用可直接跳转浏览器打开。URL 示例值：`https://open.larksuite.com/?lk_jump_to_browser=true`。

更多详细说明，参考 [配置网页跳转浏览器打开](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/configure-webpage-to-open-in-browser)。
  
