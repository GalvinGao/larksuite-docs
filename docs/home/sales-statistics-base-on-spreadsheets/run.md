---
document_id: '7233612551991656454'
directory_id: '7199928167142244357'
title: 运行
full_path: /home/sales-statistics-base-on-spreadsheets/run
breadcrumb:
- Home
- Sales statistics base on spreadsheets
- Run
document_type: GuideDocumentType
updated_at: 2023-05-16T03:12:31Z
source_url: https://open.larksuite.com/document/home/sales-statistics-base-on-spreadsheets/run
---

# 运行
:::note
本教程提供了Docker和本地运行两种方式，开发者可以根据自己常用的情况选择。
:::
## 方式一：Docker运行

运行之前需要确保[Docker](https://www.docker.com/)已经安装。Docker运行与下方本地运行二选一即可。

**mac/linux**

```
sh exec.sh
```

**windows**

```
.\exec.ps1
```

## 方式二：本地运行

1、创建并激活一个新的虚拟环境。

**mac/linux**

```
python3 -m venv venv
. venv/bin/activate
```

**windows**

```
python3 -m venv venv
venv\Scripts\activate
```

激活后，终端会显示虚拟环境的名称。

```
(venv) **** python %
```

2、安装依赖。

```
pip install -r requirements.txt
```

3、运行。

```
python3 sales.py
```
