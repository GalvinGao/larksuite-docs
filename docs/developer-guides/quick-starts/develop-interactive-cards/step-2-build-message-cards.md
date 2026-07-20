---
document_id: '7275897728244629510'
directory_id: '7273792780344868870'
title: 步骤二：搭建消息卡片
full_path: /home/quickly-develop-interactive-cards/step-2-build-the-message-card
breadcrumb:
- Developer Guides
- Quick Starts
- Develop Interactive Cards
- 'Step 2: Build message cards'
document_type: GuideDocumentType
updated_at: 2023-09-07T02:00:02Z
source_url: https://open.larksuite.com/document/home/quickly-develop-interactive-cards/step-2-build-the-message-card
---

# 步骤二：搭建消息卡片

本教程中需要搭建三张消息卡片：带有加班和请假互动操作的审批卡片，加班通过的卡片、请假通过的卡片。

## 操作步骤

1. 登录 [消息卡片搭建工具](https://open.larksuite.com/tool/cardbuilder)，单击右上方的 **新建卡片** 按钮。

  	首次登录时，消息卡片搭建工具会自动为你生成一个初始的消息卡片，你可以在该卡片的基础上进行编辑，也可以直接创建一个新的卡片。

3. 在新建卡片页面，单击 **新建空白卡片** ，然后在弹出的对话框中填写 **卡片名称** ，最后单击 **保存** 。

    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/adf1c31679d2db54f5b057c65e4086d0_sbsRNQMvwW.png?height=1732&lazyload=true&maxWidth=500&width=3032)
    
4. 新建卡片后，你可以在 **模块组件** 栏，单击需要的组件，将其添加到卡片中。

	例如：在 **内容组件** 中，单击 **标题** 组件即可为卡片添加一个标题。
    
	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/34fb8aaa987c54c349a7a83ff1f7fb43_01JbVBHV6U.png?height=1430&lazyload=true&maxWidth=500&width=1844)

5. 在中间的 **卡片预览** 区域，选择需要的模块组件，并完成各模块的编辑。
   
   ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9e1df91200ceb7e287f0a45f17be68c8_2wwMxQwcL3.gif?height=972&lazyload=true&maxWidth=700&width=2132)
    
    1. 单击 **标题** ，在右侧配置标题的 **内容** 和 **样式** 。
    
    2. 单击 **按钮**，选择 **双按钮** 排列方式，在右侧编辑按钮的 **内容、样式** 及 **回传参数** 。
    
    3. 审批卡片中，为了区分用户点击了哪个按钮，我们需要为 **按钮配置回传交互参数**。用户点击后，触发的事件会由Lark回传给在开发者后台填入的消息卡片请求地址。
    
    4. 两个按钮配置的回传交互参数如下：
    	
        - 加班：type，work_overtime。
        	
            ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/11c9cd505ec207386bf706a2b3b27b3d_lmI59I9skE.png?height=1064&lazyload=true&maxWidth=500&width=1680)
        
        - 请假：type，leave。
        

    		![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ec2a7ebf29c909254ce41a70a15a3793_OHSADzUGVk.png?height=1054&lazyload=true&maxWidth=500&width=1698)
    
    5. 修改完成后在顶部菜单栏单击 **保存并发布**。
    
5. 参考以上流程继续创建并发布 **加班通过** 及 **请假通过** 的消息卡片。创建完成后，你可以在 **我的卡片** 中看到已经成功发布的消息卡片。

	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6c78384186c3e7930100b59942ad5b70_vw113M8b0b.png?height=418&lazyload=true&maxWidth=600&width=1638)
