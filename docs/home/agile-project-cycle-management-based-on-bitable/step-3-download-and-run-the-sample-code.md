---
document_id: '7233334510162034693'
directory_id: '7199928167142211589'
title: 步骤三：下载并运行示例代码
full_path: /home/agile-project-cycle-management-based-on-bitable/experience-the-app
breadcrumb:
- Home
- Agile project cycle management based on Bitable
- 'Step 3: Download and run the sample code'
document_type: GuideDocumentType
updated_at: 2023-05-16T06:13:07Z
source_url: https://open.larksuite.com/document/home/agile-project-cycle-management-based-on-bitable/experience-the-app
---

# 步骤三：下载并运行示例代码

在本步骤，你将下载并运行教程提供的示例代码。示例代码使用 Python 语言编写，请确保你已安装 Python 运行环境。
## 步骤一：下载示例代码
1. 执行以下命令，下载示例代码到本地。
   * Mac OS 或 Linux执行以下命令。
      ```Shell
      curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/05f25bc022cec3e0b5f23ef3780a7e82_dCu2ri2s5B.zip -o bitable_calendar.zip
      unzip bitable_calendar.zip
      cd bitable_calendar/python
      ```

   * Windows执行以下命令。
      ```Shell
      curl https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/05f25bc022cec3e0b5f23ef3780a7e82_dCu2ri2s5B.zip -o bitable_calendar.zip
      bitable_calendar.zip
      cd bitable_calendar/python
      ```

2. 执行以下命令，修改`.env`文件中的应用凭证为测试应用的凭证数据。
    ```PowerShell
    vim .env
    ```
    :::note
      Windows用户可直接使用记事本进行修改。
    :::
   1. 按 `i` 进入编辑模式，然后修改 `.env` 文件中的 **APP_ID** 和 **APP_SECRET** 属性为你的应用的凭证数据。
    应用凭证信息可以在[开发者后台](https://open.larksuite.com/app)的 **凭证与基础信息** 页查看。
    ![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/be069cb96e07a3512076c0e6aba814db.png?lazyload=true&width=2690&height=1024)
   2. 修改 **APP_TOKEN** 为多维表格表格的 **token** 信息。
    多维表格的token信息可以在浏览器地址栏获取。
    ![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/2e96043c6a9fb201b022e625ece1a3f2.png?lazyload=true&width=2420&height=744)
   3. 修改完成后，按 `:wq` 保存退出。
      :::note
      私有化部署时要修改`.env`文件中 **Lark_HOST** 为私有化部署所用的域名。
      :::
3. 打开 `mock.py` 文件，然后版本信息更新到 `raw_data`属性 中。
 
 	如果您需要字段名和字段类型，需要同步修改 `schema` 和 `name_map` 的值。

 	 `mock.py`文件中的内容如下：
    ```Shell
    from config import KEY_SUBMIT, KEY_GREY, KEY_GA, KEY_STARTUP, KEY_VERSION
    schema = {
        "version": 1,
        "proposal": 5,
        "delivery": 5,
        "expected": 5,
        "online": 5,
    }

    name_map = {
        "version": KEY_VERSION,
        "proposal": KEY_STARTUP,
        "delivery": KEY_SUBMIT,
        "expected": KEY_GREY,
        "online": KEY_GA,
    }

    raw_data = [
        {
            "version": "1.0.0",
            "proposal": '2022-07-20',
            "delivery": '2022-7-27',
            "expected": '2022-07-28',
            "online": '2022-07-29',
        },
        {
            "version": "1.1.0",
            "proposal": '2022-07-30',
            "delivery": '2022-08-04',
            "expected": '2022-08-05',
            "online": '2022-08-07',
        },
        {
            "version": "1.2.0",
            "proposal": '2022-08-09',
            "delivery": '2022-08-16',
            "expected": '2022-08-16',
            "online": '2022-08-16',
        }
    ]
    ```

## 步骤二：运行示例代码
你可以参考以下两种方式运行示例代码。
### Docker运行
如果您已有[Docker](https://www.docker.com/)环境，那么直接执行以下命令即可。
* Mac OS或Linux执行以下命令：
  ```Shell
  sh exec.sh
  ```

* Windows执行以下命令：
  ```Shell
  .\exec.ps1
  ```

### 本地运行
1. 执行以下命令，创建并激活Python虚拟环境。
   * Mac OS 或 Linux执行以下命令。
      ```Shell
      python3 -m venv venv
      . venv/bin/activate
      ```

   * Windows执行以下命令。
      ```Shell
      python3 -m venv venv
      venv\Scripts\activate
      ```

2. 执行以下命令，安装代码依赖。
    ```Shell
    pip3 install -r requirements.txt
    ```

3. 依赖安装成功后，执行以下命令运行示例代码。
    ```Shell
    python3 bitable_calendar.py
    ```

    示例代码运行时，会依次调用 [教程简介](/document/home/agile-project-cycle-management-based-on-bitable/introduction) 中提到的所有接口，你可以在控制台查看调用过程。
    
    成功的运行结果如下图所示：

    ![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/5b017fa9a534f5632926f35da4c0b970.png?lazyload=true&width=2472&height=854)

