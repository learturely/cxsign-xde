# cxsign
X星签到的命令行工具，可以为每个课程添加单独的地点。支持多个账号签到。支持普通签到、拍照签到、二维码签到、位置签到、二维码位置签到、手势签到、签到码签到。

## 注意事项

**本项目使用的网络 API 均来源于网络公开资料。**

**本项目不提供任何现实中的网络服务，仅提供相关功能实现的参考，请勿用于任何实际用途。**

本项目以 `AGPL-3.0` 协议开源。同时包含如下附加协议：
+ **本项目仅供学习和研究使用，严禁商用。**
+ 不得广泛传播（严禁包括但不限于网盘分享、群组分享等一次性或持续性多人分享行为）。
+ 严禁任何不以学习研究为目的个人分享行为。

## 特别注意

**请注意账号安全**：

虽然输入密码时不会在命令行界面显示任何字符，但是密码依然被缓存本地数据库中且存在被解密的风险。

请注意保护登录者的账号密码。

  > - 如果让他人登录你的账号，请仔细甄别其是否值得信任;
  > - 如果你登录了其他人的账号，请牢记：能力越大，责任越大，同时不要辜负他人对你的信任;
  > - 你的账号不论供自己登录还是供他人登录、不论是否使用本软件或其他第三方软件登录，都推荐为其设置一个单独的、别于其他账号甚至完全无关的、不常用的密码，防止该密码被盗取后用于登录其他平台账号或猜出你设置密码的风格。

 (TODO: 考虑是否有必要为数据库添加主密码。)

