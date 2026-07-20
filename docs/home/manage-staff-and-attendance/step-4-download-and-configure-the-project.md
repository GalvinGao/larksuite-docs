---
document_id: '7275897728244056070'
directory_id: '7273792780344950790'
title: 步骤四：下载并配置项目
full_path: /home/quick-start-of-personnel-and-attendance-management-system/step-4-download-and-configure-the-project
breadcrumb:
- Home
- Manage Staff and Attendance
- 'Step 4: Download and configure the project'
document_type: GuideDocumentType
updated_at: 2023-09-07T07:08:14Z
source_url: https://open.larksuite.com/document/home/quick-start-of-personnel-and-attendance-management-system/step-4-download-and-configure-the-project
---

# 步骤四：下载并配置项目

在本步骤，你将下载并配置教程提供的示例代码。

## 操作步骤

1. 执行以下命令，下载[示例代码](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9c5f7ad78a14ce4d9450484865d9339c_VflIPfrq19.zip)。

    ```bash
    curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9c5f7ad78a14ce4d9450484865d9339c_VflIPfrq19.zip -o translation_app.zip
    ```

2. 下载完成后，使用`unzip`命令进行解压。
    
    Windows 用户可以直接使用解压缩工具进行解压。
    
    ```bash
    unzip translation_app.zip
    ```
    
	解压后的代码结构如下：
	
    ```
    ├─ translation_app
    │  ├─ public
    │  │  └─ index.html                    -- 入口html
    │  ├─ src
    │  │  ├─ App.tsx                       
    │  │  ├─ api.ts                        -- 接口定义
    │  │  ├─ axios.ts                      -- axios封装
    │  │  ├─ client
    │  │  │  └─ use-free-login.js         -- 免登代码
    │  │  ├─ compents                      -- 组件
    │  │  │  ├─ approve
    │  │  │  │  └─ index.tsx              -- 审批
    │  │  │  ├─ department
    │  │  │  │  └─ index.tsx              -- 部门管理
    │  │  │  ├─ mainContent
    │  │  │  │  └─ index.tsx
    │  │  │  └─ personnel
    │  │  │     └─ index.tsx               -- 人员
    │  │  ├─ config.js                      -- 定义常量（appId、appSecret）
    │  │  ├─ index.css
    │  │  ├─ index.tsx
    │  │  ├─ logo.svg
    │  │  ├─ server                         -- 服务层
    │  │  │  ├─ add-leave-records.js       -- 添加请假记录
    │  │  │  ├─ add-table.js               -- 创建多维表格
    │  │  │  ├─ add-work-records.js        -- 添加加班记录
    │  │  │  ├─ approval.js                -- 自定义审批事件
    │  │  │  ├─ client.js                  -- 引入node-sdk
    │  │  │  ├─ create-deparment.js        -- 创建部门
    │  │  │  ├─ create-leave-work.js       -- 请假
    │  │  │  ├─ create-sub-department.js   -- 创建子部门
    │  │  │  ├─ create-work.js             -- 加班
    │  │  │  ├─ get-all-department.js      -- 获取所有部门
    │  │  │  ├─ getrecords.js              -- 列出多维表格记录
    │  │  │  ├─ index.js                   -- 入口文件
    │  │  │  ├─ config.js                  -- 导出approve_code
    │  │  │  ├─ listen-message-card-event.js  -- 消息卡片操作
    │  │  └─ type.ts
    │  └─ tsconfig.json
    ```
   
3. 配置项目中的变量值。

    1. 将 appId、appSecret 填入 `/translation_app/src/config.js  `和 `/translation_app/src/server/client.js` 中。
    
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f285feb1e21d53fcf866bcf6cf9b36cd_9m6Pj6oSfq.png?height=1204&lazyload=true&maxWidth=600&width=1694)
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a07bc04025db80cecfaf03b15a9e7c50_CUFUZdR9xP.png?height=998&lazyload=true&maxWidth=600&width=1728)
        
    2. 应用的 **appId** 和 **appSecret** 可以在 [开发者后台](https://open.larksuite.com/app) 的 **凭证与基础信息** 页查看。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a662043459028b27a15c4a879dc76bc8_Vk2paLY0cB.png?height=516&lazyload=true&maxWidth=600&width=2326)
        
    3. 将 **请假通过** 及 **加班通过** 的卡片 ID 依次配置到 `/translation_app/src/server/message-card/listen-message-card-event.js`。
    
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f084465f4a6a84c3488e587d0bd9cebe_4n5fd2JPkA.png?height=1250&lazyload=true&maxWidth=600&width=2826)
        
    4. 将**加班审批卡片 ID** 配置到 `/translation_app/src/server/approve/create-work.js`。
    
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2610c666094919f5e0d01ed93fe07b97_dPD3X3ZVyu.png?height=1384&lazyload=true&maxWidth=600&width=2798)
        
    5. 将 **请假审批卡片 ID** 配置到 `/translation_app/src/server/approval/approval.js`。
    
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e05a23df9410890230aff14fa47f525b_OtiXtMgSjF.png?height=1214&lazyload=true&maxWidth=600&width=2118)
        
        消息卡片 ID 可以在 [消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder) 中 **我的卡片** 栏中直接点击 **复制卡片 ID**，也可以在卡片预览区域点击复制卡片 ID。
        
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e2a19268f718e362c67e23a2951590fe_yc0GAQ4XrP.png?height=1148&lazyload=true&maxWidth=600&width=1668)
        
    6. 将 CUSTOM_APPROVE_CODE 配置到 `/translation_app/src/server/config.js`
    
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8cc51a8d012a6a4b6c28185e8a6fd6b9_hehxcjQOsg.png?height=1004&lazyload=true&maxWidth=600&width=1988)
        
    	CUSTOM_APPROVE_CODE 可以在 [审批管理后台](https://www.larksuite.com/approval/admin) 的审批实例编辑页查看。
    
    	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e55feccbd6b1d6f7dbbbc5645fe33381_pK2sL0hmsl.png?height=742&lazyload=true&maxWidth=600&width=2334)
