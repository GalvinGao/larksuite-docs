---
document_id: '7275897728244137990'
directory_id: '7273792780344901638'
title: 步骤二：下载并运行示例代码
full_path: /home/quick-access-to-base/department-personnel-management-based-on-web-app/step-2-download-and-run-sample-code
breadcrumb:
- Home
- Manage Depts and Staff
- 'Step 2: Download and run sample code'
document_type: GuideDocumentType
updated_at: 2023-09-07T07:08:02Z
source_url: https://open.larksuite.com/document/home/quick-access-to-base/department-personnel-management-based-on-web-app/step-2-download-and-run-sample-code
---

# 步骤二：下载并运行示例代码


在本步骤中，您将下载并运行教程提供的示例代码。示例代码需要本地环境已安装 Node.js。

## 操作步骤

1. 执行以下命令，下载[示例代码](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a076d4994ae5781187ebce409ae5e73c_GVBzElGh3u.zip)至本地。
    
    ```bash
    curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a076d4994ae5781187ebce409ae5e73c_GVBzElGh3u.zip -o web_app_quick_start.zip
    ```
    
2. 解压示例代码（web_app_quick_start.zip）。
  
3. 在示例代码所在目录，执行以下命令，进入 `/web-app` 目录，并安装依赖包。

    ```bash
    cd web-app/
    npm install
    ```

4. 编辑 `config.js` 文件。

- macOS / Linux 端需要执行以下命令。
   
  ```bash
  vim /src/config.js
  ```
  
:::note
如果你不熟系命令行编辑文件的方法，可以使用本地编辑器打开示例代码中的 `/src/config.js` 文件进行编辑。
:::
      
- Windows 端使用本地编辑器打开示例代码中的 `/src/config.js` 文件进行编辑。

5. 将文件内 appId、appSecret 参数值，改为实际的应用凭证参数值。

	其中，macOS / Linux 端需要按 `i` 进入编辑模式。
  
  	 ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/814a7ca674f44d5681e0e031db843ad6_qXztCYQ8Ss.png?height=196&lazyload=true&maxWidth=500&width=944)

  
    应用凭证 App ID 和 App Secret 获取方式：
   
    1. 登录[Lark开发者后台](https://open.larksuite.com/app)。
   
    2. 进入应用详情页，在左侧导航栏，单击 **凭证与基础信息**。
   
    3. 在 **应用凭证** 区域，获取并保存 **App ID** 和 **App Secret**。
   
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f7f89950be7e57c2760a8b5b1f5e17c9_41zcgTRKvt.png?height=524&lazyload=true&maxWidth=600&width=3594)

6. 修改完成后，保存并退出文件。

   其中，macOS / Linux 端需要按 `esc` 退出编辑模式，并输入 `:wq` 回车，保存退出文件。

7. 在 `/web-app/` 目录下运行以下命令启动服务。

    ```
    npm run start
    ```
    
    服务成功启动后命令行回显信息如下图所示。

	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f816cbac3c89e713dc1c8c990cbf4f02_ymCmGGhUwu.png?height=628&lazyload=true&maxWidth=500&width=1198)
