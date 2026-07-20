---
document_id: '7156039415051059205'
directory_id: '7083440791243243525'
title: 步骤二：查询日历信息并订阅日历日程变更事件
full_path: /home/calendar-event-sync/subscribe-to-events
breadcrumb:
- Home
- Auto-fetch Calendar Events
- 'Step 2: Query calendar information and subscribe to calendar schedule change events'
document_type: GuideDocumentType
updated_at: 2024-12-10T11:29:09Z
source_url: https://open.larksuite.com/document/home/calendar-event-sync/subscribe-to-events
---

# 步骤二：查询日历信息并订阅日历日程变更事件

在本步骤中，你将通过已创建应用的鉴权凭证，查询测试账号的日历信息，并订阅日历变更事件。

## 操作步骤

1. 打开 [API调试台](https://open.larksuite.com/api-explorer)。

2. 在页面左上角点击 **切换应用**，并选择步骤一中创建的应用。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c8498e2d23cc01812cbbc23152b22d81_I56Cn5D2BG.png?height=1436&lazyload=true&maxWidth=600&width=2136)

3. 调用[查询主日历信息](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary)接口，获取当前登录用户的主日历信息。
    
    1. 在 API 调试台左侧 API 列表中，选择 **日历** > **日历管理** > **查询主日历信息**。
    
    2. 在左侧 **查看鉴权凭证** 区域，获取 **user_access_token** 信息，并在 **请求头** 中，点击 **切换为 user_access_token**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9b458594b18ba387389b0d98caec2d6a_3LwaMkv3AQ.png?height=1088&lazyload=true&maxWidth=600&width=2562)

    3. 点击进入 **查询参数** 页签，自行选择 **user_id_type** 类型。
        
        **user_id_type** 用于指定用户 ID 类型，本示例操作选择 **user_id**。
        

        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/87f55687562866d406830606a539639c_BWYCHTsmJA.png?height=812&lazyload=true&maxWidth=600&width=2882)

    4. 在页面右上角点击 **开始调试**。
       
        调用成功的回显信息如下图所示。你需要保存主日历 ID（`calendar_id`）的值，后续用于获取日历、日程数据。

        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/62981e75a738bf6e650e1b79d3591c91_B4f7XoapWB.png?height=1434&lazyload=true&maxWidth=600&width=2870)

4. 调用[订阅日历变更事件](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/subscription)接口，订阅当前登录用户的日历列表中的所有日历变更事件。
    
    事件订阅后当有日历的新增、删除或者更新时，会向当前应用的回调地址推送事件通知。
    
    1. 在 API 调试台左侧 API 列表中，选择 **日历** > **日历管理** > **订阅日历变更事件**。
    
    2. 确保请求头 **Authorization** 参数配置了 **user_access_token** 后，在页面右上角点击 **开始调试**。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/628730f0b712f86b64f2c0eb7d340d7a_G8VJm0Jfl3.png?height=1416&lazyload=true&maxWidth=600&width=2878)

        调用成功的回显信息如下所示。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/05e9e5fefe3a7a0b6891c20e1da20440_mvaqpkKfU7.png?height=1388&lazyload=true&maxWidth=600&width=2882)

5. 调用[订阅日程变更事件](/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/subscription)接口，订阅当前登录用户的指定日历上的日程变更事件。
    
    事件订阅后当指定日历上有日程的新增、删除或者更新时，会向当前应用的回调地址推送事件通知。
    
    1. 在 API 调试台左侧 API 列表中，选择 **日历** > **日程管理** > **订阅日程变更事件**。
    
    2. 完成参数配置，然后在页面右上角点击 **开始调试**。
        
        - 确保请求头 **Authorization** 参数配置了 **user_access_token**。
        
        - 路径参数 **calendar_id** 配置已保存的主日历 ID。
        
        调用成功的回显信息如下图所示。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/666583e5011b909c1755dc452c1669e2_bFfsM9pEHQ.png?height=1290&lazyload=true&maxWidth=600&width=2882)
