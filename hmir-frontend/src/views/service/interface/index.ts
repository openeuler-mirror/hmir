interface serviceProp {
  prop?: string;
  label?: string;
}

interface serviceTableList {
  description?: any;
  id?: any;
  state?: any;
}

export default interface serviceList {
  value: string;
  tableList?: Array<serviceTableList>;
  tableProp?: Array<serviceProp>
}

