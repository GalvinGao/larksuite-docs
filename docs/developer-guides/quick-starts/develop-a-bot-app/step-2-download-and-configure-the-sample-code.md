---
document_id: '7280787361268695046'
directory_id: '7002892512470794246'
title: 步骤二：下载并配置示例代码
full_path: /home/develop-a-bot-in-5-minutes/step-3-configure-application-credentials
breadcrumb:
- Developer Guides
- Quick Starts
- Develop a Bot App
- 'Step 2: Download and configure the sample code'
document_type: GuideDocumentType
updated_at: 2024-02-08T03:49:36Z
source_url: https://open.larksuite.com/document/home/develop-a-bot-in-5-minutes/step-3-configure-application-credentials
---

# 步骤二：下载并配置示例代码

你需要先获取应用凭证，然后下载示例代码，并将应用凭证配置在代码的相应位置，最后为示例代码配置运行环境。

## 获取应用凭证
:::note
**测试版应用和正式版应用是互相独立的两个应用**，如果要使用测试版应用进行开发体验，后续所有操作需要在测试版应用进行。
:::

:::html
<md-td>
1. 在左侧导航栏点击进入 **凭证与基础信息** 页面，在 **应用凭证** 中获取 `App ID` 和 `App Secret` 值。

    <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/337ec1d264e66f1b514657e94f96d1b2_NpX8ykOt8J.png?lazyload=true&width=2556&height=1050" style="width:70%" />
    
2. 在左侧导航栏点击进入 **事件订阅** 页面，获取 `Verification Token` 值。

	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9375f4ab583a83096d6ccd1be9c8d055_fYNK2jC2rf.png?height=950&lazyload=true&maxWidth=600&width=2324)

</md-td>
:::   

## 下载代码示例并配置应用凭证

1. 使用本地的命令行工具，下载并解压 [示例代码](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5079d07daa862fb8dfdd8960429b830e.zip)，并进入 `python` 目录。


    各模块代码逻辑的介绍，参考 [示例代码介绍](/document/home/develop-a-bot-in-5-minutes/configuration)。


    **Mac/** **Linux**
    ```
    curl  https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5079d07daa862fb8dfdd8960429b830e.zip -o robot_quick_start.zip
    unzip robot_quick_start.zip
    cd robot_quick_start/python
    ```
    **Windows**
    ```
    curl  https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5079d07daa862fb8dfdd8960429b830e.zip -o robot_quick_start.zip
    robot_quick_start.zip
    cd robot_quick_start/python
    ```
2. 修改 `.env` 文件中应用凭证数据为实际应用凭证信息。
    ```
    APP_ID=cli_9fxxxx00b
    APP_SECRET=EX6xxxxOF
    VERIFICATION_TOKEN=cq3xxxxxxkUS
    ```
    
## 启动本地服务

您可以选择本地或 Docker 容器任意一种运行方式启动后端服务。

### 方式一：本地运行
:::html
<md-td>
1. 创建并激活一个新的虚拟环境。
    
    **Windows**
    ```
    python3 -m venv venv
    venv\Scripts\activate
    ```
    **Mac/** **Linux**
    ```
    python3 -m venv venv
    . venv/bin/activate
    ```
  
   激活后，终端会显示虚拟环境的名称。

   <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9a40883c5c6ada83f03f534456e21a19_AHOHKpEgsl.png?lazyload=true&width=820&height=178" style="width:70%" />

2. 安装依赖。
    ```
    pip3 install -r requirements.txt
    ```
    安装成功后，将会有如下提示：

    <img src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3293927c121b05b54a20947fb4403d09_Y8qoUoAh8P.png?lazyload=true&width=2950&height=194" style="width:70%" />

3. 运行服务。
    ```
    python3 server.py
    ```
</md-td>
:::
### 方式二：Docker 运行

若您已安装 [Docker](https://www.docker.com/)，可直接运行 Docker 环境。

**Mac/** **Linux**
```
sh exec.sh
```
**Windows**
```
.\exec.ps1
```