## 关于命令
本节旨在叙述命令的含义，关于一些使用上问题，请看[关于使用](#关于使用)。
### 签到相关
- `cxsign --gui` 或 `cxsign -g`, 以 GUI 模式启动。
  > **注意**：还未完工，请不要使用。
- `cxsign`, 处理所有有效签到活动。或：`cxsign 2000024077709`, 处理 `active_id` 为 `2000024077709` 的签到活动。
  > 所谓“有效”是指两小时内且未结束（理应是“未签到的”，但是因为……，所以我没有这么实现）的签到活动。

  > 选项及参数说明：
  > - `-a, --account <ACCOUNT>`, 签到账号。默认以一定顺序对所有用户进行签到。
  > - `-l, --location <LOCATION>`, 位置 ID. 位置签到或二维码位置签到时需要提供。也可以通过 `--pos` 选项直接指定位置，此时本选项将失效。默认按照先课程位置后全局位置的顺序依次尝试。
  > - `--pos <POS>`, 通过地址名称、经纬度与海拔直接指定位置。位置签到或二维码位置签到时需要提供。格式为：`地名/维度/经度/海拔`. 下文中意义与此类似的参数同样采用该格式。（TODO: 支持更符合直觉的格式和经纬度地名反查。）
  > - `-e, --enc <ENC>`, `enc` 参数。二维码签到时需要提供该参数，一般是格式为 `[A-F0-9]{32}`(类似于 MD5). 可从二维码图片中得到。
  > - `-p, --pic <PIC>`, 本地图片路径。拍照签到或二维码时需要提供，可以是文件或目录。如果是目录，则会选择在该目录下修改日期最新的图片作为拍照签到图片或二维码图片。
  > - `-s, --signcode <SIGNCODE>`, 签到码。签到码签到时需要提供。

### 账号相关
- `cxsign account`, 列出所有账号。这些信息是缓存在数据库中的，可添加 `--fresh` 或 `-f` 选项以重新获取信息，以应对登录过期的情况。
- `cxsign account add 12345678910`, 添加账号。会提示输入密码。
  > 这里的账号是手机号，并非学号，也不支持二维码登录。
  
  > 重复添加同一个账号则会覆盖原先的账号。

- `cxsign account remove 1234567890`, 询问并删除账号。可添加 `--yes` 或 `-y` 选项以避免询问，直接删除。

### 课程相关
- `cxsign course`, 列出所有课程。这些信息是缓存在数据库中的，可添加 `--fresh` 或 `-f` 选项以重新获取信息。
  > 课程信息是在登录时获取的，如果在课程信息未刷新期间有变动，会导致访问出错（课程被删除）或者无法获取某个课程（新加入的课程）的签到等情况。一般而言课程在学期中的变动不大。

### 签到活动相关
- `cxsign list`, 列出所有**有效**签到活动。可添加 `--all` 或 `-a` 选项以获取**所有**签到活动。可添加 `--course` 或 `-c` 选项指定 `course_id: i64` 以获取**某个课程**的签到活动。两个选项**可以**同时起效。

### 位置相关
- `cxsign pos`, 列出**所有**位置信息。这些信息是缓存在数据库中的，可添加 `--course` 或 `-c` 选项指定 `course_id: i64` 以列出**某个课程**的位置。可添加 `--golbal` 或 `-g` 选项以获取**全局**位置。两者**不会**同时起效，其中后者优先级较高。
  > 位置签到或二维码定位签到时，如果未指定签到位置，此时会先从课程位置开始，依次尝试课程位置和全局位置 (TODO: 测试能否同时尝试而不被 ban) 。
- `cxsign pos add "帕米尔高原/35.881944/76.514167/8611"`, 添加位置。以 `/` 分割的四个部分依次为地名、纬度、经度、海拔。可添加 `--course` 或 `-c` 选项指定 `course_id: i64` 以将该位置绑定到课程。
  > 不指定课程或指定 `-1` 均代表为全局位置。

  > 添加的位置会被分配一个 `posid`, 删除时或签到时使用。
- `cxsign pos remove 0`  询问并删除 `posid` 为 `0` 的位置。可添加 `--yes` 或 `-y` 选项以避免询问，直接删除。

### 其他
```
> cxsign where-is-config # 显示配置文件夹位置。
```
```
"/home/user/.config/cxsign"
2023-11-14T14:43:04.416530798+08:00
```
> 结尾的时间代表程序运行完毕的时间，此时程序即将正常退出。

## 关于使用
本程序没有提供扫码界面，所以在二维码签到时，需要提前指定图片或指定图片所在的目录，或直接指定 `enc` 参数。

本程序的部分功能需要进行登录操作。不过这个操作似乎耗时略长。如果提前指定图片或 `enc` 参数，且账号数量较多，外加你获取二维码图片的时间（网络延迟和手速问题），可能会造成二维码过期的情况。

针对这种情况，暂时的解决方案是：不要直接指定图片文件，而是指定文件目录。此时本程序会先登录，然后询问二维码图片是否准备好，同时利用截图程序的快捷键或快捷命令截取二维码，然后回车（即默认选项“是”，代表图片准备好了），进行签到。

这一系列操作中，使用到的具体的可供参考的命令有：
```
cxsign -p ~/Pictures -l2 # 使用 2 号位置，指定了文件夹，进行签到。
# 回车，此时程序询问，暂时放置。
# 打开新的终端，输入以下命令，暂不回车。
date -Ins && spectacle -u -b -o ~/Pictures/1.png -n # 打印当前时间，并截取鼠标所在的窗口。不截取整个屏幕为了避免二维码解码变慢（不过问题不大）。
# 鼠标移至二维码所在窗口，回车等待截取成功（时间小于 0.5 秒）。
# 快速确认图片已准备好。
# 签到成功，输出当前时间。
```
理论上，单用户时，此方案输出的两次时间之差约为 0.5 秒。

## 关于帮助信息
对于程序本身和子命令，均有 `--help` 或 `-h` 选项可供使用以打印相关帮助信息。如：
```
> cxsign --help
```
```
进行签到。

Usage: cxsign [OPTIONS] [ACTIVITY] [COMMAND]

Commands:
  account          账号相关操作（列出、添加、删除）。 默认列出所有账号。
  course           列出所有课程。
  list             列出有效签到。
  pos              位置相关操作（列出、添加、删除）。 默认列出所有位置。
  where-is-config  显示配置文件夹位置。
  help             Print this message or the help of the given subcommand(s)

Arguments:
  [ACTIVITY]  签到 ID. 默认以最近起对所有有效签到顺序进行签到，且缺少参数时会跳过并继续。

Options:
  -a, --account <ACCOUNT>    签到账号。 默认以一定顺序对所有用户进行签到。
  -l, --location <LOCATION>  位置 ID. 位置签到或二维码位置签到时需要提供。 也可以通过 `--pos` 选项直接指定位置，此时本选项将失效。 默认按照先课程位置后全局位置的顺序依次尝试。
      --pos <POS>            通过地址名称、经纬度与海拔直接指定位置。 位置签到或二维码位置签到时需要提供。 格式为：`addr/lat/lon/alt`
  -e, --enc <ENC>            `enc` 参数。 二维码签到时需要提供该参数。
  -p, --pic <PIC>            本地图片路径。 拍照签到或二维码时需要提供， 如果是文件，则直接使用该文件作为拍照签到图片或二维码图片文件。 如果是目录，则会选择在该目录下修改日期最新的图片作为拍照签到图片或二维码图片。
  -s, --signcode <SIGNCODE>  签到码。 签到码签到时需要提供。
  -g, --gui                  以图形界面模式启动。
  -h, --help                 Print help
  -V, --version              Print version
```