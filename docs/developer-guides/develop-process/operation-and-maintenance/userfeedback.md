---
document_id: '7278977493923692549'
directory_id: '7278895853091241990'
title: 用户反馈
full_path: /tools-and-resources/userfeedback
breadcrumb:
- Developer Guides
- Develop Process
- Operation and Maintenance
- UserFeedback
document_type: GuideDocumentType
updated_at: 2023-09-19T09:37:01Z
source_url: https://open.larksuite.com/document/tools-and-resources/userfeedback
---

# 用户反馈

Lark开放平台为企业自建应用提供了反馈入口，通过接收来自Lark客户端的用户反馈，使应用管理员可以及时发现应用存在的问题以及有价值的用户建议，是提升应用质量的重要途径。

## 功能简介

:::note
企业自建应用和商店应用均支持用户反馈功能。
:::

以网页应用为例，当用户在Lark客户端使用应用时，如果遇到功能 Bug、操作体验不佳或者对操作逻辑产生疑问，则可以通过右上角的反馈渠道，提交故障反馈或者产品建议，后续由应用管理员接收并处理用户反馈。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1b4d046688784e82feafe2c926b45143_w8pczuhVhN.png?height=1194&lazyload=true&maxWidth=700&width=2162)


## 功能特性

- 固定反馈入口，按需快捷反馈
    
    对于使用者来说，通过一个反馈入口，即可提交所有应用的反馈意见。无论是哪个应用，用户都能根据自己的需要来反馈问题寻求帮助，业务运转更流畅。

- 收集用户声音，降低运营成本
    
    对于开发者来说，这一功能提供了一个稳定的用户反馈收集渠道。开发者可以查看用户反馈的具体内容以及收集情况的变化趋势，应用运营更高效。

- 关联日志工具，轻松定位问题
    
    反馈功能与日志检索功能紧密关联。在开发者后台的用户反馈面板中，开发者只需点击对应的日志信息，就能轻松查看用户反馈故障前后的日志情况，无需多余沟通便可快速定位问题。

## 使用功能

### 处理用户反馈

当收到用户反馈后，应用开发者需要前往应用管理页面查看并处理用户反馈。

1. 登录[开发者后台](https://open.larksuite.com/app)。找到指定应用并进入应用详情页。

2. 在左侧导航栏，选择 **运营监控** > **用户反馈**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1963273fc58577a15f1e50d7d1ea2713_hPcA1yYMbJ.png?height=1284&lazyload=true&maxWidth=600&width=2782)

3. 查看并处理故障反馈。
    
    1. 在 **故障反馈** 页面查看收到的反馈，你可以设置反馈时间、故障类型等过滤条件进行检索。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c5488cf63e655dfb0633251dcc5e80f3_GaiFAvkhf5.png?height=1092&lazyload=true&maxWidth=600&width=2316)

    2. 在具体的反馈中，通过 **服务端日志**、**客户端日志** 入口，排查故障原因。
        
       
        日志数据将在应用的 **运营监控** > **日志检索** 功能中查看，该功能的介绍参见[日志检索](/document/tools-and-resources/open-api-log-query)。
        
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6f4a1caf83d0c2c90a50e96858483cda_jvPjVqBRZV.png?height=430&lazyload=true&maxWidth=600&width=2258)

    3. 选中反馈左侧的复选框，根据实际处理情况，将反馈标记为处理中、已处理或者已关闭。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a06ab6549d8f52c878caa02ae9ebbeb5_m2sL4Al8LQ.png?height=1014&lazyload=true&maxWidth=600&width=2274)

4. 查看并处理产品建议。
    
    1. 在 **产品建议** 页面查看收到的建议，你可以设置反馈时间、处理状态等过滤条件进行检索。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/086d5877031f092b084053543c5d741b_aVHRyYdHk9.png?height=1060&lazyload=true&maxWidth=600&width=2300)

    2. 选中建议左侧的复选框，根据实际处理情况，将建议标记为处理中、已处理或者已关闭。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4c3fa82d426f4a0e7a23c61673f333d5_94CBONgLvj.png?height=1062&lazyload=true&maxWidth=600&width=2270)


### 查看反馈数据趋势

1. 登录[开发者后台](https://open.larksuite.com/app)。找到指定应用并进入应用详情页。

2. 在左侧导航栏，选择 **运营监控** > **应用质量看板** > **用户反馈**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1963273fc58577a15f1e50d7d1ea2713_hPcA1yYMbJ.png?height=1284&lazyload=true&maxWidth=600&width=2782)

3. 在 **反馈统计** 页签，可查看指定时间段、指定反馈类型、指定处理状态的数据统计趋势。
    
:::note
在页面右侧点击 **导出数据**，可将用户反馈数据导出至本地。
:::
    
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fc2be478c9b6a18ebedc4c9b8dc2af2e_dY7UltgBfM.png?height=1230&lazyload=true&maxWidth=600&width=2272)


## 常见问题

### 应用的使用者如何提交用户反馈？

以一个简单的网页应用为例，介绍应用的使用者如何提交反馈。

#### 桌面端

1. 登录Lark客户端。

2. 在工作台打开指定的网页应用。

3. 在应用右上角的更多图标内，点击 **反馈**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d877810b9b6920f4a6c6270dec8a228f_sKhJ0Pb7gc.png?height=948&lazyload=true&maxWidth=600&width=2694)

4. 在反馈对话框，根据实际情况自行选择 **故障反馈** 或 **产品建议**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/16c707593614cf03c2b938480cbd0d81_QdfmA9l63V.png?height=828&lazyload=true&maxWidth=600&width=2380)

    - 如果选择 **故障反馈**，则需要填写故障的类型、时间、描述、截图以及是否同意上传应用运行日志，并点击 **提交**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0abc43a831900d3f0d696d7bad48d539_htABPPfbns.png?height=1122&lazyload=true&maxWidth=600&width=2088)

    - 如果选择 **产品建议**，则需要填写建议内容与截图，并点击 **提交**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/94acc501220ff0c42bdbc02f2bc5b5e5_gAPOfDUC7w.png?height=1128&lazyload=true&maxWidth=600&width=2098)


#### 移动端

1. 登录Lark客户端。

2. 在工作台打开指定的网页应用。

3. 点击页面右上角的更多按钮，并选择 **反馈**。
    
    操作流程如下图所示，你可以根据实际情况选择 **故障反馈** 或者 **产品建议**。
    
    - 选择 **故障反馈**，则需要填写故障的类型、时间、描述、截图以及是否同意上传应用运行日志，并点击 **提交**。
    
    - 选择 **产品建议**，则需要填写建议内容与截图，并点击 **提交**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/82b0be6f2fb12711ebde5fd7dc9012f9_bXSNsLpMdY.png?height=919&lazyload=true&maxWidth=800&width=1640)
