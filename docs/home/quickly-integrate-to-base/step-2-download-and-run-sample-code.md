---
document_id: '7275897728244596742'
directory_id: '7273792780344885254'
title: 步骤二：下载并运行示例代码
full_path: /home/quick-access-to-base/step-2-download-and-run-sample-code
breadcrumb:
- Home
- Quickly Integrate to Base
- 'Step 2: Download and run sample code'
document_type: GuideDocumentType
updated_at: 2023-09-07T01:59:54Z
source_url: https://open.larksuite.com/document/home/quick-access-to-base/step-2-download-and-run-sample-code
---

# 步骤二：下载并运行示例代码

在本步骤中，您将下载并运行教程提供的示例代码。示例代码需要本地环境已安装 Node.js。

## 代码结构

示例代码内结构说明如下。

```JavaScript
bitable
│  ├─ addTable.js                   --添加数据表
│  ├─ addTableRecord.js             --添加表记录
│  ├─ client.js                     --全局定义文件
│  ├─ createTable.js                --创建多维表格
│  ├─ deleteTable.js                --删除表记录
│  ├─ exportTable.js                --获取表记录
│  ├─ index.js                      --入口文件
│  ├─ mock.json                     --mock
│  ├─ package.json                  --依赖包
│  └─ spinner.js                    --终端交互
```

## 操作步骤

1. 执行以下命令，下载[示例代码](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cb7a4190c8589249f0a65e5340fd228c_8KDdZWu8ij.zip)至本地。

    ```bash
    curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cb7a4190c8589249f0a65e5340fd228c_8KDdZWu8ij.zip -o bitable_quick_start.zip
    ```

2. 解压示例代码（bitable_quick_start.zip）。

3. 在示例代码所在目录，执行以下命令，进入 `/bitable` 目录，并安装依赖包。

    ```bash
    cd bitable
    npm install
    ```

4. 执行以下命令编辑 `client.js` 文件。

- macOS / Linux 端

  ```bash
  vim /client.js
  ```

:::note
如果你不熟系命令行编辑文件的方法，可以使用本地编辑器打开示例代码中的 `/client.js` 文件进行编辑。
:::

- Windows 端使用本地编辑器打开示例代码内的 `/client.js` 文件进行编辑。

5. 将文件内 appId、appSecret 配置参数值，改为实际的应用凭证参数值。

   ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4cf0f8fde52d135a25758e3b1d7bbd29_nUc3Owa6RP.png?height=404&lazyload=true&maxWidth=500&width=918)

    应用凭证 App ID 和 App Secret 获取方式：

    1. 登录[Lark开发者后台](https://open.larksuite.com/app)。
    
    2. 进入应用详情页，在左侧导航栏，单击 **凭证与基础信息**。
    
    3. 在 **应用凭证** 区域，获取并保存 **App ID** 和 **App Secret**。
    
        ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f7f89950be7e57c2760a8b5b1f5e17c9_vgOPBuGBDz.png?height=524&lazyload=true&maxWidth=600&width=3594)

6. 修改完成后，保存并退出文件，在当前目录（ `/bitable/` ）下运行以下命令启动服务。

   	```
    npm run start
    ```

    启动服务后，您可以在命令行中查看到代码交互信息。
