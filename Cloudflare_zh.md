# 在Cloudflare上构建后端（完全免费）


## 一、创建git仓库

1. 在github上创建一个仓库。
2. 在其中创建一个名为`functions`的文件夹。
3. 在`functions`文件夹中创建一个名为`metrics.js`的文件。 
4. 将以下代码复制到`metrics.js`中（当然你也可以自定义）。
```js
export async function onRequestPost(context) {
    try {
        const request = context.request;
        const body = await request.json();
        if (!body || typeof body !== 'object') {
            return await context.next();
        }
        if (!body.crate || !body.ver) {
            return new Response(JSON.stringify({
                ok: false,
                internal: false,
                err: "No crate or version received.",
            }));
        }
        const crate = body.crate.toString();
        const version = body.ver.toString();
        const ident = crate + "@" + version;
        if ((await context.env.AVAILABLE.get(ident)) === null) {
            return new Response(JSON.stringify({
                ok: false,
                internal: false,
                err: "No such crate and version: " + ident,
            }));
        }
        const time = new Date().toUTCString();
        const timezone = request.cf.timezone.toString();
        const continent = request.cf.continent || "";
        const country = request.cf.country || "";
        const city = request.cf.city || "";
        const latitude = request.cf.latitude || -1;
        const longitude = request.cf.longitude || -1;
        const res = await context.env.METRICS.prepare(
            'INSERT INTO crateio (crate, version, time, timezone, continent, country, city, latitude, longitude) ' +
            'VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);')
            .bind(crate, version, time, timezone, continent, country, city, latitude, longitude)
            .run();
        return new Response(JSON.stringify({
            ok: true,
            res: res.success,
        }));
    } catch (e) {
        return new Response(JSON.stringify({
            ok: false,
            internal: true,
            err: e.toString(),
        }));
    }
}
```
5. todo: 可克隆的github仓库


## 二、登录Cloudflare账号

1. 访问[Cloudflare仪表盘](https://dash.cloudflare.com/)。
2. 此步骤无需详细展示，使用邮箱正常注册登录即可。


## 三、添加站点

1. 登录后，点击左侧列的`Workers和Pages`。
2. 点击`创建应用程序`，选择`Pages`。
3. 选择`连接到git`，`连接github`，同意导入。
4. 选择第一步创建的仓库，点击`开始设置`。
5. 确认信息后点击`保存并部署`。


## 四、创建KV

1. 登录后，点击左侧列`Workers和Pages`下的`KV`。
2. 点击`创建命名空间`，输入任意名称，点击`添加`。
3. 在你刚创建的KV旁点击`查看`，此处的密钥填你创建的crate，无需填值。
4. 密钥格式：`<crate名称>@<crate版本>`，例如：`reportme@0.1.0`。
5. 以后如有新的crate，可在此处创建新的密钥。


## 五、创建D1

1. 登录后，点击左侧列`Workers和Pages`下的`D1`。
2. 点击`创建数据库`，`仪表盘`，输入任意名称，点击`创建`。
3. 点击`创建表`，名称为`crateio`，或者需要在js文件中修改（仅sql命令一处）。
4. 添加以下列，并点击创建。

| 列名       | 数据类型 | 主密钥 |
|-----------|---------|------|
| id        | integer | 设置  |
| crate     | text    |      |
| version   | text    |      |
| time      | text    |      |
| timezone  | text    |      |
| continent | text    |      |
| country   | text    |      |
| city      | text    |      |
| latitude  | real    |      |
| longitude | real    |      |


## 六、设置环境变量

1. 登录后，点击左侧列的`Workers和Pages`。
2. 点击你在第二步创建的Page，点击`设置`，点击`函数`。
3. 下拉找到`KV 命名空间绑定`，添加，名称为`AVAILABLE`，值为第四步创建的kv。
4. 下拉找到`D1 数据库绑定`，添加，名称为`METRICS`，值为第五步创建的d1，记得保存。
5. 通过github重新推送代码，从工作流推送到cloudflare，确保环境变量可用。

## 七、在你的其他crate中使用`reportme`！

`build.rs`:
```rust
use std::time::Duration;
use reportme::report_build;

fn main() {
    report_build("https://crateio.<你的PAGE名称>.pages.dev/metrics",
                 Duration::from_secs(10),
                 env!("CARGO_PKG_NAME"),
                 env!("CARGO_PKG_VERSION"));
}
```
