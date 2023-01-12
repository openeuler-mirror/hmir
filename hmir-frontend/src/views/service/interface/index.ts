interface serviceProp {
  prop?: string;
  label?: string;
}

interface serviceTableList {
  description?: any;
  id?: any;
  state?: any;
  nextRun?: any;
  nearestTrigger?: any;
}

export default interface serviceList {
  value: string;
  tableList?: Array<serviceTableList>;
  tableProp?: Array<serviceProp>
}

