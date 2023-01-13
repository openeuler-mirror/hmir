import type { Column } from 'element-plus'

interface serviceTableList {
  description?: any;
  name?: any;
  state?: any;
  nextRun?: any;
  nearestTrigger?: any;
  load_state?: any;
  active_state?: any;
  sub_state?: any;
  follow_unit?: any;
  object_path?: any;
  job_id?: any;
  job_ty?: any;
  job_object_path?: any;
}

export default interface serviceList {
  value: string;
  tableList?: Array<serviceTableList>;
  tableProp?: Column<any>[]
}

