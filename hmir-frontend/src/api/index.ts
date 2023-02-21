/*
 * @Author: duanwujie88 dev17001@linx-info.com
 * @Date: 2023-02-02 17:28:02
 * @LastEditors: duanwujie88 dev17001@linx-info.com
 * @LastEditTime: 2023-02-02 17:30:18
 * @FilePath: /hmir-frontend/src/api/index.ts
 * @Description: API模块
 */
import * as ceph from "./ceph";
import * as ttyd from "./ttyd";
import * as quit from "./quit";
import * as login from "./login";
import * as service from "./service";
import * as process from "./process";
import * as  system from "./system"

const api = {
  ...ceph,
  ...ttyd,
  ...quit,
  ...login,
  ...service,
  ...process,
  ...system
}

export default api