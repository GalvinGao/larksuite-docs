---
document_id: '7273741028051058694'
directory_id: '7002892512470810630'
title: 步骤二：下载并配置示例代码
full_path: /home/integrating-web-apps-in-5-minutes/step-3-start-the-local-service
breadcrumb:
- Home
- Quickly Develop H5 (Python)
- 'Step 2: Download and run the sample code'
document_type: GuideDocumentType
updated_at: 2023-11-07T12:33:06Z
source_url: https://open.larksuite.com/document/home/integrating-web-apps-in-5-minutes/step-3-start-the-local-service
---

# 步骤二：下载并配置示例代码

你需要先获取应用凭证，然后下载示例代码，并将应用凭证配置在代码的相应位置，最后为示例代码配置运行环境。

## 获取应用凭证

1. 在[开发者后台](https://open.larksuite.com/app/)，点击应用名称或应用图标进入应用详情页。

    ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1dd0ed243f124d8001901522bd196626_6KICVofqyj.png?height=1460&lazyload=true&maxWidth=600&width=2462)

2. 在左侧导航栏点击进入 **凭证与基础信息** 页面，在 **应用凭证** 中获取 `App ID` 和 `App Secret` 值。

    ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1324a903a8c08a27836316a6bc2011f7_hHBttFt5ic.png?height=1200&lazyload=true&maxWidth=600&width=2670)

## 下载代码示例并配置应用凭证

1. 使用本地的命令行工具，下载并解压 [示例代码](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8651c8d456469c0881343f9b7d006833.zip)，并进入 `python` 目录。


	各模块代码逻辑的介绍，参考[示例代码介绍](/document/home/integrating-web-apps-in-5-minutes/debug-and-release)。


    **Mac/** **Linux**
    ```
    curl  https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8651c8d456469c0881343f9b7d006833.zip -o web_app_with_jssdk.zip
    unzip web_app_with_jssdk.zip
    cd web_app_with_jssdk/python
    ```
    **Windows**
    ```
    curl  https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8651c8d456469c0881343f9b7d006833.zip -o web_app_with_jssdk.zip
    tar -xzvf web_app_with_jssdk.zip
    cd web_app_with_jssdk/python
    ```

2. 修改 `.env` 文件中应用凭证数据为实际应用凭证信息。

    ```
    APP_ID=cli_9fxxxx00b
    APP_SECRET=EX6xxxxOF
    ```

## 启动本地服务


1. 创建并激活一个新的虚拟环境。

    **Mac/** **Linux**
    ```
    python3 -m venv venv
    . venv/bin/activate
    ```
    **Windows**
    ```
    python3 -m venv venv
    venv\Scripts\activate
    ```
    
    激活后，终端会显示虚拟环境的名称。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9a40883c5c6ada83f03f534456e21a19_X4kHriZmBl.png?height=178&lazyload=true&maxWidth=600&width=820)
    
2. 安装依赖。
    ```
    pip install -r requirements.txt
    ```
3. 启动项目并获取内网访问地址。

    ```
    python3 server.py
    ```
    
    启动后会生成临时域名，如下图所示，仅在同一局域网内有效。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/de113c3631feeea9e6a62277138c2196_RiszAgYTKZ.png?height=590&lazyload=true&maxWidth=600&width=1442)

