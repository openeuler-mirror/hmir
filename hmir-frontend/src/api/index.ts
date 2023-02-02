import * as ceph from "./ceph";
import * as ttyd from "./ttyd";
import * as quit from "./quit";
import * as login from "./login";
import * as service from "./service";

const api = {
  ...ceph,
  ...ttyd,
  ...quit,
  ...login,
  ...service
}

export default api